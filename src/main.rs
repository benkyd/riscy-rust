use std::fs::File;
use std::io::BufReader;
use std::io::Read;

const XLEN: usize = 32;

const DRAM_SIZE: usize = 1 * 1024 * 1024 * 1024; // 1GB
const DRAM_BASE: usize = 0x8000;

// define words as byte fraction
//const QUADWORD: usize = 16;
//const DOUBLEWORD: usize = 8;
//const WORD: usize = 4;
//const HALFWORD: usize = 2;
//const BYTE: usize = 1;

type QuadWord = u128;
type DoubleWord = u64;
type Word = u32;
type HalfWord = u16;
type Byte = u8;

struct Bus {
    // 1GB of memory
    memory: Vec<Byte>,
}

impl Bus {
    fn new() -> Bus {
        Bus {
            memory: vec![0; DRAM_SIZE],
        }
    }
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}

struct Instruction {
    opcode: Byte,
    rd: Byte,
    rs1: Byte,
    rs2: Byte,
    funct3: Byte,
    funct7: Byte,
    imm: Word,
}

struct VMRV32I {
    // 32 bi bus
    bus: Bus,
    // 32 registers
    x: [Word; 32],
    // 32-bit program counter
    pc: Word,
}

impl VMRV32I {
    fn new() -> VMRV32I {
        VMRV32I {
            bus: Bus::new(),
            x: [0; 32],
            pc: 0,
        }
    }

    fn load_prog(&mut self, file: &str) {
        println!("VM > Loading program: {}", file);

        let f = File::open(file).expect("file not found");
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).expect("error reading file");

        println!("VM > Program size: {} bytes", buffer.len());

        // put program at the base of DRAM
        for i in 0..buffer.len() {
            self.bus.memory[i + DRAM_BASE] = buffer[i];
        }

        println!("VM > Program loaded to 0x{:08x}", self.pc);
    }

    fn init_cpu(&mut self) {
        println!("VM RISC-V 32I CPU");
        println!("-----------------");
        println!("VM > Initializing CPU");

        self.bus = Bus::new();
        self.pc = DRAM_BASE as Word;
        self.x[0] = 0; // x0 is tied to ground
    }
}

fn main() {
    println!("VM Starting Up");

    let mut cpu = VMRV32I::new();
    cpu.init_cpu();
    cpu.load_prog("./test/test.bin");
}

