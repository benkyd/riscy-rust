use std::usize;

use bits::match_mask;
use enum_dispatch::*;
use strum::EnumIter;

use super::encoding::{GenInstruction, Instruction};
use crate::cpu;
use crate::ext::encoding::ImmediateMode;
use crate::helpers::sext;
use crate::system::rv32;

// FOR BRANCH INSTRUCTIONS ITS IMPERATIVE TO REMEMBER
// THAT WE INCREMENT PC AFTER THE EXECUTION

#[derive(Default, Copy, Clone)]
pub struct MULW; // MULW rd, rs1, rs2 - Multiply Word
                 // Multiply rs1 and rs2 and store the lower 32 bits in rd
impl Instruction for MULW {
    fn name(&self) -> &'static str {
        "MULW"
    }

    fn match_inst(&self, inst: rv32::Word) -> bool {
        match_mask!(inst, "xxxxxxxxxxxxxxxxxxxxxxxxx0110111")
    }

    fn step(&self, inst: GenInstruction, state: &mut cpu::CPUState) {
        let inst = unsafe { inst.U };
        let val = inst.full_imm() << 12;
        state.x[inst.rd() as usize] = val;
    }
}

#[derive(EnumIter)]
#[enum_dispatch(Instruction)]
pub enum ExtensionM {
    MULW(MULW),
}

