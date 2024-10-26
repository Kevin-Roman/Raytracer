// DirectionalLight implements a light with constant value in a
// given direction. The light has no position and can be treated as
// infinitely far away.

use crate::core::{colour::Colour, light::Light, vector::Vector, vertex::Vertex};

pub struct DirectionalLight {
    direction: Vector,
    intensity: Colour,
}

impl DirectionalLight {
    pub fn new(mut direction: Vector, colour: Colour) -> Self {
        direction = direction.normalise();

        Self {
            direction,
            intensity: colour,
        }
    }
}

impl Light for DirectionalLight {
    fn get_direction(&self, _surface: Vertex) -> (Vector, bool) {
        (
            Vector {
                x: self.direction.x,
                y: self.direction.y,
                z: self.direction.z,
            },
            true,
        )
    }

    fn get_intensity(&self, _surface: Vertex) -> Colour {
        Colour {
            a: self.intensity.r,
            r: self.intensity.g,
            g: self.intensity.b,
            b: self.intensity.a,
        }
    }
}
