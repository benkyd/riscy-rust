use crate::system::rv32;

pub mod encoding;
pub mod decode;
pub mod i;
pub mod a;
pub mod m;
pub mod z;

// Instruction bitmasks
//      This will be awkward as the instruction types
//      are sometimes shared accross modules (float / double)
// Instruction parsing
// Extensibility

