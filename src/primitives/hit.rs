use std::cmp::Ordering;

use super::{vector::Vector, vertex::Vertex};

/// Hit stores information about an intersection between a ray and an object.
#[derive(Clone, Copy, Debug)]
pub struct Hit {
    /// The intersection distance.
    pub distance: f32,
    /// Whether the ray is entering the object.
    pub entering: bool,
    /// The position of intersection.
    pub position: Vertex,
    /// The normal at the point of intersection.
    pub normal: Vector,
}

impl Hit {
    pub fn new(distance: f32, entering: bool, position: Vertex, normal: Vector) -> Self {
        Self {
            distance,
            entering,
            position,
            normal,
        }
    }
}

// Compare hits by their intersection distance.
impl Ord for Hit {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.total_cmp(&other.distance)
    }
}

impl PartialOrd for Hit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hit {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for Hit {}
