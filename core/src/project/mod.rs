//! VPy Project System
//!
//! Handles .vpyproj files for multi-file projects with dependencies and resources.

mod schema;
mod loader;

pub use schema::*;
// Re-export loader functions (currently unused, will be used by IDE)
#[allow(unused_imports)]
pub use loader::*;
