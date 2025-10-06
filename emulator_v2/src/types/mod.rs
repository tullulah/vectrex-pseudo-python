//! Type definitions and basic structures
//! Port of various header files from vectrexy

pub mod base;
pub mod line;
pub mod vector2;

pub use base::*;
pub use line::Line;
pub use vector2::{magnitude, normalized, Vector2};
