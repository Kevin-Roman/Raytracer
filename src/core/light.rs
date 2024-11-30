use crate::primitives::{colour::Colour, vector::Vector, vertex::Vertex};

/// Light is the trait for lighting in the environment.
pub trait Light: Sync {
    /// Get the position and direction of the light to the surface, and whether the surface is lit by the light.
    fn get_direction(&self, surface: Vertex) -> (Option<Vertex>, Vector, bool);

    fn get_intensity(&self) -> Colour;

    fn get_position(&self) -> Option<Vertex> {
        None
    }
}
