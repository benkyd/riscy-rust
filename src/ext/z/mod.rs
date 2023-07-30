use std::usize;

use bits::match_mask;
use enum_dispatch::*;
use strum::EnumIter;

use super::encoding::{GenInstruction, Instruction};
use crate::cpu;
use crate::ext::encoding::ImmediateMode;
use crate::helpers::sext;
use crate::system::rv32;

// ZiCSR - Control and Status Register Instructions
// WILL ALSO MATCH ZIENCEI INSTRUCTIONS
// ALTHOUGH THEY ARE NOT IMPLEMENTED YET

// FOR BRANCH INSTRUCTIONS ITS IMPERATIVE TO REMEMBER
// THAT WE INCREMENT PC AFTER THE EXECUTION

#[derive(Default, Copy, Clone)]
pub struct CSRRW; // CSRRW rd, rs1, csr - Atomic Read/Write CSR
                  // Read the CSR into rd, then write rs1 into the CSR
                  // rd = csr
                  // csr = rs1
impl Instruction for CSRRW {
    fn name(&self) -> &'static str {
        "CSRRW"
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
pub enum ExtensionZ {
    CSRRW(CSRRW),
}

