// Hit is stores and manipulates information about an intersection
// between a ray and an object.

use std::cmp::Ordering;

use super::{tex_coords::TexCoords, vector::Vector, vertex::Vertex};

#[derive(Clone, Copy, Debug)]
pub struct Hit {
    pub t: f32,                 // The intersection distance.
    pub entering: bool,         // True if entering an object, false if leaving.
    pub position: Vertex,       // The position of intersection.
    pub normal: Vector,         // The normal at the point of intersection.
    pub coordinates: TexCoords, // The texture coordinates at the point of intersection.
}

impl Hit {
    pub fn new(
        t: f32,
        entering: bool,
        position: Vertex,
        normal: Vector,
        coordinates: TexCoords,
    ) -> Self {
        Self {
            t,
            entering,
            position,
            normal,
            coordinates,
        }
    }
}

impl Ord for Hit {
    fn cmp(&self, other: &Self) -> Ordering {
        self.t.total_cmp(&other.t)
    }
}

impl PartialOrd for Hit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hit {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
    }
}

impl Eq for Hit {}
