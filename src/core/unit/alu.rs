use AluInstruction::*;

use super::super::instruction::*;
use super::{FetchResult, RegisterWrite};

pub fn execute(fetch_result: FetchResult, decode_result: AluInstruction) -> RegisterWrite {
    match decode_result {
        LUI(instr) => lui(instr),
        AUIPC(instr) => auipc(fetch_result, instr),
        ADDI(instr) => addi(instr),
        SLTI(instr) => slti(instr),
        SLTIU(instr) => sltiu(instr),
        XORI(instr) => xori(instr),
        ORI(instr) => ori(instr),
        ANDI(instr) => andi(instr),
        SLLI(instr) => slli(instr),
        SRLI(instr) => srli(instr),
        SRAI(instr) => srai(instr),
        ADD(instr) => add(instr),
        SUB(instr) => sub(instr),
        SLT(instr) => slt(instr),
        SLTU(instr) => sltu(instr),
        XOR(instr) => xor(instr),
        SLL(instr) => sll(instr),
        SRL(instr) => srl(instr),
        SRA(instr) => sra(instr),
        OR(instr) => or(instr),
        AND(instr) => and(instr),
    }
}

fn lui(instr: UType) -> RegisterWrite {
    RegisterWrite { index: instr.register_destination_index, value: instr.immediate }
}

fn auipc(fetch_result: FetchResult, instr: UType) -> RegisterWrite {
    RegisterWrite {
        index: instr.register_destination_index,
        value: fetch_result.captured_pc.wrapping_add(instr.immediate),
    }
}

fn addi(instr: IType) -> RegisterWrite {
    RegisterWrite {
        index: instr.register_destination_index,
        value: instr.register_source_one.value.wrapping_add(instr.immediate),
    }
}

fn slti(instr: IType) -> RegisterWrite {
    RegisterWrite {
        index: instr.register_destination_index,
        value: set_less_than(instr.register_source_one.value, instr.immediate),
    }
}

fn sltiu(instr: IType) -> RegisterWrite {
    RegisterWrite {
        index: instr.register_destination_index,
        value: set_less_than_unsigned(instr.register_source_one.value, instr.immediate),
    }
}

fn xori(instr: IType) -> RegisterWrite {
    RegisterWrite { index: instr.register_destination_index, value: instr.register_source_one.value ^ instr.immediate }
}

fn ori(instr: IType) -> RegisterWrite {
    RegisterWrite { index: instr.register_destination_index, value: instr.register_source_one.value | instr.immediate }
}

fn andi(instr: IType) -> RegisterWrite {
    RegisterWrite { index: instr.register_destination_index, value: instr.register_source_one.value & instr.immediate }
}

fn slli(instr: RType) -> RegisterWrite {
    RegisterWrite {
        index: instr.register_destination_index,
        value: instr.register_source_one.value << instr.register_source_two.value,
    }
}

fn srli(instr: RType) -> RegisterWrite {
    RegisterWrite {
        index: instr.register_destination_index,
        value: instr.register_source_one.value >> instr.register_source_two.value,
    }
}

fn and(instr: RType) -> RegisterWrite {
    RegisterWrite {
        index: instr.register_destination_index,
        value: instr.register_source_one.value & instr.register_source_two.value,
    }
}

fn or(instr: RType) -> RegisterWrite {
    RegisterWrite {
        index: instr.register_destination_index,
        value: instr.register_source_one.value | instr.register_source_two.value,
    }
}

fn sra(instr: RType) -> RegisterWrite {
    RegisterWrite {
        index: instr.register_destination_index,
        value: arithmetic_shift(instr.register_source_one.value, instr.register_source_two.value),
    }
}

fn srl(instr: RType) -> RegisterWrite {
    RegisterWrite {
        index: instr.register_destination_index,
        value: instr.register_source_one.value >> instr.register_source_two.value,
    }
}

fn sll(instr: RType) -> RegisterWrite {
    RegisterWrite {
        index: instr.register_destination_index,
        value: instr.register_source_one.value << instr.register_source_two.value,
    }
}

fn xor(instr: RType) -> RegisterWrite {
    RegisterWrite {
        index: instr.register_destination_index,
        value: instr.register_source_one.value ^ instr.register_source_two.value,
    }
}

fn sltu(instr: RType) -> RegisterWrite {
    RegisterWrite {
        index: instr.register_destination_index,
        value: set_less_than_unsigned(instr.register_source_one.value, instr.register_source_two.value),
    }
}

fn slt(instr: RType) -> RegisterWrite {
    RegisterWrite {
        index: instr.register_destination_index,
        value: set_less_than(instr.register_source_one.value, instr.register_source_two.value),
    }
}

fn sub(instr: RType) -> RegisterWrite {
    RegisterWrite {
        index: instr.register_destination_index,
        value: instr.register_source_two.value.wrapping_sub(instr.register_source_one.value),
    }
}

fn add(instr: RType) -> RegisterWrite {
    RegisterWrite {
        index: instr.register_destination_index,
        value: instr.register_source_one.value.wrapping_add(instr.register_source_one.value),
    }
}

fn srai(instr: RType) -> RegisterWrite {
    RegisterWrite {
        index: instr.register_destination_index,
        value: arithmetic_shift(instr.register_source_one.value, instr.register_source_two.value),
    }
}

fn set_less_than(a: u32, b: u32) -> u32 {
    u32::from((a as i32) < (b as i32))
}

fn set_less_than_unsigned(a: u32, b: u32) -> u32 {
    u32::from(a < b)
}

fn arithmetic_shift(a: u32, shift_by: u32) -> u32 {
    ((a as i32) >> shift_by) as u32
}
