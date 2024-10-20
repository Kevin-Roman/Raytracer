// Ray stores and manipulates 3D rays.

use super::{vector::Vector, vertex::Vertex};

#[derive(Debug)]
pub struct Ray {
    pub position: Vertex,
    pub direction: Vector,
}

impl Ray {
    pub fn new() -> Self {
        Self {
            position: Vertex::default(),
            direction: Vector::default(),
        }
    }

    pub fn from(position: Vertex, direction: Vector) -> Self {
        Self {
            position,
            direction,
        }
    }
}
