use crate::{
    core::light::Light,
    primitives::{colour::Colour, vector::Vector, vertex::Vertex},
};

/// DirectionalLight is a light with constant intensity in a given direction.
pub struct DirectionalLight {
    direction: Vector,
    intensity: Colour,
}

impl DirectionalLight {
    pub fn new(direction: Vector, colour: Colour) -> Self {
        Self {
            direction: direction.normalise(),
            intensity: colour,
        }
    }
}

impl Light for DirectionalLight {
    fn get_direction(&self, _surface: Vertex) -> (Option<Vertex>, Vector, bool) {
        (None, self.direction, true)
    }

    fn get_intensity(&self, _surface: Vertex) -> Colour {
        self.intensity
    }
}
