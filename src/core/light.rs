use super::{colour::Colour, vector::Vector, vertex::Vertex};

pub trait Light {
    fn get_direction(&self, surface: Vertex) -> (Vector, bool);

    fn get_intensity(&self, surface: Vertex) -> Colour;
}
