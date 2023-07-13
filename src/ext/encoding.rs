use modular_bitfield::prelude::*;
use enum_dispatch::*;

use crate::system::rv32;
use crate::cpu;
use crate::helpers;

#[enum_dispatch]
pub trait Instruction {
    fn name(&self) -> &'static str;
    fn match_inst(&self, inst: rv32::Word) -> bool;
    fn step(&self, inst: GenInstruction, state: &mut cpu::CPUState);
}

pub trait ImmediateMode {
    fn sext_imm(&self) -> rv32::XLen;
    fn full_imm(&self) -> rv32::XLen;
}

// Null undecided type
#[bitfield]
#[derive(Debug)]
pub struct NullType {
    pub opcode: B7,
    pub _unused: B25,
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

impl ImmediateMode for IType {
    fn sext_imm(&self) -> rv32::XLen {
        helpers::sext(self.full_imm(), 12)
    }

    fn full_imm(&self) -> rv32::XLen {
        self.imm() as rv32::XLen
    }
}

#[bitfield]
pub struct SType {
    opcode: B7,
    imm_4_0: B5,
    funct3: B3,
    rs1: B5,
    rs2: B5,
    imm_11_5: B7,
}

// imm[11:5] = inst[31:25], imm[4:0] = inst[11:7]
impl ImmediateMode for SType {
    fn sext_imm(&self) -> rv32::XLen {
        helpers::sext(self.full_imm(), 12)
    }

    fn full_imm(&self) -> rv32::XLen {
        ((self.imm_11_5() as rv32::XLen) << 5) | self.imm_4_0() as rv32::XLen
    }
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


#[repr(align(8))]
pub union GenInstruction {
    pub inst: rv32::Word,
    pub null: std::mem::ManuallyDrop<NullType>,
    pub R: std::mem::ManuallyDrop<RType>,
    pub I: std::mem::ManuallyDrop<IType>,
    pub S: std::mem::ManuallyDrop<SType>,
    pub B: std::mem::ManuallyDrop<BType>,
    pub U: std::mem::ManuallyDrop<UType>,
    pub J: std::mem::ManuallyDrop<JType>,
}
