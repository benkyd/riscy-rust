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
    pub trap: rv32::Word,
    pub bus: Rc<RefCell<Bus>>,
    // for simplicities sake we will just put CSRs here, in the CPU.
    // in reality they are part of the system and should be in the system module
    //
    // We will also manage them outside of the extensions interface, as they are
    // not really extensions, but part of the base spec.
    //
    // We will also not implement all of them, just the ones we need for the Linux
    // TODO: Do all of the CSRs exist on each hart? Or are they per hart?
    mstatus: rv32::Word, // Machine status reg to disable interrupts

    // Timers
    cyclel: rv32::Word,   // Lower 32 bits of the cycle counter
    cycleh: rv32::Word,   // Upper 32 bits of the cycle counter
    timel: rv32::Word,    // Lower 32 bits of the timer
    timeh: rv32::Word,    // Upper 32 bits of the timer
    timecmpl: rv32::Word, // Lower 32 bits of the timer compare register
    timecmph: rv32::Word, // Upper 32 bits of the timer compare register

    // Machine Information Registers
    mvendorid: rv32::Word, // Vendor ID of the hart
    marchid: rv32::Word,   // Architecture ID of the hart
    mimpid: rv32::Word,    // Implementation ID of the hart
    mhartid: rv32::Word,   // Hardware thread ID of the hart

    // Machine Trap Stuffs
    mscratch: rv32::Word, // Scratch register for machine trap handlers
    mtvec: rv32::Word,    // Address of trap handler
    mie: rv32::Word,      // Machine interrupt enable
    mip: rv32::Word,      // Machine interrupt pending

    mepc: rv32::Word,   // Machine exception program counter
    mtval: rv32::Word,  // Machine trap value
    mcause: rv32::Word, // Machine trap cause
}

pub struct CPU {
    state: CPUState,
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
            state: CPUState {
                x: [0; 32],
                pc: 0,
                trap: 0,
                bus,
                mstatus: 0,
                cyclel: 0,
                cycleh: 0,
                timel: 0,
                timeh: 0,
                timecmpl: 0,
                timecmph: 0,
                mvendorid: 0,
                marchid: 0,
                mimpid: 0,
                mhartid: 0,
                mscratch: 0,
                mtvec: 0,
                mie: 0,
                mip: 0,
                mepc: 0,
                mtval: 0,
                mcause: 0,
            },
            instruction_decoder,
            extensions,
        }
    }

    pub fn init(&mut self) {
        println!("-----------------");
        println!("VM RISC-V 32I CPU");
        println!("-----------------");
        println!("VM > Initializing CPU");

        self.state.pc = DRAM_BASE as rv32::Word;
        self.state.x[0] = 0x00000000; // x0 is tied to ground
        self.state.x[2] = DRAM_BASE + ram::DRAM_SIZE as u32; // x2 the stack pointer
        println!("VM > CPU Initialisd with extensions {:?}", self.extensions);
        self.dump_reg();
    }

    pub fn get_pc(&self) -> rv32::Word {
        return self.state.pc;
    }

    fn fetch(&self) -> rv32::Word {
        self.state.bus.borrow_mut().load_32(self.state.pc)
    }

    pub fn step(&mut self) -> Result<(), String> {
        // CSR stuff before fetch execute
        //

        // TODO: We can execute multiple instructions per cycle
        let inst = self.fetch();
        println!("VM > Fetched 0x{:08x}: 0x{:08x}", self.state.pc, inst);
        self.state.x[0] = 0x00000000;

        self.state.pc = self.state.pc + rv32::WORD as u32;

        self.instruction_decoder
            .borrow_mut()
            .decode_exec_inst(inst, &mut self.state)?;

        Ok(())
    }

    pub fn exec(&mut self) -> Result<(), String> {
        while self.state.pc - DRAM_BASE < ram::DRAM_SIZE as u32 {
            self.step()?;
            self.dump_reg();
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
