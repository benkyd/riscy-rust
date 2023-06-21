use modular_bitfield::prelude::*;
use bits::match_mask;

use crate::system::rv32;
use crate::cpu;

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

trait Instruction {
    fn register(&self, ext: &mut Extension) where Self: Sized {
        // ext.register(Box::new(self));
    }
    fn decode(&self);
    fn match_inst(&self, inst: rv32::Word) -> bool;
    fn step(&self, inst: rv32::Word, state: &mut cpu::CPU);
}

#[derive(Copy, Clone)]
struct ADDI;
impl ADDI {
    fn new() -> ADDI {
        ADDI
    }
}
impl Instruction for ADDI  {
    fn decode(&self) {

    }

    fn match_inst(&self, inst: rv32::Word) -> bool {
        match_mask!(inst, "xxxxxxxxxxxxxxxxxx000xxxx0010011")
    }

    fn step(&self, inst: rv32::Word, state: &mut cpu::CPU) {
    }
}


trait GenExtension {
    fn register(&mut self, inst: Box<dyn Instruction>) {
    }
}

struct Extension {
    instruction_set: Vec<Box<dyn Instruction>>,
}

impl GenExtension for Extension {

}

impl Extension {
    fn new() -> Extension {
        Extension {
            instruction_set: Vec::new(),
        }
    }
}

struct ExtensionI;
impl GenExtension for ExtensionI {

}

pub fn load_extensions(ext: Vec<char>) {
    for extension in ext {

    }
}

// pub fn decode_inst(inst: rv32::Word) -> fn() {
//     // we need to go over every instruction and see if it matches
//     // we can do smarter things with cacheing later - this aint blazin
// }
