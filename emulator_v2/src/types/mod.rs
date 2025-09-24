//! Type definitions and basic structures
//! Port of various header files from vectrexy

pub mod base;
pub mod vector2;
pub mod line;

pub use base::*;
pub use vector2::{Vector2, magnitude, normalized};
pub use line::Line;