use super::{bus::BusInterface, register_file::RegisterFile};

pub trait Pipeline<M>
where
    M: BusInterface<u32, i8>,
    M: BusInterface<u32, u8>,
    M: BusInterface<u32, i16>,
    M: BusInterface<u32, u16>,
    M: BusInterface<u32, u32>,
{
    fn new() -> Self;
    fn execute(&mut self, pc: u32, register_file: &mut RegisterFile, memory: &mut M) -> u32;
}
