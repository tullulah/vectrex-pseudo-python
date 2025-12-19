pub mod lexer;
pub mod ast;
pub mod parser;
pub mod codegen;
pub mod target;
pub mod backend;
pub mod project;  // VPy project system (.vpyproj)
pub mod resolver; // Multi-file import resolution
pub mod unifier;  // AST unification for multi-file projects
pub mod library;  // VPy library system (.vpylib)
pub mod vecres;   // Vector resource format (.vec)
pub mod musres;   // Music resource format (.vmus)
pub mod struct_layout; // Struct layout computation (Phase 2)
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
