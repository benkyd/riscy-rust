use crate::ext::decode;
use crate::system::bus::*;
use crate::system::ram;
use crate::system::rv32;
use std::{cell::RefCell, rc::Rc};

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
pub struct CPUState {
    pub x: [rv32::Word; 32],
    pub pc: rv32::Word,
}

pub struct CPU {
    state: CPUState,
    bus: Rc<RefCell<Bus>>,
    instruction_decoder: Rc<RefCell<decode::DecodeCycle>>,
    extensions: Vec<char>,
}

impl CPU {
    pub fn new(
        bus: Rc<RefCell<Bus>>,
        instruction_decoder: Rc<RefCell<decode::DecodeCycle>>,
        extensions: Vec<char>,
    ) -> CPU {
        CPU {
            state: CPUState { x: [0; 32], pc: 0 },
            bus,
            instruction_decoder,
            extensions,
        }
    }

    pub fn init(&mut self) {
        println!("VM RISC-V 32I CPU");
        println!("-----------------");
        println!("VM > Initializing CPU");

        self.state.pc = DRAM_BASE as rv32::Word;
        self.state.x[0] = 0x00000000; // x0 is tied to ground
        self.state.x[2] = ram::DRAM_SIZE as u32; // x2 the addressable
        println!("VM > CPU Initialisd with extensions {:?}", self.extensions);
    }

    pub fn get_pc(&self) -> rv32::Word {
        return self.state.pc;
    }

    fn fetch(&self) -> rv32::Word {
        self.bus.borrow_mut().load_32(self.state.pc)
    }

    pub fn exec(&mut self) -> Result<(), String> {
        while self.state.pc - DRAM_BASE < ram::DRAM_SIZE as u32 {
            // fetch
            let inst = self.fetch();
            println!("VM > Fetched 0x{:08x}: 0x{:08x}", self.state.pc, inst);
            self.state.x[0] = 0x00000000;

            self.instruction_decoder
                .borrow_mut()
                .decode_exec_inst(inst, &mut self.state)?;

            self.dump_reg();
            self.state.pc = self.state.pc + rv32::WORD as u32;
        }
        Ok(())
    }

    fn dump_reg(&mut self) {
        println!("VM > Dumping registers");
        println!("   > PC : 0x{:08x}", self.state.pc);
        for i in 0..8 {
            print!("   > ");
            for j in 0..4 {
                let coord = (i * 4) + j;
                print!("x{0: <2}: 0x{1: <08x} ", coord, self.state.x[coord]);
            }
            println!("");
        }
    }
}
