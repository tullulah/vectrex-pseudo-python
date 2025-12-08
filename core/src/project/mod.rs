//! VPy Project System
//!
//! Handles .vpyproj files for multi-file projects with dependencies and resources.

mod schema;
mod loader;

pub use schema::*;
pub use loader::*;
