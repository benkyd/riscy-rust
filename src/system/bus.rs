pub const DRAM_BASE: u32 = 0x80000000;
pub const DRAM_SIZE: u32 = 1 * 1024 * 1024 * 1024; // 1GBram
pub const DRAM_TOP: u32 = DRAM_BASE + DRAM_SIZE;

pub const UART_BASE: u32 = 0x10000000;
pub const UART_SIZE: u32 = 0x100;
pub const UART_TOP: u32 = UART_BASE + UART_SIZE;

use crate::system::ram;
use crate::system::rv32;
use crate::system::uart;

pub struct Bus {
    memory: ram::RAM,
    uart: uart::UART,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            memory: ram::RAM::new(),
            uart: uart::UART::new(),
        }
    }

    pub fn load_8(&mut self, address: rv32::XLen) -> rv32::Byte {
        match address {
            DRAM_BASE..=DRAM_TOP => self.memory.read_8(address),
            _ => {
                panic!("VM > BUS > Peripheral at 0x{:08x} does not exist", address);
            }
        }
    }

    pub fn load_16(&mut self, address: rv32::XLen) -> rv32::HalfWord {
        match address {
            DRAM_BASE..=DRAM_TOP => self.memory.read_16(address),
            _ => {
                panic!("VM > BUS > Peripheral at 0x{:08x} does not exist", address);
            }
        }
    }

    pub fn load_32(&mut self, address: rv32::XLen) -> rv32::Word {
        match address {
            UART_BASE..=UART_TOP => self.uart.read_kb(address),
            DRAM_BASE..=DRAM_TOP => self.memory.read_32(address),
            _ => {
                panic!("VM > BUS > Peripheral at 0x{:08x} does not exist", address);
            }
        }
    }

    pub fn load_64(&mut self, address: rv32::XLen) -> rv32::DoubleWord {
        match address {
            DRAM_BASE..=DRAM_TOP => self.memory.read_64(address),
            _ => {
                panic!("VM > BUS > Peripheral at 0x{:08x} does not exist", address);
            }
        }
    }

    pub fn store_8(&mut self, address: rv32::XLen, data: rv32::Byte) {
        match address {
            DRAM_BASE..=DRAM_TOP => self.memory.write_8(address, data),
            _ => {
                panic!("VM > BUS > Peripheral at 0x{:08x} does not exist", address);
            }
        }
    }

    pub fn store_16(&mut self, address: rv32::XLen, data: rv32::HalfWord) {
        match address {
            DRAM_BASE..=DRAM_TOP => self.memory.write_16(address, data),
            _ => {
                panic!("VM > BUS > Peripheral at 0x{:08x} does not exist", address);
            }
        }
    }

    pub fn store_32(&mut self, address: rv32::XLen, data: rv32::Word) {
        match address {
            UART_BASE..=UART_TOP => self.uart.write(address, data),
            DRAM_BASE..=DRAM_TOP => self.memory.write_32(address, data),
            _ => {
                panic!("VM > BUS > Peripheral at 0x{:08x} does not exist", address);
            }
        }
    }

    pub fn store_64(&mut self, address: rv32::XLen, data: rv32::DoubleWord) {
        match address {
            DRAM_BASE..=DRAM_TOP => self.memory.write_64(address, data),
            _ => {
                panic!("VM > BUS > Peripheral at 0x{:08x} does not exist", address);
            }
        }
    }
}
