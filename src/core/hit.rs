// Hit is stores and manipulates information about an intersection
// between a ray and an object.

use super::{tex_coords::TexCoords, vector::Vector, vertex::Vertex};

#[derive(Debug)]
pub struct Hit {
    pub t: f32,                         // The intersection distance.
    pub entering: bool,                 // True if entering an object, false if leaving.
    pub position: Option<Vertex>,       // The position of intersection.
    pub normal: Option<Vector>,         // The normal at the point of intersection.
    pub coordinates: Option<TexCoords>, // The texture coordinates at the point of intersection.
}

impl Hit {
    pub fn new(
        t: f32,
        entering: bool,
        position: Option<Vertex>,
        normal: Option<Vector>,
        coordinates: Option<TexCoords>,
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
