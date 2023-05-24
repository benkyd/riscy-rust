use std::fs::File;
use std::io::BufReader;
use std::io::Read;

mod rv32;
mod bus;
mod ram;
mod inst;

use crate::ram::*;
use crate::bus::*;

#[repr(align(8))]
union Instruction {
    inst: rv32::Word,
    r: std::mem::ManuallyDrop<inst::RType>,
    i: std::mem::ManuallyDrop<inst::IType>,
    s: std::mem::ManuallyDrop<inst::SType>,
    b: std::mem::ManuallyDrop<inst::BType>,
    u: std::mem::ManuallyDrop<inst::UType>,
    j: std::mem::ManuallyDrop<inst::JType>,
}

struct VMRV32I {
    // 32 bi bus
    bus: bus::Bus,
    // 32 registers
    x: [rv32::Word; 32],
    // 32-bit program counter
    pc: rv32::Word,
}

impl VMRV32I {
    fn new() -> VMRV32I {
        VMRV32I {
            bus: Bus::new(),
            x: [0; 32],
            pc: 0,
        }
    }

    fn load_prog(&mut self, file: &str) {
        println!("VM > Loading program: {}", file);

        let f = File::open(file).expect("file not found");
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).expect("error reading file");

        println!("VM > Program size: {} bytes", buffer.len());

        // put program at the base of DRAM
        for i in 0..buffer.len() {
            self.bus.memory.0[i] = buffer[i];
        }

        println!("VM > Program loaded to 0x{:08x}", self.pc);

        println!("VM > WORD at 0x80000000: 0x{:04x}", self.bus.memory.read::<rv32::Word>(0x80000000))
    }

    fn init_cpu(&mut self) {
        println!("VM RISC-V 32I CPU");
        println!("-----------------");
        println!("VM > Initializing CPU");

        self.bus = Bus::new();
        self.pc = DRAM_BASE as rv32::Word;
        self.x[0] = 0x00000000; // x0 is tied to ground
        self.x[2] = self.bus.memory.len() as u32; // x2 the addressable space
    }

    fn fetch(&mut self) -> Instruction {
        Instruction { inst: 0xFFFFFFFF }
    }

    fn exec(&mut self) {
        while self.pc > self.bus.memory.len() as u32 {
            let inst = self.fetch();
        }
    }
}

fn main() {
    println!("VM Starting Up");

    let mut cpu = VMRV32I::new();
    cpu.init_cpu();
    cpu.load_prog("./test/add.bin");
    cpu.exec();
}
