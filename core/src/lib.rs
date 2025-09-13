pub mod lexer;
pub mod ast;
pub mod parser;
pub mod codegen;
pub mod target;
pub mod backend;
pub mod emulator;
// Full LSP server implementation (diagnostics, hover, semantic tokens, etc.)
pub mod lsp;

// Convenience re-exports
pub use lexer::*;
pub use parser::*;
pub use ast::*;
