// DirectionalLight implements a light with constant value in a
// given direction. The light has no position and can be treated as
// infinitely far away.

use crate::{
    core::light::Light,
    primitives::{colour::Colour, vector::Vector, vertex::Vertex},
};

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
    fn get_direction(&self, _surface: Vertex) -> (Vector, bool) {
        (self.direction, true)
    }

    fn get_intensity(&self, _surface: Vertex) -> Colour {
        self.intensity
    }
}
