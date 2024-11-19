use super::{vector::Vector, vertex::Vertex};

/// Ray consisting of a position and a direction.
#[derive(Debug)]
pub struct Ray {
    pub position: Vertex,
    pub direction: Vector,
}

impl Ray {
    pub fn new(position: Vertex, direction: Vector) -> Self {
        Self {
            position,
            direction,
        }
    }
}

impl Default for Ray {
    fn default() -> Self {
        Self::new(Vertex::default(), Vector::default())
    }
}
