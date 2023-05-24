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
        T: num::Num
            + num::ToPrimitive
            + Default
            + std::fmt::LowerHex
            + std::ops::Shl<T, Output = T>
            + std::ops::BitOr<T, Output = T>
            + num::cast::NumCast,
    {
        let address: usize = (address - bus::DRAM_BASE) as usize;
        let memory = &self.0;

        (address..)
            .take(core::mem::size_of::<T>())
            .enumerate()
            .fold(T::default(), |mut acc, (i, x)| {
                println!("VM > Reading from 0x{:08x} to 0x{:08x}", x, acc);
                println!("VM > Memory: 0x{:02x}", memory[x]);
                println!("VM > Now Shift: {}", i * 8);

                acc << u32::from(i as u32 * 8) | memory[x].from()
            })
    }

    pub fn write<T>(&mut self, address: rv32::XLen, data: T) {}
}
