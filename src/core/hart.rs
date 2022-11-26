use std::marker::PhantomData;

use super::memory_interface::MemoryInterface;
use super::pipeline::Pipeline;
use super::register_file::RegisterFile;

pub struct Hart<M: MemoryInterface<u32>, P: Pipeline<M>> {
    program_counter: u32,
    register_file: RegisterFile,
    pipeline: P,
    phantom: PhantomData<M>,
}

impl<M: MemoryInterface<u32>, P: Pipeline<M>> Hart<M, P> {
    pub fn new() -> Self {
        Hart { program_counter: 0, register_file: RegisterFile::new(32), pipeline: P::new(), phantom: PhantomData }
    }

    pub fn execute(&mut self, memory: &mut M) {
        self.program_counter = self.pipeline.execute(self.program_counter, &mut self.register_file, memory);
    }
}
