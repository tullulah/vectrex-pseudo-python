//! Vector2 type for 2D coordinates and math operations
//! Port of vectrexy/libs/core/include/core/Vector2.h

use serde::{Deserialize, Serialize};

/* C++ Original:
struct Vector2 {
    float x = 0.f;
    float y = 0.f;

    void operator+=(const Vector2& rhs) {
        x += rhs.x;
        y += rhs.y;
    }
};
*/
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Default for Vector2 {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

// C++ Original: inline Vector2 operator+(const Vector2& lhs, const Vector2& rhs)
impl std::ops::Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// C++ Original: void operator+=(const Vector2& rhs)
impl std::ops::AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

// C++ Original: inline Vector2 operator-(const Vector2& lhs, const Vector2& rhs)
impl std::ops::Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

// C++ Original: inline Vector2 operator*(const Vector2& lhs, float scalar)
impl std::ops::Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

// C++ Original: inline Vector2 operator/(const Vector2& lhs, float scalar)
impl std::ops::Div<f32> for Vector2 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self::Output {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

// C++ Original: inline float Magnitude(const Vector2& v)
pub fn magnitude(v: Vector2) -> f32 {
    (v.x * v.x + v.y * v.y).sqrt()
}

// C++ Original: inline Vector2 Normalized(const Vector2& v)
pub fn normalized(v: Vector2) -> Vector2 {
    let mag = magnitude(v);
    if mag > 0.0 {
        v / mag
    } else {
        Vector2::zero()
    }
}
