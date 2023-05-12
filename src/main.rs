const XLEN: usize = 32;
// define words as byte fraction
const QUADWORD: usize = 16;
const DOUBLEWORD: usize = 8;
const WORD: usize = 4;
const HALFWORD: usize = 2;
const BYTE: usize = 1;

type QuadWord = u128;
type DoubleWord = u64;
type Word = u32;
type HalfWord = u16;
type Byte = u8;

struct VMRV32I {
    // 32 registers
    x: [Word; 32],
    // 32-bit program counter
    pc: Word,
}

struct VMChunk {

}

fn main() {
    println!("Hello, world!");
}

