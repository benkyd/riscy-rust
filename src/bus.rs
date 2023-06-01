pub const DRAM_BASE: u32 = 0x80000000;

use crate::rv32;
use crate::ram;

pub struct Bus {
    memory: ram::RAM,
}

impl Bus {
    pub fn new() -> Bus {
        Bus { memory: ram::RAM::new() }
    }

    pub fn load_8(&mut self, address: rv32::XLen) -> rv32::Byte {
        match address {
            DRAM_BASE.. => {
                self.memory.read_8(address)
            }
            _ => {
                println!("VM > BUS > Peripheral at 0x{:08x} does not exist", address);
                rv32::Byte::default()
            },
        }
    }

    pub fn load_16(&mut self, address: rv32::XLen) -> rv32::HalfWord {
        match address {
            DRAM_BASE.. => {
                self.memory.read_16(address)
            }
            _ => {
                println!("VM > BUS > Peripheral at 0x{:08x} does not exist", address);
                rv32::HalfWord::default()
            },
        }
    }

    pub fn load_32(&mut self, address: rv32::XLen) -> rv32::Word {
        match address {
            DRAM_BASE.. => {
                self.memory.read_32(address)
            }
            _ => {
                println!("VM > BUS > Peripheral at 0x{:08x} does not exist", address);
                rv32::Word::default()
            },
        }
    }

    pub fn load_64(&mut self, address: rv32::XLen) -> rv32::DoubleWord {
        match address {
            DRAM_BASE.. => {
                self.memory.read_64(address)
            }
            _ => {
                println!("VM > BUS > Peripheral at 0x{:08x} does not exist", address);
                rv32::DoubleWord::default()
            },
        }
    }

    pub fn store_8(&mut self, address: rv32::XLen, data: rv32::Byte) {
        match address {
            DRAM_BASE.. => {
                self.memory.write_8(address, data)
            },
            _ => {
                println!("VM > BUS > Peripheral at 0x{:08x} does not exist", address);
            },
        }
    }


    pub fn store_16(&mut self, address: rv32::XLen, data: rv32::HalfWord) {
        match address {
            _ => {
                println!("VM > BUS > Peripheral at 0x{:08x} does not exist", address);
            },
        }
    }


    pub fn store_32(&mut self, address: rv32::XLen, data: rv32::Word) {
        match address {
            _ => {
                println!("VM > BUS > Peripheral at 0x{:08x} does not exist", address);
            },
        }
    }


    pub fn store_64(&mut self, address: rv32::XLen, data: rv32::DoubleWord) {
        match address {
            _ => {
                println!("VM > BUS > Peripheral at 0x{:08x} does not exist", address);
            },
        }
    }
}
