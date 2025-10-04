//! Vectrex Emulator V2
//! Modular port of Vectrexy emulator (https://github.com/amaiorano/vectrexy)
//! 
//! This is a line-by-line port from C++ to Rust, maintaining compatibility
//! and functionality while providing a clean modular architecture.

pub mod types;
pub mod core;

#[cfg(feature = "wasm")]
pub mod wasm_api;

// Re-export commonly used types
pub use types::*;
pub use core::*;

#[cfg(feature = "wasm")]
pub use wasm_api::*;