// Light is the base class for lights.

use super::{colour::Colour, vector::Vector, vertex::Vertex};

// Light is the base trait for lights``.
pub trait Light {
    fn get_direction(&self, surface: Vertex) -> (Vector, bool);

    fn get_intensity(&self, surface: Vertex) -> Colour;
}

pub struct BaseLight {}

impl BaseLight {
    pub fn new() -> Self {
        Self {}
    }
}

impl Light for BaseLight {
    fn get_direction(&self, _surface: Vertex) -> (Vector, bool) {
        return (Vector::default(), false);
    }

    fn get_intensity(&self, _surface: Vertex) -> Colour {
        return Colour::default();
    }
}
