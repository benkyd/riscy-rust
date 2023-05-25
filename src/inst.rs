use modular_bitfield::prelude::*;

use crate::rv32;

// Arithmetic logic
#[bitfield]
pub struct RType {
    opcode: B7,
    rd: B5,
    funct3: B3,
    rs1: B5,
    rs2: B5,
    funct7: B7,
}

// Loads & immeiate arithmetic
#[bitfield]
pub struct IType {
    opcode: B7,
    rd: B5,
    funct3: B3,
    rs1: B5,
    imm: B12,
}

// Stores
#[bitfield]
pub struct SType {
    opcode: B7,
    imm_l: B5,
    funct3: B3,
    rs1: B5,
    rs2: B5,
    imm_h: B7,
}

// Conditional jump
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

// Upper immediate
#[bitfield]
pub struct UType {
    opcode: B7,
    rd: B5,
    imm: B20,
}


// Unconditional jump
#[bitfield]
pub struct JType {
    opcode: B7,
    rd: B5,
    imm_19_12: B8,
    imm_11: B1,
    imm_10_1: B10,
    imm_20: B1,
}

#[repr(align(8))]
pub union Instruction {
    pub inst: rv32::Word,
    pub r: std::mem::ManuallyDrop<RType>,
    pub i: std::mem::ManuallyDrop<IType>,
    pub s: std::mem::ManuallyDrop<SType>,
    pub b: std::mem::ManuallyDrop<BType>,
    pub u: std::mem::ManuallyDrop<UType>,
    pub j: std::mem::ManuallyDrop<JType>,
}
