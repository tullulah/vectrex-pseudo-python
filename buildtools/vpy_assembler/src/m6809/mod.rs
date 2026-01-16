//! M6809 assembler implementation
//! 
//! Port from core/src/backend/ - native M6809 assembler

pub mod binary_emitter;
pub mod asm_to_binary;

pub use binary_emitter::BinaryEmitter;
pub use asm_to_binary::{assemble_m6809, set_include_dir, load_vectrex_symbols};
