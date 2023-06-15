use crate::system::rv32;

mod decode;

// Instruction bitmasks
//      This will be awkward as the instruction types
//      are sometimes shared accross modules (float / double)
// Instruction parsing
// Extensibility

type Instruction = rv32::Word;

