use crate::system::bus;
use crate::system::rv32;

pub const UART_TXD: u32 = bus::UART_BASE + 0x00;
pub const UART_RXD: u32 = bus::UART_BASE + 0x05;

fn didkeypress() -> bool {
    use std::io::Read;
    let mut stdin = std::io::stdin();
    let mut buffer = [0; 1];
    stdin.read_exact(&mut buffer).unwrap();
    buffer[0] != 0
}

fn getkey() -> u8 {
    use std::io::Read;
    let mut stdin = std::io::stdin();
    let mut buffer = [0; 1];
    stdin.read_exact(&mut buffer).unwrap();
    buffer[0]
}


pub struct UART();
impl UART {
    pub fn new() -> UART {
        println!("VM > Initialised UART");
        UART()
    }

    pub fn write(&mut self, address: rv32::XLen, value: rv32::Word) {
        match address {
            UART_TXD => {
                print!("{}", value as u8 as char);
            }
            _ => {
                panic!("VM > UART > Peripheral at 0x{:08x} does not exist", address);
            }
        }
    }

    pub fn read_kb(&mut self, address: rv32::XLen) -> rv32::Word {
        if address != UART_RXD {
            return 0x60 | didkeypress() as u8 as rv32::Word;
        } else if address == UART_TXD && didkeypress() {
            return getkey() as rv32::Word;
        }
        0
    }
}

