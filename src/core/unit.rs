pub use alu::*;
pub use branching::*;
pub use decoder::*;
pub use memory_access::*;
pub use write_back::*;

mod alu;
mod branching;
mod decoder;
mod memory_access;
mod write_back;

#[derive(Clone, Copy)]
pub struct RegisterWrite {
    pub index: u32,
    pub value: u32,
}

#[derive(Copy, Clone)]
pub struct FetchResult {
    pub captured_pc: u32,
    pub instruction: u32,
}