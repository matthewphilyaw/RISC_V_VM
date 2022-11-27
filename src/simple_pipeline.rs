use std::mem::size_of;

use crate::core::bus::BusInterface;
use crate::core::unit::{
    branch, decode_instruction, execute, fetch_instruction, load, store, write_back, DecodeError, FetchResult,
    RegisterWrite,
};

use crate::core::pipeline::Pipeline;
use crate::core::register_file::RegisterFile;

use crate::core::instruction::Instruction;

#[derive(Clone, Copy)]
struct DecodedInput {
    fetch_result: FetchResult,
}

#[derive(Clone, Copy)]
struct AluInput {
    fetch_result: FetchResult,
    decoded_instruction: Instruction,
}

#[derive(Clone, Copy)]
struct MemoryAccessInput {
    fetch_result: FetchResult,
    decoded_instruction: Instruction,
    operation: Option<RegisterWrite>,
}

#[derive(Clone, Copy)]
struct WriteBackInput {
    fetch_result: FetchResult,
    decoded_instruction: Instruction,
    operation: Option<RegisterWrite>,
}

#[derive(Clone, Copy)]
pub struct SimplePipeline {
    decode_input: Option<DecodedInput>,
    execute_input: Option<AluInput>,
    memory_access_input: Option<MemoryAccessInput>,
    write_back_input: Option<WriteBackInput>,
}

impl<M> Pipeline<M> for SimplePipeline
where
    M: BusInterface<u32, i8>,
    M: BusInterface<u32, u8>,
    M: BusInterface<u32, i16>,
    M: BusInterface<u32, u16>,
    M: BusInterface<u32, u32>,
{
    fn new() -> Self {
        SimplePipeline { decode_input: None, execute_input: None, memory_access_input: None, write_back_input: None }
    }

    fn execute(&mut self, pc: u32, register_file: &mut RegisterFile, memory: &mut M) -> u32 {
        if let Some(WriteBackInput { fetch_result: _, decoded_instruction: _, operation }) = self.write_back_input {
            operation.map(|op| write_back(op, register_file));
        }

        let next_decode_input = fetch_instruction(pc, memory).map(|fetch_result| DecodedInput { fetch_result });
        let next_execute_input = self.decode_input.map(|decoded_input| decode_stage(decoded_input, register_file));
        let next_memory_access_input = self.execute_input.map(execute_stage);
        let next_write_back_input =
            self.memory_access_input.map(|memory_access_input| memory_stage(memory_access_input, memory));

        self.decode_input = next_decode_input;
        self.execute_input = next_execute_input;
        self.memory_access_input = next_memory_access_input;
        self.write_back_input = next_write_back_input;

        match next_execute_input.and_then(will_branch) {
            Some(jump_to_address) => {
                self.decode_input = None;
                self.execute_input = None;
                jump_to_address
            }
            None => pc + size_of::<u32>() as u32,
        }
    }
}

fn decode_stage(DecodedInput { fetch_result }: DecodedInput, register_file: &RegisterFile) -> AluInput {
    match decode_instruction(fetch_result, register_file) {
        Ok(decoded_instruction) => AluInput { fetch_result, decoded_instruction },
        Err(DecodeError::BadInstruction { address, instruction }) => {
            panic!("Invalid instruction. address: {:x}, instruction: {:0>32b}", address, instruction)
        }
    }
}

fn execute_stage(AluInput { fetch_result, decoded_instruction }: AluInput) -> MemoryAccessInput {
    MemoryAccessInput {
        fetch_result,
        decoded_instruction,
        operation: match decoded_instruction {
            Instruction::Alu(instr) => Some(execute(fetch_result, instr)),
            _ => None,
        },
    }
}

fn will_branch(AluInput { fetch_result, decoded_instruction }: AluInput) -> Option<u32> {
    match decoded_instruction {
        Instruction::Branching(instr) => branch(fetch_result, instr),
        _ => None,
    }
}

fn memory_stage<M>(
    MemoryAccessInput { fetch_result, decoded_instruction, operation }: MemoryAccessInput,
    memory: &mut M,
) -> WriteBackInput
where
    M: BusInterface<u32, i8>,
    M: BusInterface<u32, u8>,
    M: BusInterface<u32, i16>,
    M: BusInterface<u32, u16>,
    M: BusInterface<u32, u32>,
{
    let op = match decoded_instruction {
        Instruction::MemoryLoad(instr) => Some(load(instr, memory)),
        Instruction::MemoryStore(instr) => {
            store(instr, memory);
            None
        }
        _ => operation,
    };

    WriteBackInput { fetch_result, decoded_instruction, operation: op }
}
