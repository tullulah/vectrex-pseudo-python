pub mod lexer;
pub mod ast;
pub mod parser;
pub mod codegen;
pub mod target;
pub mod backend;
// Legacy emulator module removed; use vectrex_emulator crate instead.
// pub mod emulator; // intentionally disabled
#[cfg(not(target_arch = "wasm32"))]
pub mod lsp;
// Removed unused wasm feature gating after emulator extraction.

// Convenience re-exports
pub use lexer::*;
pub use parser::*;
pub use ast::*;
// wasm_api re-export removed (now provided via vectrex_emulator crate when compiling to WASM)
