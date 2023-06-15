use crate::system::rv32;
use crate::cpu;

macro_rules! match_mask {
    ($int:expr, $($bit:expr)+) => { 'scope: {
        let mut int = $int;

        $({
            let msb = int & (1 << 31);
            let bit = $bit;
            if (bit == 0 || bit == 1) && bit != msb.reverse_bits() {
                break 'scope false;
            }
            int <<= 1;
        })+

        true
    }};
}

trait Instruction {
    fn get_mask() -> rv32::Word;
    fn step(&self, inst: rv32::Word, state: &mut cpu::CPU);
}

struct ADDI {}
impl Instruction for ADDI  {
    fn get_mask() -> rv32::Word {
    }
}

pub fn decode_inst(inst: rv32::WORD) -> fn() {
    // loop over all bitmasks
    // 

}
