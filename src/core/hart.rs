use std::marker::PhantomData;

use super::bus::BusInterface;
use super::pipeline::Pipeline;
use super::register_file::RegisterFile;

pub struct Hart<M, P: Pipeline<M>>
where
    M: BusInterface<u32, i8>,
    M: BusInterface<u32, u8>,
    M: BusInterface<u32, i16>,
    M: BusInterface<u32, u16>,
    M: BusInterface<u32, u32>,
{
    program_counter: u32,
    register_file: RegisterFile,
    pipeline: P,
    phantom: PhantomData<M>,
}

impl<M, P: Pipeline<M>> Hart<M, P>
where
    M: BusInterface<u32, i8>,
    M: BusInterface<u32, u8>,
    M: BusInterface<u32, i16>,
    M: BusInterface<u32, u16>,
    M: BusInterface<u32, u32>,
{
    pub fn new() -> Self {
        Hart { program_counter: 0, register_file: RegisterFile::new(32), pipeline: P::new(), phantom: PhantomData }
    }

    pub fn execute(&mut self, memory: &mut M) {
        self.program_counter = self.pipeline.execute(self.program_counter, &mut self.register_file, memory);
    }
}
