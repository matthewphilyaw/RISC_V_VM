use crate::core::memory_interface::Interface;

use super::super::memory_interface::MemoryInterface;

#[derive(Copy, Clone)]
pub struct FetchResult {
    pub captured_pc: u32,
    pub instruction: u32,
}

pub fn fetch_instruction<M: MemoryInterface<u32>>(pc: u32, memory: &M) -> Option<FetchResult> {
    let instruction: u32 = Interface::<u32, u32>::read(memory, pc);
    Some(FetchResult { captured_pc: pc, instruction })
}
