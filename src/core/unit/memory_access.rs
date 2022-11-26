use super::super::memory_interface::{Interface, MemoryInterface};
use super::RegisterWrite;
use crate::core::instruction::{MemoryLoadInstruction, MemoryStoreInstruction};

use MemoryLoadInstruction::*;
use MemoryStoreInstruction::*;

pub fn store<M: MemoryInterface<u32>>(decode_result: MemoryStoreInstruction, memory: &mut M) {
    match decode_result {
        SB(instr) => {
            memory.write(instr.register_source_one.value + instr.immediate, instr.register_source_two.value as u8);
        }
        SH(instr) => {
            memory.write(instr.register_source_one.value + instr.immediate, instr.register_source_two.value as u16);
        }
        SW(instr) => {
            memory.write(instr.register_source_one.value + instr.immediate, instr.register_source_two.value as u32);
        }
    };
}

pub fn load<M: MemoryInterface<u32>>(decode_result: MemoryLoadInstruction, memory: &M) -> RegisterWrite {
    match decode_result {
        LB(instr) => RegisterWrite {
            index: instr.register_destination_index,
            value: Interface::<u32, i8>::read(memory, instr.register_source_one.value + instr.immediate) as u32,
        },
        LBU(instr) => RegisterWrite {
            index: instr.register_destination_index,
            value: Interface::<u32, u8>::read(memory, instr.register_source_one.value + instr.immediate) as u32,
        },
        LH(instr) => RegisterWrite {
            index: instr.register_destination_index,
            value: Interface::<u32, i16>::read(memory, instr.register_source_one.value + instr.immediate) as u32,
        },
        LHU(instr) => RegisterWrite {
            index: instr.register_destination_index,
            value: Interface::<u32, u16>::read(memory, instr.register_source_one.value + instr.immediate) as u32,
        },
        LW(instr) => RegisterWrite {
            index: instr.register_destination_index,
            value: Interface::<u32, u32>::read(memory, instr.register_source_one.value + instr.immediate) as u32,
        },
    }
}
