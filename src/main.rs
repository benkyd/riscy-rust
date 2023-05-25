#![feature(unchecked_math)]
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

mod rv32;
mod bus;
mod ram;
mod inst;

use crate::bus::*;

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
    }

    fn dump_prog(&mut self) {
        println!("VM > Dumping program (virtual addresses)");
        for i in 0..12 {
            println!("VM > 0x{:08x}: 0x{:02x}", i, self.bus.memory.0[i]);
        }
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

    fn fetch(&mut self) -> inst::Instruction {
        inst::Instruction { inst: 0xFFFFFFFF }
    }

    fn exec(&mut self) {
        let val: u8 = self.bus.memory.read_8(0x80000000);
        println!("VM > BYTE  at 0x80000000: 0x{:02x}", val);
        let val = self.bus.memory.read_16(0x80000000);
        println!("VM > HWORD at 0x80000000: 0x{:04x}", val);
        let val = self.bus.memory.read_32(0x80000000);
        println!("VM > WORD  at 0x80000000: 0x{:08x}", val);
        let val = self.bus.memory.read_64(0x80000000);
        println!("VM > DWORD at 0x80000000: 0x{:016x}", val);

        let val = self.bus.load_8(0x7FFFFFFF);
        println!("VM > BYTE  at 0x80000000: 0x{:02x}", val);

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
    cpu.dump_prog();
    cpu.exec();
}
