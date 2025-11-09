use crate::primitives::{colour::Colour, vector::Vector, vertex::Vertex};

/// Enum-based light type for zero-cost static dispatch.
/// This replaces Box<dyn Light> for better performance.
#[derive(Clone)]
pub enum Light {
    Directional {
        direction: Vector,
        intensity: Colour,
    },
    Point {
        position: Vertex,
        intensity: Colour,
    },
}

impl Light {
    /// Create a new directional light
    pub fn new_directional(direction: Vector, intensity: Colour) -> Self {
        Self::Directional {
            direction: direction.normalise(),
            intensity,
        }
    }

    /// Create a new point light
    pub fn new_point(position: Vertex, intensity: Colour) -> Self {
        Self::Point {
            position,
            intensity,
        }
    }

    /// Get the position and direction of the light to the surface, and whether the surface is lit by the light.
    pub fn get_direction(&self, surface: Vertex) -> (Option<Vertex>, Vector, bool) {
        match self {
            Light::Directional { direction, .. } => (None, *direction, true),
            Light::Point { position, .. } => (
                Some(*position),
                (surface.vector - position.vector).normalise(),
                true,
            ),
        }
    }

    pub fn get_intensity(&self) -> Colour {
        match self {
            Light::Directional { intensity, .. } => *intensity,
            Light::Point { intensity, .. } => *intensity,
        }
    }

    pub fn get_position(&self) -> Option<Vertex> {
        match self {
            Light::Directional { .. } => None,
            Light::Point { position, .. } => Some(*position),
        }
    }
}
