use bits::match_mask;

use crate::system::rv32;
use crate::cpu;

trait Instruction {
    fn match_inst(inst: rv32::Word) -> bool;
    fn step(&self, inst: rv32::Word, state: &mut cpu::CPU);
}

struct ADDI;
impl Instruction for ADDI  {
    fn match_inst(inst: rv32::Word) -> bool {
        match_mask!(inst, "xxxxxxxxxxxxxxxxxx000xxxx0010011")
    }

    fn step(&self, inst: rv32::Word, state: &mut cpu::CPU) {
    }
}


#[derive(Clone, Copy)]
enum I {
    ADDI(ADDI),
}

#[derive(Clone, Copy)]
enum Extensions {
    I,
}

pub fn decode_inst(inst: rv32::Word) -> fn() {
    // we need to go over every instruction and see if it matches
    // we can do smarter things with cacheing later - this aint blazin
}
