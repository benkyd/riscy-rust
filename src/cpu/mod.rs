use std::{cell::RefCell, rc::Rc};
use crate::bus::*;
use crate::ram;
use crate::inst;
use crate::rv32;

// Register ABI         Description             Saver
// x0       zero        Zero                    Immutable
// x1       ra          Return address          Callee
// x2       sp          Stack pointer           Callee
// x3       gp          Global pointer          —
// x4       tp          Thread pointer          —
// x5       t0          Temporary link          Caller
// x6-x7    t1-t2       Temporaries             Caller
// x8       s0 / fp     Saved / frame pointer   Callee
// x9       s1          Saved register          Callee
// x10-x11  a0-a1       Fn args/return values   Caller
// x12-x17  a2-a7       Fn args                 Caller
// x18-x27  s2-s11      Saved registers         Callee
// x28-x31  t3-t6       Temporaries             Caller
pub struct CPU {
    x: [rv32::Word; 32],
    pc: rv32::Word,
    bus: Rc<RefCell<Bus>>,
    // extensions:
}

impl CPU {
    pub fn new(bus: Rc<RefCell<Bus>>) -> CPU {
        CPU {
            x: [0; 32],
            pc: 0,
            bus,
        }
    }

    pub fn init(&mut self) {
        println!("VM RISC-V 32I CPU");
        println!("-----------------");
        println!("VM > Initializing CPU");

        self.pc = DRAM_BASE as rv32::Word;
        self.x[0] = 0x00000000; // x0 is tied to ground
        self.x[2] = ram::DRAM_SIZE as u32; // x2 the addressable
    }

    pub fn get_pc(&mut self) -> rv32::Word {
        return self.pc;
    }

    fn fetch(&mut self) -> inst::Instruction {
        inst::Instruction {
            inst: self.bus.borrow_mut().load_32(self.pc),
        }
    }

    pub fn exec(&mut self) {
        while self.pc - DRAM_BASE < ram::DRAM_SIZE as u32 {
            // fetch
            let inst = self.fetch();
            println!("VM > Fetched 0x{:08x}: 0x{:08x}", self.pc, unsafe {
                inst.inst
            });
            self.pc = self.pc + rv32::WORD as u32;
            self.x[0] = 0x00000000;

            // decode and execute
            // we can use nulltype to extract the opcode
            let opcode = unsafe { inst.null.opcode() };
            // then we can match the opcode to extract the op type
            match opcode {
                inst::I_TYPE => {
                    let inst = unsafe { inst.I };
                    println!("VM > Decoded I Type instruction {:?}", inst);
                    match inst.funct3() {
                        0x0 => {
                            self.x[inst.rd() as usize] =
                                self.x[inst.rs1() as usize].wrapping_add(inst.imm() as u32);
                        }
                        _ => println!("VM > INST {:03b} not implemented", inst.funct3()),
                    };
                }
                inst::R_TYPE => {
                    let inst = unsafe { inst.R };
                    println!("VM > Decoded R Type instruction {:?}", inst);
                    match inst.funct3() {
                        0x0 => {
                            self.x[inst.rd() as usize] = self.x[inst.rs1() as usize]
                                .wrapping_add(self.x[inst.rs2() as usize]);
                        }
                        _ => println!("VM > INST {:03b} not implemented", inst.funct3()),
                    }
                }
                _ => println!("VM > OPCODE {:08b} not implemented", opcode),
            };

            self.dump_reg();
        }
    }

    fn dump_reg(&mut self) {
        println!("VM > Dumping registers");
        println!("PC : 0x{:08x}", self.pc);
        for i in 0..4 {
            for j in 0..8 {
                let coord = (i * 8) + j;
                print!("x{0: <2}: {1: <4}", coord, self.x[coord]);
            }
            println!("");
        }
    }
}
