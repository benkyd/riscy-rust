pub const DRAM_BASE: u32 = 0x80000000;

use crate::ram;

pub struct Bus {
    pub memory: ram::RAM,
}

impl Bus {
    pub fn new() -> Bus {
        Bus { memory: ram::RAM::new() }
    }
}
