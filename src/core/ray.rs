// Ray stores and manipulates 3D rays.

use super::{vector::Vector, vertex::Vertex};
use std::fmt;

pub struct Ray {
    pub position: Vertex,
    pub direction: Vector,
}

impl Ray {
    pub fn new() -> Self {
        Self {
            position: Vertex::origin(),
            direction: Vector::zero(),
        }
    }

    pub fn from(position: Vertex, direction: Vector) -> Self {
        Self {
            position,
            direction,
        }
    }
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Ray{{[{}, {}, {}], [{}, {}, {}]}}",
            self.position.vector.x,
            self.position.vector.y,
            self.position.vector.z,
            self.direction.x,
            self.direction.y,
            self.direction.z
        )
    }
}
