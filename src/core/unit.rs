pub use alu::*;
pub use branching::*;
pub use decoder::*;
pub use fetch::*;
pub use memory_access::*;
pub use write_back::*;

mod alu;
mod branching;
mod decoder;
mod fetch;
mod memory_access;
mod write_back;

#[derive(Clone, Copy)]
pub struct RegisterWrite {
    pub index: u32,
    pub value: u32,
}
