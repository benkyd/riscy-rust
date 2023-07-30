use std::usize;

use bits::match_mask;
use enum_dispatch::*;
use strum::EnumIter;

use super::encoding::{GenInstruction, Instruction};
use crate::cpu;
use crate::helpers::sext;
use crate::system::rv32;

// FOR BRANCH INSTRUCTIONS ITS IMPERATIVE TO REMEMBER
// THAT WE INCREMENT PC AFTER THE EXECUTION

#[derive(Default, Copy, Clone)]
pub struct LRW; // LR.W rd, rs1 - Load Reserved Word
                // Load a word from memory into rd
                // and set the extraflags to rs1
                // rd = mem[rs1]
impl Instruction for LRW {
    fn name(&self) -> &'static str {
        "LR.W"
    }

    fn match_inst(&self, inst: rv32::Word) -> bool {
        match_mask!(inst, "00010xx00000xxxxx010xxxxx0101111")
    }

    fn step(&self, inst: GenInstruction, state: &mut cpu::CPUState) {
        let inst = unsafe { inst.R };
        let rs1 = state.x[inst.rs1() as usize];
        state.extraflags = (state.extraflags & 0x07) | (rs1 << 3);
        state.x[inst.rd() as usize] = sext(state.bus.borrow_mut().load_32(rs1), 32);
    }
}

#[derive(Default, Copy, Clone)]
pub struct SCW; // SC.W rd, rs1 - Store Conditional Word
                // Store a word from rd into memory
                // if the extraflags match rs1
                // mem[rs1] = rd
impl Instruction for SCW {
    fn name(&self) -> &'static str {
        "SC.W"
    }

    fn match_inst(&self, inst: rv32::Word) -> bool {
        match_mask!(inst, "00011xxxxxxxxxxxx010xxxxx0101111")
    }

    fn step(&self, inst: GenInstruction, state: &mut cpu::CPUState) {
        let inst = unsafe { inst.R };
        let rs1 = state.x[inst.rs1() as usize];
        let write_flag = (state.extraflags >> 3) == (rs1 & 0x1fffffff);
        if write_flag {
            state.bus.borrow_mut().store_32(rs1, state.x[inst.rs2() as usize]);
            state.x[inst.rd() as usize] = 0;
        } else {
            state.x[inst.rd() as usize] = 1;
        }
    }
}

#[derive(EnumIter)]
#[enum_dispatch(Instruction)]
pub enum ExtensionA {
    LRW(LRW),
    SCW(SCW),
}
