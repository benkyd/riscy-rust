use modular_bitfield::prelude::*;

use crate::system::rv32;

pub const R_TYPE: u8 = 0b00110011;
pub const I_TYPE: u8 = 0b00010011;
pub const S_TYPE: u8 = 0b00100011;
pub const B_TYPE: u8 = 0b01100011;
pub const U_TYPE: u8 = 0b00110111;
pub const J_TYPE: u8 = 0b01110011;

// Null undecided type
#[bitfield]
#[derive(Debug)]
pub struct NullType {
    pub opcode: B7,
    pub _undefined: B25,
}

// Arithmetic logic
#[bitfield]
#[derive(Debug)]
pub struct RType {
    pub opcode: B7,
    pub rd: B5,
    pub funct3: B3,
    pub rs1: B5,
    pub rs2: B5,
    pub funct7: B7,
}

// Loads & immeiate arithmetic
#[bitfield]
#[derive(Debug)]
pub struct IType {
    pub opcode: B7,
    pub rd: B5,
    pub funct3: B3,
    pub rs1: B5,
    pub imm: B12,
}

// Stores
#[bitfield]
#[derive(Debug)]
pub struct SType {
    pub opcode: B7,
    pub imm_l: B5,
    pub funct3: B3,
    pub rs1: B5,
    pub rs2: B5,
    pub imm_h: B7,
}

// Conditional jump
#[bitfield]
#[derive(Debug)]
pub struct BType {
    pub opcode: B7,
    pub imm_11: B1,
    pub imm_4_1: B4,
    pub funct3: B3,
    pub rs1: B5,
    pub rs2: B5,
    pub imm_10_5: B6,
    pub imm_12: B1,
}

// Upper immediate
#[bitfield]
#[derive(Debug)]
pub struct UType {
    pub opcode: B7,
    pub rd: B5,
    pub imm: B20,
}

// Unconditional jump
#[bitfield]
#[derive(Debug)]
pub struct JType {
    pub opcode: B7,
    pub rd: B5,
    pub imm_19_12: B8,
    pub imm_11: B1,
    pub imm_10_1: B10,
    pub imm_20: B1,
}

/* #[derive(Debug)]
pub enum Decode {
    null(std::mem::ManuallyDrop<NullType>),
    R(std::mem::ManuallyDrop<RType>),
    I(std::mem::ManuallyDrop<IType>),
    S(std::mem::ManuallyDrop<SType>),
    B(std::mem::ManuallyDrop<BType>),
    U(std::mem::ManuallyDrop<UType>),
    J(std::mem::ManuallyDrop<JType>),
} */

#[repr(align(8))]
pub union Instruction {
    pub inst: rv32::Word,
    pub null: std::mem::ManuallyDrop<NullType>,
    pub R: std::mem::ManuallyDrop<RType>,
    pub I: std::mem::ManuallyDrop<IType>,
    pub S: std::mem::ManuallyDrop<SType>,
    pub B: std::mem::ManuallyDrop<BType>,
    pub U: std::mem::ManuallyDrop<UType>,
    pub J: std::mem::ManuallyDrop<JType>,
}
