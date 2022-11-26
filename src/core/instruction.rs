pub mod full_opcode_constants;
pub mod opcode_group_constants;

#[derive(Copy, Clone)]
pub struct DecodedRegisterValue {
    pub index: u32,
    pub value: u32,
}

#[derive(Copy, Clone)]
pub struct JType {
    pub opcode: u32,
    pub full_opcode: u32,
    pub register_destination_index: u32,
    pub immediate: u32,
}

#[derive(Copy, Clone)]
pub struct UType {
    pub opcode: u32,
    pub full_opcode: u32,
    pub register_destination_index: u32,
    pub immediate: u32,
}

#[derive(Copy, Clone)]
pub struct BType {
    pub opcode: u32,
    pub full_opcode: u32,
    pub register_source_one: DecodedRegisterValue,
    pub register_source_two: DecodedRegisterValue,
    pub immediate: u32,
}

#[derive(Copy, Clone)]
pub struct SType {
    pub opcode: u32,
    pub full_opcode: u32,
    pub register_source_one: DecodedRegisterValue,
    pub register_source_two: DecodedRegisterValue,
    pub immediate: u32,
}

#[derive(Copy, Clone)]
pub struct IType {
    pub opcode: u32,
    pub full_opcode: u32,
    pub register_destination_index: u32,
    pub register_source_one: DecodedRegisterValue,
    pub immediate: u32,
}

#[derive(Copy, Clone)]
pub struct RType {
    pub opcode: u32,
    pub full_opcode: u32,
    pub register_destination_index: u32,
    pub register_source_one: DecodedRegisterValue,
    pub register_source_two: DecodedRegisterValue,
}

#[derive(Clone, Copy)]
pub enum Instruction {
    Alu(AluInstruction),
    Branching(BranchingInstruction),
    MemoryLoad(MemoryLoadInstruction),
    MemoryStore(MemoryStoreInstruction),
}

#[derive(Clone, Copy)]
pub enum AluInstruction {
    LUI(UType),
    AUIPC(UType),
    ADDI(IType),
    SLTI(IType),
    SLTIU(IType),
    XORI(IType),
    ORI(IType),
    ANDI(IType),
    SLLI(RType),
    SRLI(RType),
    SRAI(RType),
    ADD(RType),
    SUB(RType),
    SLL(RType),
    SLT(RType),
    SLTU(RType),
    XOR(RType),
    SRL(RType),
    SRA(RType),
    OR(RType),
    AND(RType),
}

#[derive(Clone, Copy)]
pub enum BranchingInstruction {
    JAL(JType),
    JALR(IType),
    BEQ(BType),
    BNE(BType),
    BLT(BType),
    BGE(BType),
    BLTU(BType),
    BGEU(BType),
}

#[derive(Clone, Copy)]
pub enum MemoryLoadInstruction {
    LB(IType),
    LH(IType),
    LW(IType),
    LBU(IType),
    LHU(IType),
}

#[derive(Clone, Copy)]
pub enum MemoryStoreInstruction {
    SB(SType),
    SH(SType),
    SW(SType),
}
