use std::time::{SystemTime, UNIX_EPOCH};

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
    pub mstatus: rv32::Word, // Machine status reg to disable interrupts

    // Timers
    pub cyclel: rv32::Word,   // Lower 32 bits of the cycle counter
    pub cycleh: rv32::Word,   // Upper 32 bits of the cycle counter
    pub timel: rv32::Word,    // Lower 32 bits of the timer
    pub timeh: rv32::Word,    // Upper 32 bits of the timer
    pub timecmpl: rv32::Word, // Lower 32 bits of the timer compare register
    pub timecmph: rv32::Word, // Upper 32 bits of the timer compare register

    // Machine Information Registers
    pub mvendorid: rv32::Word, // Vendor ID of the hart
    pub marchid: rv32::Word,   // Architecture ID of the hart
    pub mimpid: rv32::Word,    // Implementation ID of the hart
    pub mhartid: rv32::Word,   // Hardware thread ID of the hart

    // Machine Trap Stuffs
    pub mscratch: rv32::Word, // Scratch register for machine trap handlers
    pub mtvec: rv32::Word,    // Address of trap handler
    pub mie: rv32::Word,      // Machine interrupt enable
    pub mip: rv32::Word,      // Machine interrupt pending

    pub mepc: rv32::Word,   // Machine exception program counter
    pub mtval: rv32::Word,  // Machine trap value
    pub mcause: rv32::Word, // Machine trap cause

    // Note: only a few bits are used.  (Machine = 3, User = 0)
    // Bits 0..1 = privilege.
    // Bit 2 = WFI (Wait for interrupt)
    // Bit 3+ = Load/Store reservation LSBs.
    pub extraflags: rv32::Word,
}

pub struct CPU {
    state: CPUState,
    instruction_decoder: Rc<RefCell<decode::DecodeCycle>>,
    extensions: Vec<char>,
    last_it_time: u128,
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
                extraflags: 0,
            },
            instruction_decoder,
            extensions,
            last_it_time: 0,
        }
    }

    pub fn init(&mut self) {
        println!("-----------------");
        println!("VM RISC-V 32I CPU");
        println!("-----------------");
        println!("VM > Initializing CPU");

        self.last_it_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_micros();

        self.state.pc = DRAM_BASE as rv32::Word;
        self.state.x[0] = 0x00000000; // x0 is tied to ground
        self.state.x[2] = DRAM_BASE + DRAM_SIZE as u32; // x2 the stack pointer
        self.state.mvendorid = 0x696969; // Vendor ID of the hart
        self.state.marchid = 0x285700; // Architecture ID of the hart
        self.state.mimpid = 0; // Implementation ID of the hart
        self.state.mhartid = 0; // Hardware thread ID of the hart

        println!("VM > CPU Initialisd with extensions {:?}", self.extensions);
        self.dump_reg();
    }

    pub fn get_pc(&self) -> rv32::Word {
        return self.state.pc;
    }

    fn fetch(&self) -> rv32::Word {
        self.state.bus.borrow_mut().load_32(self.state.pc)
    }

    pub fn step(&mut self, elapsed_micros: u128) -> Result<(), String> {
        // CSR stuff before fetch execute
        let new_timer = ((self.state.timel as u128) + elapsed_micros) as rv32::Word;
        if new_timer > self.state.timel {
            self.state.timeh += 1;
        }
        self.state.timel = new_timer;

        // handle time interrupt
        if self.state.timel >= self.state.timecmpl
            && self.state.timeh <= self.state.timecmph
            && (self.state.timecmpl, self.state.timecmph) != (0, 0)
        {
            self.state.extraflags &= !4;
            self.state.mip |= 1 << 7; // https://stackoverflow.com/a/61916199/2926815  Fire interrupt.
        } else {
            self.state.mip &= !(1 << 7);
        }

        // if WFI is set, we exit early
        if self.state.extraflags & 4 != 0 {
            return Ok(());
        }

        let mut trap: rv32::Word = 0;
        let mut rval: rv32::Word = 0;
        let cycle: rv32::Word = self.state.cyclel;

        if (self.state.mip & (1 << 7) != 0)
            && (self.state.mie & (1 << 7) != 0/*mtie*/)
            && (self.state.mstatus & 0x8 /*mie*/ != 0)
        {
            // stall
            trap = 0x80000007;
            self.state.pc = self.state.pc - rv32::WORD as u32;
        } else {
            // TODO: We can execute multiple instructions per cycle

            // fetch
            let inst = self.fetch();
            println!("VM > Fetched 0x{:08x}: 0x{:08x}", self.state.pc, inst);
            self.state.x[0] = 0x00000000;

            // decode and execute
            self.instruction_decoder
                .borrow_mut()
                .decode_exec_inst(inst, &mut self.state)?;

            self.state.pc = self.state.pc + rv32::WORD as u32;
        }

        // handle trap and interrupt
        if trap != 0 {
            if trap & 0x80000000 != 0 {
                // interrupt
                self.state.mcause = trap;
                self.state.mtval = 0;
                self.state.pc = self.state.pc + rv32::WORD as u32;
            } else {
                // trap
                self.state.mcause = trap - 1;
                self.state.mtval &= if trap > 5 && trap <= 8 {
                    rval
                } else {
                    self.state.pc
                };
            }
            self.state.mepc = self.state.pc; // the kernel may advance mepc on it's own
                                             // on an interrupt, the system will move MIE into MPIE
            self.state.mstatus =
                (((self.state.mstatus) & 0x08) << 4) | (((self.state.extraflags) & 3) << 11);
            self.state.pc = self.state.mtvec - rv32::WORD as u32;
            // enter machine mode
            self.state.extraflags |= 3;
            self.state.pc = self.state.pc + rv32::WORD as u32;
        }

        if self.state.cyclel > cycle {
            self.state.cycleh += 1;
        }
        self.state.cyclel = cycle;

        Ok(())
    }

    pub fn exec_step(&mut self) -> Result<(), String> {
        let since_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let us = since_epoch.as_micros();
        let elapsed = us - self.last_it_time;
        self.last_it_time += elapsed;

        self.step(elapsed)?;

        Ok(())
    }

    pub fn exec(&mut self) -> Result<(), String> {
        while self.state.pc - DRAM_BASE < DRAM_SIZE as u32 {
            let since_epoch = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
            let us = since_epoch.as_micros();
            let elapsed = us - self.last_it_time;
            self.last_it_time += elapsed;

            self.step(elapsed)?;
        }
        Ok(())
    }

    pub fn dump_reg(&mut self) {
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
