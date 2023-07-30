use crate::system::bus;
use crate::system::rv32;

pub struct RAM(pub Vec<rv32::Byte>);

impl RAM {
    pub fn new() -> RAM {
        println!("VM > Initialised RAM with size: {} bytes", bus::DRAM_SIZE);
        RAM(vec![0; bus::DRAM_SIZE as usize])
    }

    pub fn len(&mut self) -> usize {
        self.0.len()
    }

    pub fn read_8(&mut self, address: rv32::XLen) -> rv32::Byte {
        let memory = &self.0;
        let address = (address - bus::DRAM_BASE) as usize;
        memory[address]
    }

    pub fn read_16(&mut self, address: rv32::XLen) -> rv32::HalfWord {
        let memory = &self.0;
        let address = (address - bus::DRAM_BASE) as usize;
        let ret: rv32::HalfWord =
            memory[address] as rv32::HalfWord | (memory[address + 1] as rv32::HalfWord) << 8;
        ret
    }

    pub fn read_32(&mut self, address: rv32::XLen) -> rv32::Word {
        let memory = &self.0;
        let address = (address - bus::DRAM_BASE) as usize;
        let ret: rv32::Word = memory[address] as rv32::Word
            | (memory[address + 1] as rv32::Word) << 8
            | (memory[address + 2] as rv32::Word) << 16
            | (memory[address + 3] as rv32::Word) << 24;
        ret
    }

    pub fn read_64(&mut self, address: rv32::XLen) -> rv32::DoubleWord {
        let memory = &self.0;
        let address = (address - bus::DRAM_BASE) as usize;
        let ret: rv32::DoubleWord = memory[address] as rv32::DoubleWord
            | (memory[address + 1] as rv32::DoubleWord) << 8
            | (memory[address + 2] as rv32::DoubleWord) << 16
            | (memory[address + 3] as rv32::DoubleWord) << 24
            | (memory[address + 4] as rv32::DoubleWord) << 32
            | (memory[address + 5] as rv32::DoubleWord) << 40
            | (memory[address + 6] as rv32::DoubleWord) << 48
            | (memory[address + 7] as rv32::DoubleWord) << 56;
        ret
    }

    pub fn write_8(&mut self, address: rv32::XLen, data: rv32::Byte) {
        let memory = &mut self.0;
        let address = (address - bus::DRAM_BASE) as usize;
        memory[address] = data;
    }

    pub fn write_16(&mut self, address: rv32::XLen, data: rv32::HalfWord) {
        let memory = &mut self.0;
        let address = (address - bus::DRAM_BASE) as usize;
        memory[address] = (data & 0xFF) as rv32::Byte;
        memory[address + 1] = ((data >> 8) & 0xFF) as rv32::Byte;
    }

    pub fn write_32(&mut self, address: rv32::XLen, data: rv32::Word) {
        let memory = &mut self.0;
        let address = (address - bus::DRAM_BASE) as usize;
        memory[address] = (data & 0xFF) as rv32::Byte;
        memory[address + 1] = ((data >> 8) & 0xFF) as rv32::Byte;
        memory[address + 2] = ((data >> 16) & 0xFF) as rv32::Byte;
        memory[address + 3] = ((data >> 24) & 0xFF) as rv32::Byte;
    }

    pub fn write_64(&mut self, address: rv32::XLen, data: rv32::DoubleWord) {
        let memory = &mut self.0;
        let address = (address - bus::DRAM_BASE) as usize;
        memory[address] = (data & 0xFF) as rv32::Byte;
        memory[address + 1] = ((data >> 8) & 0xFF) as rv32::Byte;
        memory[address + 2] = ((data >> 16) & 0xFF) as rv32::Byte;
        memory[address + 3] = ((data >> 24) & 0xFF) as rv32::Byte;
        memory[address + 4] = ((data >> 32) & 0xFF) as rv32::Byte;
        memory[address + 5] = ((data >> 40) & 0xFF) as rv32::Byte;
        memory[address + 6] = ((data >> 48) & 0xFF) as rv32::Byte;
        memory[address + 7] = ((data >> 56) & 0xFF) as rv32::Byte;
    }
}
