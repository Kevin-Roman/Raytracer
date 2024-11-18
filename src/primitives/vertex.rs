// A four element vector with lots of operators and common functions.

use std::ops::{Add, Neg, Sub};

use super::vector::Vector;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub vector: Vector,
    pub w: f32,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            vector: Vector::new(x, y, z),
            w,
        }
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }
}

impl Add<Vector> for Vertex {
    type Output = Self;

    fn add(self, other: Vector) -> Self::Output {
        Self::new(
            self.vector.x + other.x,
            self.vector.y + other.y,
            self.vector.z + other.z,
            self.w,
        )
    }
}

impl Sub<Vector> for Vertex {
    type Output = Self;

    fn sub(self, other: Vector) -> Self::Output {
        Self::new(
            self.vector.x - other.x,
            self.vector.y - other.y,
            self.vector.z - other.z,
            self.w,
        )
    }
}

impl Neg for Vertex {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.vector.x, -self.vector.y, -self.vector.z, -self.w)
    }
}
