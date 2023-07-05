use bits::match_mask;
use enum_dispatch::*;
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
    fn match_inst(&self, inst: rv32::Word) -> bool;
    fn step(&self, inst: rv32::Word, state: &mut cpu::CPU);
}

#[derive(Copy, Clone)]
struct ADDI;

impl Instruction for ADDI {
    fn match_inst(&self, inst: rv32::Word) -> bool {
        match_mask!(inst, "xxxxxxxxxxxxxxxxxx000xxxx0010011")
    }

    fn step(&self, inst: rv32::Word, state: &mut cpu::CPU) {
        // self.x[inst.rd() as usize] = self.x[inst.rs1() as usize].wrapping_add(inst.imm() as u32);
    }
}

#[derive(Copy, Clone)]
struct ADD;

impl Instruction for ADD {
    fn match_inst(&self, inst: rv32::Word) -> bool {
        match_mask!(inst, "0000000xxxxxxxxxxx000xxxx0110011")
    }

    fn step(&self, inst: rv32::Word, state: &mut cpu::CPU) {}
}

#[enum_dispatch(Instruction)]
enum ExtensionI {
    ADDI(ADDI),
    ADD(ADD),
}

enum Extensions {
    ExtensionI(Option<ExtensionI>),
}

pub struct DecodeCycle {
    extensions: Vec<Extensions>,
}

impl DecodeCycle {
    pub fn new(&mut self, ext: Vec<char>) {
        for extension in ext {
            match extension {
                'i' => {
                    self.extensions.push(Extensions::ExtensionI(None));
                }
                _ => {
                    println!("VM > Unknown Extension '{}'", extension);
                }
            }
        }
    }

    pub fn decode_inst(&self, inst: rv32::Word) -> Option<Instruction> {
        // we need to go over every instruction and see if it matches
        // we can do smarter things with cacheing later - this aint blazin
        for extension in &self.extensions {
            match extension.match_inst(inst) {
                Some(inst) => return Some(inst),
                None => continue,
            };
        };
        None
    }
}
