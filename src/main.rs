use std::{cell::RefCell, rc::Rc};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

mod bus;
mod cpu;
mod inst;
mod ram;
mod rv32;

use crate::bus::*;
use crate::cpu::*;


struct VMRV32I {
    bus: Rc<RefCell<bus::Bus>>,
    cpu: cpu::CPU,
}

impl VMRV32I {
    fn new() -> VMRV32I {
        let bus = Rc::new(RefCell::new(Bus::new()));
        let mut cpu = CPU::new(Rc::clone(&bus));
        cpu.init();
        VMRV32I { cpu, bus }
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
            self.bus.borrow_mut().store_8(i as u32 + bus::DRAM_BASE, buffer[i]);
        }

        println!("VM > Program loaded to 0x{:08x}", self.cpu.get_pc());
    }

    fn dump_prog(&mut self) {
        println!("VM > Dumping program (virtual addresses)");
        for i in 0..12 {
            println!(
                "VM > 0x{:08x}: 0x{:02x}",
                i,
                self.bus.borrow_mut().load_8(i + bus::DRAM_BASE)
            );
        }
    }

    fn dispatch(&mut self) {
        self.cpu.exec();
    }
}

fn main() {
    println!("VM Starting Up");

    let mut vm = VMRV32I::new();
    vm.load_prog("./test/add.bin");
    vm.dump_prog();
    vm.dispatch();
}
