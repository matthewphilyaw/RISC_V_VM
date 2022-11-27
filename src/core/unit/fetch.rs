use crate::core::bus::BusInterface;

#[derive(Copy, Clone)]
pub struct FetchResult {
    pub captured_pc: u32,
    pub instruction: u32,
}

pub fn fetch_instruction<M: BusInterface<u32, u32>>(pc: u32, memory: &M) -> Option<FetchResult> {
    let instruction: u32 = BusInterface::<u32, u32>::read(memory, pc);
    Some(FetchResult { captured_pc: pc, instruction })
}
