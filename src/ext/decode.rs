use bits::match_mask;
use enum_dispatch::*;
use strum::{IntoEnumIterator, EnumIter};
use modular_bitfield::prelude::*;

use crate::cpu;
use crate::system::rv32;

// trait Instruction {
//   fn impl_register(&self, exts: &mut Vec<Extension>, name: &'static str) {
//     for ext in exts {
//       if ext.name == name {
//         exts.add(self)
//       }
//     }
//   }
// }
//
// fn main() {
//   let mut extensions = vec![ext1, ext2, ...];
//   ADDI.register(&extensions);
//   SUBI.register(&extensions);
// }
//
// // ...
//
// impl Instruction for ADDI {
//   fn register(&self, exts: &mut Vec<Extension>) {
//      self.impl_register(exts, "instr_set")
//   }
// }

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

enum EncodingType {
    R(RType),
    I(IType),
}

#[repr(align(8))]
pub union GenInstruction {
    pub inst: rv32::Word,
    pub null: std::mem::ManuallyDrop<NullType>,
    pub R: std::mem::ManuallyDrop<RType>,
    pub I: std::mem::ManuallyDrop<IType>,
}

#[enum_dispatch]
trait Instruction {
    fn name(&self) -> &'static str;
    fn match_inst(&self, inst: rv32::Word) -> bool;
    fn step(&self, inst: rv32::Word, state: &mut cpu::CPUState);
}

#[derive(Default, Copy, Clone)]
struct ADDI;

impl Instruction for ADDI {
    fn name(&self) -> &'static str {
        "ADDI"
    }

    fn match_inst(&self, inst: rv32::Word) -> bool {
        println!("VM > Checking ADDI");
        println!("VM > ADDI: 0b{:032b}", inst);
        println!("VM > ADDI: 0bxxxxxxxxxxxxxxxxxx000xxxx0010011");
        match_mask!(inst, "xxxxxxxxxxxxxxxxxx000xxxx0010011")
    }

    fn step(&self, inst: rv32::Word, state: &mut cpu::CPUState) {
        println!("VM > Decoded I Type instruction 0x{:08x}", inst);
        println!("VM > Executing ADDI");
        // self.x[inst.rd() as usize] = self.x[inst.rs1() as usize].wrapping_add(inst.imm() as u32);
    }
}

#[derive(Default, Copy, Clone)]
struct ADD;

impl Instruction for ADD {
    fn name(&self) -> &'static str {
        "ADD"
    }

    fn match_inst(&self, inst: rv32::Word) -> bool {
        println!("VM > Checking ADD");
        println!("VM > ADD: 0b{:032b}", inst);
        println!("VM > ADD: 0b0000000xxxxxxxxxxx000xxxx0110011");
        match_mask!(inst, "0000000xxxxxxxxxxx000xxxx0110011")
    }

    fn step(&self, inst: rv32::Word, state: &mut cpu::CPUState) {
        println!("VM > Decoded R Type instruction 0x{:08x}", inst);
        println!("VM > Executing ADD");
    }
}

#[enum_dispatch(Instruction)]
#[derive(EnumIter)]
enum ExtensionI {
    ADDI(ADDI),
    ADD(ADD),
}

enum Extensions {
    ExtensionI(Option<ExtensionI>),
}

pub struct DecodeCycle {
    extensions: Vec<char>,
}

impl DecodeCycle {
    pub fn new(ext: Vec<char>) -> DecodeCycle {
        DecodeCycle { extensions: ext }
    }

    pub fn decode_exec_inst(&self, inst: rv32::Word, state: &mut cpu::CPUState) -> Result<(), String> {
        // we want to go through each extension and then go through each instruction in that extension
        // if we find a match, we want to execute it
        // if we don't find a match, we want to return an error

        for extension in self.extensions.iter() {
            match extension {
                'i' => {
                    println!("VM > Attempting to decode instruction as I extension: 0x{:08x}", inst);
                    for instruction in ExtensionI::iter() {
                        println!("VM > Checking instruction: {:?}", instruction.name());
                        if instruction.match_inst(inst) {
                            println!("VM > Decoded instruction as I extension: 0x{:08x}", inst);
                            instruction.step(inst, state);
                            return Ok(());
                        }
                    }
                }
                _ => {
                    println!("VM > Unknown Extension");
                }
            }
        }
        Ok(())
   }
}
