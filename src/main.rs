#![feature(type_alias_impl_trait)]
#![feature(return_position_impl_trait_in_trait)]

use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::{cell::RefCell, rc::Rc};

mod cpu;
mod err;
mod ext;
mod helpers;
mod management;
mod system;

use crate::cpu::*;
use crate::ext::decode;
use crate::system::bus;

struct VMRV32I {
    bus: Rc<RefCell<bus::Bus>>,
    cpu: cpu::CPU,
    instruction_decoder: Rc<RefCell<decode::DecodeCycle>>,
}

impl VMRV32I {
    fn new() -> VMRV32I {
        let extensions = vec!['i', 'm', 'a', 'z'];

        let bus = Rc::new(RefCell::new(bus::Bus::new()));
        let instruction_decoder =
            Rc::new(RefCell::new(decode::DecodeCycle::new(extensions.clone())));
        let mut cpu = CPU::new(
            Rc::clone(&bus),
            Rc::clone(&instruction_decoder),
            extensions.clone(),
        );

        cpu.init();
        VMRV32I {
            cpu,
            bus,
            instruction_decoder,
        }
    }

    fn load_prog(&mut self, file: &str) -> u32 {
        println!("VM > Loading program: {}", file);

        let f = File::open(file).expect("file not found");
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).expect("error reading file");

        println!("VM > Program size: {} bytes", buffer.len());

        // put program at the base of DRAM
        for i in 0..buffer.len() {
            self.bus
                .borrow_mut()
                .store_8(i as u32 + bus::DRAM_BASE, buffer[i]);
        }

        println!("VM > Program loaded to 0x{:08x}", self.cpu.get_pc());
        buffer.len() as u32
    }

    fn has_load_prog(&self) -> bool {
        self.bus.borrow_mut().load_32(bus::DRAM_BASE) != 0
    }

    fn dump_prog(&mut self, size: u32) {
        println!("VM > Dumping program (virtual addresses)");
        for i in 0..size {
            if i % 4 == 0 {
                println!(
                    "VM > 0x{:08x}: 0x{:08x}",
                    i,
                    self.bus.borrow_mut().load_32(i + bus::DRAM_BASE)
                );
            }
        }
    }

    fn dump_relavent_memory(&self) {
        println!("VM > Dumping relavent memory");
    }

    fn dispatch_step(&mut self) {
        match self.cpu.exec_step() {
            Ok(_) => (),
            Err(e) => println!("VM > Program exited violently with error: {}", e),
        }

        self.cpu.dump_reg();
    }

    fn dispatch(&mut self) {
        match self.cpu.exec() {
            Ok(_) => println!("VM > Program exited peacefully"),
            Err(e) => println!("VM > Program exited violently with error: {}", e),
        }

        self.cpu.dump_reg();

        println!("VM > CPU has stalled");
    }
}

fn main() {
    println!("VM > Loading CPU Management Engine");
    let manager = management::Management::new();
    println!("VM > Starting Up");
    let mut vm = VMRV32I::new();

    let mut should_run = false;
    manager
        .vm_params()
        .iter()
        .for_each(|action| match action.action {
            management::Action::Load => {
                println!("VM > Loading file: {}", action.arg);
                vm.load_prog(&action.arg);
            }
            management::Action::Run => {
                println!("VM > Running program");
                should_run = true;
            }
            _ => (),
        });

    if should_run && vm.has_load_prog() {
        vm.dispatch();
        vm.dump_relavent_memory();
        return;
    } else if should_run {
        println!("VM > CPU has stalled");
        return;
    }

    println!("VM > No program loaded");

    // event loop for interactive mode
    loop {
        let action = manager.prompt();
        match action.action {
            management::Action::Load => {
                println!("VM > Loading file: {}", action.arg);
                vm.load_prog(&action.arg);
            }
            management::Action::Run => {
                println!("VM > Running program");
                if !vm.has_load_prog() {
                    println!("VM > No program loaded");
                    continue;
                }
                vm.dispatch();
            }
            management::Action::Step => {
                println!("VM > Stepping program");
                if !vm.has_load_prog() {
                    println!("VM > No program loaded");
                    continue;
                }
                vm.dispatch_step();
            }
            management::Action::Dump => {
                println!("VM > Dumping program");
                vm.dump_prog(0x100);
            }
            management::Action::Quit => {
                println!("VM > Quitting");
                break;
            }
            _ => (),
        }
    }
}
