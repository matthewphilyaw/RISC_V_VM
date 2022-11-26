use super::{memory_interface::MemoryInterface, register_file::RegisterFile};

pub trait Pipeline<M: MemoryInterface<u32>> {
    fn new() -> Self;
    fn execute(&mut self, pc: u32, register_file: &mut RegisterFile, memory: &mut M) -> u32;
}
