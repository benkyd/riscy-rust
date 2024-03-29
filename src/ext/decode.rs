use strum::IntoEnumIterator;

use super::encoding::{GenInstruction, Instruction};
use crate::cpu;
use crate::system::rv32;

use crate::ext::a;
use crate::ext::i;
use crate::ext::m;
use crate::ext::z;

pub struct DecodeCycle {
    extensions: Vec<char>,
}

impl DecodeCycle {
    pub fn new(ext: Vec<char>) -> DecodeCycle {
        DecodeCycle { extensions: ext }
    }

    // TODO: speed this up by matching based on the opcode field and then fn3
    // TODO: Pass around only the union
    pub fn decode_exec_inst(
        &self,
        inst: rv32::Word,
        state: &mut cpu::CPUState,
    ) -> Result<(), &str> {
        // we want to go through each extension and then go through each instruction in that extension
        // if we find a match, we want to execute it
        // if we don't find a match, we want to return an error

        fn enumerate_extension<T: IntoEnumIterator + Instruction>(
            inst: rv32::Word,
            state: &mut cpu::CPUState,
        ) -> Option<()> {
            for instruction in T::iter() {
                if instruction.match_inst(inst) {
                    let geninst = GenInstruction { inst };
                    instruction.step(geninst, state);
                    return Some(());
                }
            }
            None
        }

        for extension in self.extensions.iter() {
            match extension {
                'i' => {
                    if let Some(()) = enumerate_extension::<i::ExtensionI>(inst, state) {
                        return Ok(());
                    }
                }
                'a' => {
                    if let Some(()) = enumerate_extension::<a::ExtensionA>(inst, state) {
                        return Ok(());
                    }
                }
                'm' => {
                    if let Some(()) = enumerate_extension::<m::ExtensionM>(inst, state) {
                        return Ok(());
                    }
                }
                'z' => {
                    if let Some(()) = enumerate_extension::<z::ExtensionZ>(inst, state) {
                        return Ok(());
                    }
                }
                _ => println!("VM > Unknown Extension"),
            }
        }
        Err("No instruction found")
    }
}
