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
pub struct BRANCH; // This is is the first time we write a catchall
                   // instruction, this will match BEQ, BNE, BLT,
                   // BGE, BLTU, BEGE
impl Instruction for BRANCH {
    fn name(&self) -> &'static str {
        "BEQ, BNE, BLT, BGE, BLTU, BGEU"
    }

    fn match_inst(&self, inst: rv32::Word) -> bool {
        match_mask!(inst, "xxxxxxxxxxxxxxxxxxxxxxxxx1100011")
    }

    fn step(&self, inst: GenInstruction, state: &mut cpu::CPUState) {
        println!("VM > Executing BEQ, BNE, BLT, BGE, BLTU, BGEU");
        let inst = unsafe { inst.B };
        let offset = state.pc + (inst.sext_imm() << 1) - 4;
        match inst.funct3() {
            0b000 => {
                // beq
                if inst.rs1() == inst.rs2() {
                    state.pc = offset
                }
            }
            0b001 => {
                // bne
                if inst.rs1() != inst.rs2() {
                    state.pc = offset
                }
            }
            0b100 => {
                // blt
                if inst.rs1() < inst.rs2() {
                    state.pc = offset
                }
            }
            0b101 => {
                // bge
                if inst.rs1() >= inst.rs2() {
                    state.pc = offset
                }
            }
            0b110 => {
                // bltu
                if (inst.rs1() as u32) < (inst.rs2() as u32) {
                    state.pc = offset
                }
            }
            0b111 => {
                // bgeu
                if (inst.rs1() as u32) >= (inst.rs2() as u32) {
                    state.pc = offset
                }
            }
            _ => state.trap = 3,
        }
    }
}

#[derive(Default, Copy, Clone)]
pub struct LOAD; // Another catchall instruction, this will match
                 // LB, LH, LW, LBU, LHU
impl Instruction for LOAD {
    fn name(&self) -> &'static str {
        "LOAD"
    }

    fn match_inst(&self, inst: rv32::Word) -> bool {
        match_mask!(inst, "xxxxxxxxxxxxxxxxxxxxxxxxx0000011")
    }

    fn step(&self, inst: GenInstruction, state: &mut cpu::CPUState) {
        println!("VM > Executing LOAD");
        let inst = unsafe { inst.I };
        let offset = inst.sext_imm();
        let addr = state.x[inst.rs1() as usize].wrapping_add(offset);
        match inst.funct3() {
            0b000 => {
                // lb
                state.x[inst.rd() as usize] =
                    state.bus.borrow_mut().load_8(addr) as i8 as i32 as u32
            }
            0b001 => {
                // lh
                state.x[inst.rd() as usize] =
                    state.bus.borrow_mut().load_16(addr) as i16 as i32 as u32
            }
            0b010 => {
                // lw
                state.x[inst.rd() as usize] = state.bus.borrow_mut().load_32(addr) as i32 as u32
            }
            0b100 => state.x[inst.rd() as usize] = state.bus.borrow_mut().load_8(addr) as u32, // lbu
            0b101 => state.x[inst.rd() as usize] = state.bus.borrow_mut().load_16(addr) as u32, // lhu
            _ => state.trap = 3,
        }
    }
}

#[derive(Default, Copy, Clone)]
pub struct STORE;

impl Instruction for STORE {
    fn name(&self) -> &'static str {
        "STORE"
    }

    fn match_inst(&self, inst: rv32::Word) -> bool {
        match_mask!(inst, "xxxxxxxxxxxxxxxxxxxxxxxxx0100011")
    }

    fn step(&self, inst: GenInstruction, state: &mut cpu::CPUState) {
        println!("VM > Executing STORE");
        let inst = unsafe { inst.S };
        let offset = inst.sext_imm();
        let addr = state.x[inst.rs1() as usize].wrapping_add(offset);
        match inst.funct3() {
            0b000 => state // sb
                .bus
                .borrow_mut()
                .store_8(addr, state.x[inst.rs2() as usize] as u8),
            0b001 => state // sh
                .bus
                .borrow_mut()
                .store_16(addr, state.x[inst.rs2() as usize] as u16),
            0b010 => state // sw
                .bus
                .borrow_mut()
                .store_32(addr, state.x[inst.rs2() as usize] as u32),
            _ => state.trap = 3,
        }
    }
}

#[derive(Default, Copy, Clone)]
pub struct IMM;

