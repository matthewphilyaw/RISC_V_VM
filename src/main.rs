use crate::core::hart::Hart;

use crate::memory::Memory;
use crate::simple_pipeline::SimplePipeline;

mod core;
mod memory;
mod simple_pipeline;

fn main() {
    let program = vec![
        19u8, 1, 0, 12, 239, 0, 192, 2, 3, 35, 0, 9, 19, 3, 19, 0, 35, 40, 96, 8, 111, 240, 31, 255, 19, 1, 193, 255,
        35, 32, 17, 0, 35, 32, 176, 8, 131, 32, 1, 0, 19, 1, 65, 0, 103, 128, 0, 0, 19, 1, 193, 255, 35, 32, 17, 0, 3,
        35, 0, 9, 147, 3, 176, 0, 179, 5, 115, 0, 239, 240, 95, 253, 131, 32, 1, 0, 19, 1, 65, 0, 103, 128, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let mut full_memory = vec![0u8; 1024];
    full_memory[0..program.len()].clone_from_slice(&program);

    let mut memory = Memory::with_initial_values(full_memory);
    let mut hart = Hart::<Memory, SimplePipeline>::new();

    for _ in 0..1000 {
        hart.execute(&mut memory);
    }
}
