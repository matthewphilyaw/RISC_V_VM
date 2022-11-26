use super::{super::instruction::*, FetchResult};
use BranchingInstruction::*;

pub fn branch(fetch_result: FetchResult, decode_result: BranchingInstruction) -> Option<u32> {
    match decode_result {
        JAL(instr) => jal(fetch_result, instr),
        JALR(instr) => jalr(instr),
        BEQ(instr) => beq(fetch_result, instr),
        BNE(instr) => bne(fetch_result, instr),
        BLT(instr) => blt(fetch_result, instr),
        BLTU(instr) => bltu(fetch_result, instr),
        BGE(instr) => bge(fetch_result, instr),
        BGEU(instr) => bgeu(fetch_result, instr),
    }
}

fn jal(fetch_result: FetchResult, instr: JType) -> Option<u32> {
    Some(fetch_result.captured_pc.wrapping_add(instr.immediate))
}

fn jalr(instr: IType) -> Option<u32> {
    Some(instr.register_source_one.value.wrapping_add(instr.immediate))
}

fn beq(fetch_result: FetchResult, instr: BType) -> Option<u32> {
    match instr.register_source_one.value == instr.register_source_two.value {
        true => Some(fetch_result.captured_pc.wrapping_add(instr.immediate)),
        false => None,
    }
}

fn bne(fetch_result: FetchResult, instr: BType) -> Option<u32> {
    match instr.register_source_one.value != instr.register_source_two.value {
        true => Some(fetch_result.captured_pc.wrapping_add(instr.immediate)),
        false => None,
    }
}

fn blt(fetch_result: FetchResult, instr: BType) -> Option<u32> {
    match (instr.register_source_one.value as i32) < (instr.register_source_two.value as i32) {
        true => Some(fetch_result.captured_pc.wrapping_add(instr.immediate)),
        false => None,
    }
}

fn bltu(fetch_result: FetchResult, instr: BType) -> Option<u32> {
    match instr.register_source_one.value < instr.register_source_two.value {
        true => Some(fetch_result.captured_pc.wrapping_add(instr.immediate)),
        false => None,
    }
}

fn bge(fetch_result: FetchResult, instr: BType) -> Option<u32> {
    match (instr.register_source_one.value as i32) >= (instr.register_source_two.value as i32) {
        true => Some(fetch_result.captured_pc.wrapping_add(instr.immediate)),
        false => None,
    }
}

fn bgeu(fetch_result: FetchResult, instr: BType) -> Option<u32> {
    match instr.register_source_one.value >= instr.register_source_two.value {
        true => Some(fetch_result.captured_pc.wrapping_add(instr.immediate)),
        false => None,
    }
}
