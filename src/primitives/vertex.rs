use std::ops::{Add, Neg, Sub};

use super::Vector;

/// A 3D vertex (homogeneous coordinate).
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_add_vector() {
        let v = Vertex::new(1.0, 2.0, 3.0, 1.0);
        let vec = Vector::new(4.0, 5.0, 6.0);
        let result = v + vec;
        assert_eq!(result.vector.x, 5.0);
        assert_eq!(result.vector.y, 7.0);
        assert_eq!(result.vector.z, 9.0);
        assert_eq!(result.w, 1.0);
    }

    #[test]
    fn test_vertex_subtract_vector() {
        let v = Vertex::new(5.0, 7.0, 9.0, 1.0);
        let vec = Vector::new(4.0, 5.0, 6.0);
        let result = v - vec;
        assert_eq!(result.vector.x, 1.0);
        assert_eq!(result.vector.y, 2.0);
        assert_eq!(result.vector.z, 3.0);
        assert_eq!(result.w, 1.0);
    }

    #[test]
    fn test_vertex_negation() {
        let v = Vertex::new(1.0, 2.0, 3.0, 1.0);
        let result = -v;
        assert_eq!(result.vector.x, -1.0);
        assert_eq!(result.vector.y, -2.0);
        assert_eq!(result.vector.z, -3.0);
        assert_eq!(result.w, -1.0);
    }

    #[test]
    fn test_vertex_homogeneous_coordinates() {
        let v = Vertex::new(2.0, 4.0, 6.0, 2.0);
        assert_eq!(v.w, 2.0);
    }
}
