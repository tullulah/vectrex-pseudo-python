pub mod lexer;
pub mod ast;
pub mod parser;
pub mod codegen;
pub mod target;
pub mod backend;
pub mod emulator;

// Convenience re-exports
pub use lexer::*;
pub use parser::*;
pub use ast::*;
