//! Vectrex Emulator V2
//! Modular port of Vectrexy emulator (https://github.com/amaiorano/vectrexy)
//!
//! This is a line-by-line port from C++ to Rust, maintaining compatibility
//! and functionality while providing a clean modular architecture.

pub mod core;
pub mod types;

pub mod wasm_api;
// pub mod wasm_api_simple;
pub mod simple_test;
pub mod simple_wasm_test;

// Re-export commonly used types
pub use core::*;
pub use types::*;

pub use wasm_api::*;
// pub use wasm_api_simple::*;
pub use simple_test::*;
pub use simple_wasm_test::*;
