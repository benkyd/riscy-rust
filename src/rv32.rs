pub const XLEN: usize = 32;
pub type XLen = u32;

enum Data {
    QuadWord(u128),
    DoubleWord(u64),
    Word(u32),
    HalfWord(u16),
    Byte(u8),
}

// define words as byte fraction
//pub const QUADWORD: usize = 16;
pub const DOUBLEWORD: usize = 8;
pub const WORD: usize = 4;
pub const HALFWORD: usize = 2;
pub const BYTE: usize = 1;

//pub type DoubleWord = u64;
pub type Word = u32;
pub type HalfWord = u16;
pub type Byte = u8;
