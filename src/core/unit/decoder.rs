use crate::core::instruction::Instruction;

use super::super::instruction::full_opcode_constants;
use super::super::instruction::opcode_group_constants;
use super::super::instruction::*;
use super::super::register_file::RegisterFile;
use super::fetch::FetchResult;

use super::super::instruction::AluInstruction::*;
use super::super::instruction::BranchingInstruction::*;
use super::super::instruction::Instruction::*;
use super::super::instruction::MemoryLoadInstruction::*;
use super::super::instruction::MemoryStoreInstruction::*;

use DecodeError::*;

pub enum DecodeError {
    BadInstruction { address: u32, instruction: u32 },
}

pub fn decode_instruction(fetch_result: FetchResult, register_file: &RegisterFile) -> Result<Instruction, DecodeError> {
    match opcode(fetch_result.instruction) {
        opcode_group_constants::LUI => u_type(fetch_result),
        opcode_group_constants::AUIPC => u_type(fetch_result),
        opcode_group_constants::STORE => s_type(fetch_result, register_file),
        opcode_group_constants::BRANCHING => b_type(fetch_result, register_file),
        opcode_group_constants::JALR => i_type(fetch_result, register_file),
        opcode_group_constants::LOAD => i_type(fetch_result, register_file),
        opcode_group_constants::JAL => j_type(fetch_result),
        opcode_group_constants::ALU => r_type(fetch_result, register_file),
        opcode_group_constants::ALU_IMMEDIATE => match funct_3(fetch_result.instruction) {
            funct_3 if funct_3 == 0b01 || funct_3 == 0b101 => r_type(fetch_result, register_file),
            _ => i_type(fetch_result, register_file),
        },
        _ => {
            println!("{:b}", opcode(fetch_result.instruction));
            bad_instruction(fetch_result)
        }
    }
}

fn r_type(fetch_result: FetchResult, register_file: &RegisterFile) -> Result<Instruction, DecodeError> {
    let instruction = fetch_result.instruction;
    let opcode = opcode(instruction);
    let funct_3 = funct_3(instruction);
    let funct_7 = funct_7(instruction);

    let rs1 = register_source_one_index(instruction);
    let rs2 = register_source_two_index(instruction);

    let rs1_value = register_file.read(rs1 as usize);
    let rs2_value = register_file.read(rs2 as usize);

    let full_opcode = build_full_opcode(opcode, funct_3, funct_7);

    let decoded = RType {
        opcode,
        full_opcode,
        register_destination_index: register_destination_index(instruction),
        register_source_one: DecodedRegisterValue { index: register_source_one_index(instruction), value: rs1_value },
        register_source_two: DecodedRegisterValue { index: register_source_two_index(instruction), value: rs2_value },
    };

    match full_opcode {
        full_opcode_constants::SLLI => Ok(Alu(SLLI(decoded))),
        full_opcode_constants::SRLI => Ok(Alu(SRLI(decoded))),
        full_opcode_constants::SRAI => Ok(Alu(SRAI(decoded))),
        full_opcode_constants::ADD => Ok(Alu(ADD(decoded))),
        full_opcode_constants::SUB => Ok(Alu(SUB(decoded))),
        full_opcode_constants::SLL => Ok(Alu(SLL(decoded))),
        full_opcode_constants::SLT => Ok(Alu(SLT(decoded))),
        full_opcode_constants::SLTU => Ok(Alu(SLTU(decoded))),
        full_opcode_constants::XOR => Ok(Alu(XOR(decoded))),
        full_opcode_constants::SRL => Ok(Alu(SRL(decoded))),
        full_opcode_constants::SRA => Ok(Alu(SRA(decoded))),
        full_opcode_constants::OR => Ok(Alu(OR(decoded))),
        full_opcode_constants::AND => Ok(Alu(AND(decoded))),
        _ => bad_instruction(fetch_result),
    }
}

fn i_type(fetch_result: FetchResult, register_file: &RegisterFile) -> Result<Instruction, DecodeError> {
    let instruction = fetch_result.instruction;
    let opcode = opcode(instruction);
    let funct_3 = funct_3(instruction);

    let immediate = sign_extend(instruction >> 20, 12, 32);

    let rs1 = register_source_one_index(instruction);
    let rs1_value = register_file.read(rs1 as usize);
    let full_opcode = build_full_opcode(opcode, funct_3, 0);

    let decoded = IType {
        opcode,
        full_opcode,
        register_destination_index: register_destination_index(instruction),
        register_source_one: DecodedRegisterValue { index: register_source_one_index(instruction), value: rs1_value },
        immediate,
    };

    match full_opcode {
        full_opcode_constants::JALR => Ok(Branching(JALR(decoded))),
        full_opcode_constants::LB => Ok(MemoryLoad(LB(decoded))),
        full_opcode_constants::LBU => Ok(MemoryLoad(LBU(decoded))),
        full_opcode_constants::LH => Ok(MemoryLoad(LH(decoded))),
        full_opcode_constants::LHU => Ok(MemoryLoad(LHU(decoded))),
        full_opcode_constants::LW => Ok(MemoryLoad(LW(decoded))),
        full_opcode_constants::ADDI => Ok(Alu(ADDI(decoded))),
        full_opcode_constants::SLTI => Ok(Alu(SLTI(decoded))),
        full_opcode_constants::SLTIU => Ok(Alu(SLTIU(decoded))),
        full_opcode_constants::XORI => Ok(Alu(XORI(decoded))),
        full_opcode_constants::ORI => Ok(Alu(ORI(decoded))),
        full_opcode_constants::ANDI => Ok(Alu(ANDI(decoded))),
        _ => bad_instruction(fetch_result),
    }
}

