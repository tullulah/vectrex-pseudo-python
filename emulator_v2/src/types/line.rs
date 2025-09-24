//! Line structure for vector graphics rendering
//! Port of vectrexy/libs/core/include/core/Line.h

use super::vector2::Vector2;

/* C++ Original:
struct Line {
    Vector2 p0;
    Vector2 p1;
    float brightness = 1.f;
};
*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line {
    pub p0: Vector2,
    pub p1: Vector2,
    pub brightness: f32,
}

impl Default for Line {
    fn default() -> Self {
        Self {
            p0: Vector2::default(),
            p1: Vector2::default(),
            brightness: 1.0,
        }
    }
}

impl Line {
    pub fn new(p0: Vector2, p1: Vector2, brightness: f32) -> Self {
        Self { p0, p1, brightness }
    }
}