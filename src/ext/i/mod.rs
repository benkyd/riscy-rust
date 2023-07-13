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
pub struct LUI; // Load Upper Immediate
                // Load the immedate mode value into the MSB of rd
                // The last 12 bits of rd should be 0
impl Instruction for LUI {
    fn name(&self) -> &'static str {
        "LUI"
    }

    fn match_inst(&self, inst: rv32::Word) -> bool {
        match_mask!(inst, "xxxxxxxxxxxxxxxxxxxxxxxxx0110111")
    }

    fn step(&self, inst: GenInstruction, state: &mut cpu::CPUState) {
        println!("VM > Executing LUI");
        let inst = unsafe { inst.U };
        let val = inst.full_imm() << 12;
        state.x[inst.rd() as usize] = val;
    }
}

#[derive(Default, Copy, Clone)]
pub struct AUIPC; // Add Upper Immedate to PC
                  // Set rd to the immediate mode value + pc
impl Instruction for AUIPC {
    fn name(&self) -> &'static str {
        "AUIPC"
    }

    fn match_inst(&self, inst: rv32::Word) -> bool {
        match_mask!(inst, "xxxxxxxxxxxxxxxxxxxxxxxxx0010111")
    }

    fn step(&self, inst: GenInstruction, state: &mut cpu::CPUState) {
        println!("VM > Executing AUIPC");
        let inst = unsafe { inst.U };
        let val = inst.full_imm() << 12;
        let pc_add = state.pc.wrapping_add(val);
        state.x[inst.rd() as usize] = pc_add;
    }
}

#[derive(Default, Copy, Clone)]
pub struct JAL; // Jump and Link
                // Set pc to offset (imm) + pc
                // Set rd to the old pc + 4
impl Instruction for JAL {
    fn name(&self) -> &'static str {
        "JAL"
    }

    fn match_inst(&self, inst: rv32::Word) -> bool {
        match_mask!(inst, "xxxxxxxxxxxxxxxxxxxxxxxxx1101111")
    }

    fn step(&self, inst: GenInstruction, state: &mut cpu::CPUState) {
        println!("VM > Executing JAL");
        let inst = unsafe { inst.J };
        let offset = sext(inst.full_imm() << 1, 32);
        let pc = offset.wrapping_add(state.pc);
        state.x[inst.rd() as usize] = state.pc + rv32::WORD as u32;
        state.pc = pc - 4;
    }
}

#[derive(Default, Copy, Clone)]
pub struct JALR; // JAL but R type offset encoding
                 // Add imm to rs1 then make it even (LSB = 0)
                 // Set the PC to the contents of rd
                 // Set rd to the old pc + 4
impl Instruction for JALR {
    fn name(&self) -> &'static str {
        "JALR"
    }

    fn match_inst(&self, inst: rv32::Word) -> bool {
        // testing against print 
        println!("JALR: {:032b}", inst);
        println!("JALR: xxxxxxxxxxxxxxxxx000xxxxx1100111");
        match_mask!(inst, "xxxxxxxxxxxxxxxxx000xxxxx1100111")
    }

    fn step(&self, inst: GenInstruction, state: &mut cpu::CPUState) {
        println!("VM > Executing JALR");
        let inst = unsafe { inst.I };
        let offset = sext(inst.full_imm(), 32);
        let pc = offset.wrapping_add(state.x[inst.rs1() as usize]);
        state.x[inst.rd() as usize] = state.pc + rv32::WORD as u32;
        state.pc = pc - 4;
    }
}

#[derive(Default, Copy, Clone)]
pub struct ADDI;

impl Instruction for ADDI {
    fn name(&self) -> &'static str {
        "ADDI"
    }

    fn match_inst(&self, inst: rv32::Word) -> bool {
        match_mask!(inst, "xxxxxxxxxxxxxxxxx000xxxxx0010011")
    }

    fn step(&self, inst: GenInstruction, state: &mut cpu::CPUState) {
        println!("VM > Executing ADDI");
        let inst = unsafe { inst.I };
        state.x[inst.rd() as usize] = state.x[inst.rs1() as usize].wrapping_add(inst.sext_imm())
    }
}

#[derive(Default, Copy, Clone)]
pub struct ADD;

impl Instruction for ADD {
    fn name(&self) -> &'static str {
        "ADD"
    }

    fn match_inst(&self, inst: rv32::Word) -> bool {
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
    LUI(LUI),
    AUIPC(AUIPC),
    JAL(JAL),
    JALR(JALR),
    ADDI(ADDI),
    ADD(ADD),
}