fn s_type(fetch_result: FetchResult, register_file: &RegisterFile) -> Result<Instruction, DecodeError> {
    let instruction = fetch_result.instruction;
    let opcode = opcode(instruction);
    let funct_3 = funct_3(instruction);

    let immediate_lower = register_destination_index(instruction);
    let immediate_upper = funct_7(instruction);
    let immediate = sign_extend((immediate_upper << 7) | immediate_lower, 12, 32);

    let rs1 = register_source_one_index(instruction);
    let rs2 = register_source_two_index(instruction);

    let rs1_value = register_file.read(rs1 as usize);
    let rs2_value = register_file.read(rs2 as usize);
    let full_opcode = build_full_opcode(opcode, funct_3, 0);

    let decoded = SType {
        opcode,
        full_opcode,
        register_source_one: DecodedRegisterValue { index: register_source_one_index(instruction), value: rs1_value },
        register_source_two: DecodedRegisterValue { index: register_source_two_index(instruction), value: rs2_value },
        immediate,
    };

    match full_opcode {
        full_opcode_constants::SB => Ok(MemoryStore(SB(decoded))),
        full_opcode_constants::SH => Ok(MemoryStore(SH(decoded))),
        full_opcode_constants::SW => Ok(MemoryStore(SW(decoded))),
        _ => bad_instruction(fetch_result),
    }
}

fn u_type(fetch_result: FetchResult) -> Result<Instruction, DecodeError> {
    let instruction = fetch_result.instruction;
    let opcode = opcode(instruction);

    let decoded = UType {
        opcode,
        full_opcode: opcode,
        register_destination_index: register_destination_index(instruction),
        immediate: (instruction & 0xffff_f000) as u32,
    };

    match opcode {
        full_opcode_constants::LUI => Ok(Alu(LUI(decoded))),
        full_opcode_constants::AUIPC => Ok(Alu(AUIPC(decoded))),
        _ => bad_instruction(fetch_result),
    }
}

fn b_type(fetch_result: FetchResult, register_file: &RegisterFile) -> Result<Instruction, DecodeError> {
    let instruction = fetch_result.instruction;
    let opcode = opcode(instruction);
    let funct_3 = funct_3(instruction);
    let funct_7 = funct_7(instruction);

    let register_destination_index = register_destination_index(instruction);

    let immediate = sign_extend(
        ((funct_7 >> 6) << 11)
            | ((register_destination_index & 0x1) << 10)
            | ((funct_7 & 0x3f) << 4)
            | ((register_destination_index & 0x1e) >> 1),
        12,
        32,
    );

    let rs1 = register_source_one_index(instruction);
    let rs2 = register_source_two_index(instruction);

    let rs1_value = register_file.read(rs1 as usize);
    let rs2_value = register_file.read(rs2 as usize);
    let full_opcode = build_full_opcode(opcode, funct_3, 0);

    let decoded = BType {
        opcode,
        full_opcode,
        register_source_one: DecodedRegisterValue { index: register_source_one_index(instruction), value: rs1_value },
        register_source_two: DecodedRegisterValue { index: register_source_two_index(instruction), value: rs2_value },
        immediate,
    };

    match full_opcode {
        full_opcode_constants::BEQ => Ok(Branching(BEQ(decoded))),
        full_opcode_constants::BNE => Ok(Branching(BNE(decoded))),
        full_opcode_constants::BLT => Ok(Branching(BLT(decoded))),
        full_opcode_constants::BGE => Ok(Branching(BGE(decoded))),
        full_opcode_constants::BLTU => Ok(Branching(BLTU(decoded))),
        full_opcode_constants::BGEU => Ok(Branching(BGEU(decoded))),
        _ => bad_instruction(fetch_result),
    }
}

fn j_type(fetch_result: FetchResult) -> Result<Instruction, DecodeError> {
    let instruction = fetch_result.instruction;
    let opcode = opcode(instruction);
    let imm_1_to_10 = (instruction & 0x7FE00000) >> 21;
    let imm_11 = (instruction & 0x100000) >> 20;
    let imm_12_to_19 = (instruction & 0xFF000) >> 12;
    let imm_20 = (instruction & 0x80000000) >> 31;

    let immediate = sign_extend((imm_20 << 20) | (imm_12_to_19 << 12) | (imm_11 << 11) | (imm_1_to_10 << 1), 20, 32);

    let decoded = JType {
        opcode,
        full_opcode: opcode,
        register_destination_index: register_destination_index(instruction),
        immediate,
    };

    match opcode {
        full_opcode_constants::JAL => Ok(Branching(JAL(decoded))),
        _ => bad_instruction(fetch_result),
    }
}

fn opcode(instruction: u32) -> u32 {
    instruction & 0x7F
}

fn funct_3(instruction: u32) -> u32 {
    (instruction >> 12) & 0x7
}

fn funct_7(instruction: u32) -> u32 {
    (instruction >> 25) & 0x7F
}

fn register_destination_index(instruction: u32) -> u32 {
    (instruction >> 7) & 0x1f
}

fn register_source_one_index(instruction: u32) -> u32 {
    (instruction >> 15) & 0x1f
}

fn register_source_two_index(instruction: u32) -> u32 {
    (instruction >> 20) & 0x1f
}

fn build_full_opcode(opcode: u32, funct_3: u32, funct_7: u32) -> u32 {
    (funct_7 << 10) | (funct_3 << 7) | opcode
}

fn sign_extend(value: u32, bits: usize, width: usize) -> u32 {
    let cast = value as i32;
    let shift_by = width - bits;
    let value: i32 = (cast << shift_by) >> shift_by;

    return value as u32;
}

fn bad_instruction(fetch_result: FetchResult) -> Result<Instruction, DecodeError> {
    Err(BadInstruction { address: fetch_result.captured_pc, instruction: fetch_result.instruction })
}
