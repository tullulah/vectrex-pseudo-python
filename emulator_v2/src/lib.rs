//! Vectrex Emulator V2
//! Modular port of Vectrexy emulator (https://github.com/amaiorano/vectrexy)
//!
//! This is a line-by-line port from C++ to Rust, maintaining compatibility
//! and functionality while providing a clean modular architecture.

pub mod core;
pub mod types;

#[cfg(feature = "wasm")]
pub mod wasm_api;

// Re-export commonly used types
pub use core::*;
pub use types::*;

#[cfg(feature = "wasm")]
pub use wasm_api::*;
