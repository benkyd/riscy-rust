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

    //fn read<T>(&mut self, address: XLen as usize) -> T {
    //let memory = &self.0;
    //let shift: T;
    //// Little endian!!!
    //match T {
    ////QuadWord:
    ////DoubleWord:
    //Word => shift = (memory[address] as T)
    //| ((memory[address] as T) << 8)
    //| ((memory[address] as T) << 16)
    //| ((memory[address] as T) << 24)
    ////HalfWord:
    ////Byte:
    //}
    //}
}