impl Instruction for IMM {
    fn name(&self) -> &'static str {
        "ADDI, SLTI, SLTIU, XORI, ORI, ANDI"
    }

    fn match_inst(&self, inst: rv32::Word) -> bool {
        if match_mask!(inst, "xxxxxxxxxxxxxxxxx000xxxxx0010011") {
            return true;
        }
        if match_mask!(inst, "xxxxxxxxxxxxxxxxx010xxxxx0010011") {
            return true;
        }
        if match_mask!(inst, "xxxxxxxxxxxxxxxxx011xxxxx0010011") {
            return true;
        }
        if match_mask!(inst, "xxxxxxxxxxxxxxxxx100xxxxx0010011") {
            return true;
        }
        if match_mask!(inst, "xxxxxxxxxxxxxxxxx110xxxxx0010011") {
            return true;
        }
        if match_mask!(inst, "xxxxxxxxxxxxxxxxx111xxxxx0010011") {
            return true;
        }
        false
    }

    fn step(&self, inst: GenInstruction, state: &mut cpu::CPUState) {
        println!("VM > Executing ADDI, SLTI, SLTIU, XORI, ORI, ANDI");
        let inst = unsafe { inst.I };
        let mut retval = 0;
        let rs1 = state.x[inst.rs1() as usize];

        match inst.funct3() {
            0b000 => retval = rs1 + inst.sext_imm(), // addi
            0b010 => retval = ((rs1 as i32) < (inst.sext_imm() as i32)) as u32, // slti
            0b011 => retval = ((rs1 as u32) < (inst.sext_imm() as u32)) as u32, // sltiu
            0b100 => retval = rs1 ^ inst.sext_imm(), // xori
            0b110 => retval = rs1 | inst.sext_imm(), // ori
            0b111 => retval = rs1 & inst.sext_imm(), // andi
            _ => state.trap = 3,
        }

        state.x[inst.rd() as usize] = retval;
    }
}

#[derive(Default, Copy, Clone)]
pub struct SHIFTI; // Compound instruction for SLLI, SRLI and SRAI
                   // These aren't actually encoded as R type but
                   // it makes it more convenient to extract the shamt
impl Instruction for SHIFTI {
    fn name(&self) -> &'static str {
        "SLLI, SRLI, SRAI"
    }

    fn match_inst(&self, inst: rv32::Word) -> bool {
        if match_mask!(inst, "0x00000xxxxxxxxxx001xxxxx0010011") {
            return true;
        }
        if match_mask!(inst, "0x00000xxxxxxxxxx101xxxxx0010011") {
            return true;
        }
        false
    }

    fn step(&self, inst: GenInstruction, state: &mut cpu::CPUState) {
        println!("VM > Executing SLLI, SRLI, SRAI");
        let inst = unsafe { inst.R }; // fun7 is the L/A selector
                                      // rs2 is shamt
        let mut retval = 0;
        let shamt = inst.rs2();
        let rs1 = state.x[inst.rs1() as usize];

        match inst.funct3() {
            0b001 => retval = rs1 << shamt, //slli
            0b101 => match inst.funct7() {
                0b0000000 => retval = rs1 >> shamt,                   // srli
                0b0100000 => retval = ((rs1 as i32) >> shamt) as u32, // srai
                _ => state.trap = 3,
            },
            _ => state.trap = 3,
        }

        state.x[inst.rd() as usize] = retval;
    }
}

#[derive(Default, Copy, Clone)]
pub struct OP; // generalised instruction for all operations
               // including add, sub, sli, slt, sltu, xor, srl
               // sra, or, and
impl Instruction for OP {
    fn name(&self) -> &'static str {
        "ADD, SUB, SLI, SLT, SLTU, XOR, SRL, SRA, OR, AND"
    }

    fn match_inst(&self, inst: rv32::Word) -> bool {
        match_mask!(inst, "0x00000xxxxxxxxxxxxxxxxxx0110011")
    }

    fn step(&self, inst: GenInstruction, state: &mut cpu::CPUState) {
        println!("VM > Executing ADD, SUB, SLI, SLT, SLTU, XOR, SRL, SRA, OR, AND");
        let inst = unsafe { inst.R };
        let mut retval = 0;
        let rs1 = state.x[inst.rs1() as usize];
        let rs2 = state.x[inst.rs2() as usize];

        match inst.funct3() {
            0b000 => match inst.funct7() {
                0b0000000 => retval = rs1 + rs2, // add
                0b0100000 => retval = rs1 - rs2, // sub
                _ => state.trap = 3,
            },
            0b001 => retval = rs1 << (rs2 & 0x1F), // sll
            0b010 => retval = ((rs1 as i32) < (rs2 as i32)) as u32, // slt
            0b011 => retval = ((rs1 as u32) < (rs2 as u32)) as u32, // sltu
            0b100 => retval = rs1 ^ rs2, // xor
            0b101 => match inst.funct7() {
                0b0000000 => retval = rs1 >> (rs2 & 0x1F), // srl
                0b0100000 => retval = ((rs1 as i32) >> (rs2 & 0x1F)) as u32, // sra
                _ => state.trap = 3,
            },
            0b110 => retval = rs1 | rs2, // or
            0b111 => retval = rs1 & rs2, // and
            _ => state.trap = 3,
        }

        state.x[inst.rd() as usize] = retval;
    }
}

#[derive(EnumIter)]
#[enum_dispatch(Instruction)]
pub enum ExtensionI {
    LUI(LUI),
    AUIPC(AUIPC),
    JAL(JAL),
    JALR(JALR),
    BRANCH(BRANCH),
    LOAD(LOAD),
    STORE(STORE),
    IMM(IMM),
    SHIFTI(SHIFTI),
    OP(OP),
}
