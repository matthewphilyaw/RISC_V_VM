use super::super::bus::BusInterface;
use super::RegisterWrite;
use crate::core::{instruction::{MemoryLoadInstruction, MemoryStoreInstruction}, bus::BusReadResponse};

use MemoryLoadInstruction::*;
use MemoryStoreInstruction::*;

pub fn store<M>(decode_result: MemoryStoreInstruction, memory: &mut M)
where
    M: BusInterface<u32, u8>,
    M: BusInterface<u32, u16>,
    M: BusInterface<u32, u32>,
{
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

pub fn load<M>(decode_result: MemoryLoadInstruction, memory: &M) -> RegisterWrite
where
    M: BusInterface<u32, i8>,
    M: BusInterface<u32, u8>,
    M: BusInterface<u32, i16>,
    M: BusInterface<u32, u16>,
    M: BusInterface<u32, u32>,
{
    let (index, memory_read) = match decode_result {
        LB(instr) => (instr.register_destination_index, BusInterface::<u32, i8>::read(memory, instr.register_source_one.value + instr.immediate)),
        LBU(instr) => (instr.register_destination_index, BusInterface::<u32, u8>::read(memory, instr.register_source_one.value + instr.immediate)),
        LH(instr) => (instr.register_destination_index, BusInterface::<u32, i16>::read(memory, instr.register_source_one.value + instr.immediate)),
        LHU(instr) => (instr.register_destination_index, BusInterface::<u32, u16>::read(memory, instr.register_source_one.value + instr.immediate)),
        LW(instr) => (instr.register_destination_index, BusInterface::<u32, u32>::read(memory, instr.register_source_one.value + instr.immediate)),
    };

    if let BusReadResponse::Success(value) = memory_read {
        RegisterWrite {
            index,
            value
        }
    } else {
        panic!("Invalid memory read");
    }
}
