use crate::{
    core::light::Light,
    primitives::{colour::Colour, vector::Vector, vertex::Vertex},
};

/// Light with constant intensity in a given direction.
pub struct PointLight {
    position: Vertex,
    intensity: Colour,
}

impl PointLight {
    pub fn new(position: Vertex, colour: Colour) -> Self {
        Self {
            position,
            intensity: colour,
        }
    }
}

impl Light for PointLight {
    fn get_direction(&self, surface: Vertex) -> (Option<Vertex>, Vector, bool) {
        (
            Some(self.position),
            (surface.vector - self.position.vector).normalise(),
            true,
        )
    }

    fn get_intensity(&self) -> Colour {
        self.intensity
    }

    fn get_position(&self) -> Option<Vertex> {
        Some(self.position)
    }
}
