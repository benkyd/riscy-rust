use crate::bus;
use crate::rv32;

pub const DRAM_SIZE: u32 = 1 * 1024 * 1024 * 1024; // 1GB

pub struct RAM(pub Vec<rv32::Byte>);

impl RAM {
    pub fn new() -> RAM {
        RAM(vec![0; DRAM_SIZE as usize])
    }

    pub fn len(&mut self) -> usize {
        self.0.len()
    }

    pub fn read<T>(&mut self, address: rv32::XLen) -> T
    where
        T: std::ops::Shl<Output = T>
            + std::ops::BitOr<Output = T>
            + From<rv32::Byte>
            + From<rv32::HalfWord>
            + From<rv32::Word>
            //+ From<rv32::DoubleWord>
            + Default
    {
        let address: usize = (address - bus::DRAM_BASE) as usize;
        let memory = &self.0;

        (address..)
            .take(core::mem::size_of::<T>())
            .enumerate()
            .fold(T::default(), |acc, (i, x)| acc | (memory[x] << (i * 8)).into())
    }

    pub fn write<T>(&mut self, address: rv32::XLen, data: T) {

    }
}
