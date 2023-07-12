use bits::match_mask;
use enum_dispatch::*;
use strum::EnumIter;

use super::encoding::{GenInstruction, Instruction};
use crate::helpers::sext;
use crate::cpu;
use crate::system::rv32;

#[derive(Default, Copy, Clone)]
pub struct ADDI;

impl Instruction for ADDI {
    fn name(&self) -> &'static str {
        "ADDI"
    }

    fn match_inst(&self, inst: rv32::Word) -> bool {
        println!("VM > Checking ADDI");
        println!("VM > ADDI: 0b{:032b}", inst);
        println!("VM > ADDI: 0bxxxxxxxxxxxxxxxxx000xxxxx0010011");
        match_mask!(inst, "xxxxxxxxxxxxxxxxx000xxxxx0010011")
    }

    fn step(&self, inst: GenInstruction, state: &mut cpu::CPUState) {
        println!("VM > Executing ADDI");
        let inst = unsafe { inst.I };
        state.x[inst.rd() as usize] = state.x[inst.rs1() as usize].wrapping_add(sext(inst.imm() as u32, 32));
    }
}

#[derive(Default, Copy, Clone)]
pub struct ADD;

impl Instruction for ADD {
    fn name(&self) -> &'static str {
        "ADD"
    }

    fn match_inst(&self, inst: rv32::Word) -> bool {
        println!("VM > Checking ADD");
        println!("VM > ADD: 0b{:032b}", inst);
        println!("VM > ADD: 0b0000000xxxxxxxxxx000xxxxx0110011");
        match_mask!(inst, "0000000xxxxxxxxxx000xxxxx0110011")
    }

    fn step(&self, inst: GenInstruction, state: &mut cpu::CPUState) {
        println!("VM > Executing ADD");
        let inst = unsafe { inst.R };
        state.x[inst.rd() as usize] =
            state.x[inst.rs1() as usize].wrapping_add(state.x[inst.rs2() as usize]);
    }
}

#[enum_dispatch(Instruction)]
#[derive(EnumIter)]
pub enum ExtensionI {
    ADDI(ADDI),
    ADD(ADD),
}

