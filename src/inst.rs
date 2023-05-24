use modular_bitfield::prelude::*;

#[bitfield]
pub struct RType {
    opcode: B7,
    rd: B5,
    funct3: B3,
    rs1: B5,
    rs2: B5,
    funct7: B7,
}

#[bitfield]
pub struct IType {
    opcode: B7,
    rd: B5,
    funct3: B3,
    rs1: B5,
    imm: B12,
}

#[bitfield]
pub struct SType {
    opcode: B7,
    imm_l: B5,
    funct3: B3,
    rs1: B5,
    rs2: B5,
    imm_h: B7,
}

#[bitfield]
pub struct BType {
    opcode: B7,
    imm_11: B1,
    imm_4_1: B4,
    funct3: B3,
    rs1: B5,
    rs2: B5,
    imm_10_5: B6,
    imm_12: B1,
}

#[bitfield]
pub struct UType {
    opcode: B7,
    rd: B5,
    imm: B20,
}

#[bitfield]
pub struct JType {
    opcode: B7,
    rd: B5,
    imm_19_12: B8,
    imm_11: B1,
    imm_10_1: B10,
    imm_20: B1,
}
