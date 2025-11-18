use crate::primitives::{Colour, Vector, Vertex};

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
    pub fn new_directional(direction: Vector, intensity: Colour) -> Self {
        Self::Directional {
            direction: direction.normalise(),
            intensity,
        }
    }

    pub fn new_point(position: Vertex, intensity: Colour) -> Self {
        Self::Point {
            position,
            intensity,
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_directional_light_normalizes_direction() {
        let direction = Vector::new(3.0, -4.0, 0.0);
        let intensity = Colour::new(1.0, 1.0, 1.0, 1.0);
        let light = Light::new_directional(direction, intensity);

        let surface = Vertex::new(0.0, 0.0, 0.0, 1.0);
        let (pos, dir, lit) = light.get_direction(surface);

        assert!(pos.is_none());
        assert!(lit);
        assert_relative_eq!(dir.length(), 1.0, epsilon = 1e-6);
    }

    #[test]
    fn test_point_light_direction_calculation() {
        let position = Vertex::new(0.0, 5.0, 0.0, 1.0);
        let intensity = Colour::new(1.0, 1.0, 1.0, 1.0);
        let light = Light::new_point(position, intensity);

        let surface = Vertex::new(0.0, 0.0, 0.0, 1.0);
        let (light_pos, dir, lit) = light.get_direction(surface);

        assert!(light_pos.is_some());
        assert!(lit);
        assert_relative_eq!(dir.length(), 1.0, epsilon = 1e-6);
        // Direction should point from surface to light (upward)
        assert!(dir.y < 0.0);
    }

    #[test]
    fn test_directional_light_same_direction_everywhere() {
        let direction = Vector::new(1.0, -1.0, 0.0);
        let intensity = Colour::new(1.0, 1.0, 1.0, 1.0);
        let light = Light::new_directional(direction, intensity);

        let surface1 = Vertex::new(0.0, 0.0, 0.0, 1.0);
        let surface2 = Vertex::new(100.0, 200.0, 300.0, 1.0);

        let (_, dir1, _) = light.get_direction(surface1);
        let (_, dir2, _) = light.get_direction(surface2);

        assert_relative_eq!(dir1.x, dir2.x, epsilon = 1e-6);
        assert_relative_eq!(dir1.y, dir2.y, epsilon = 1e-6);
        assert_relative_eq!(dir1.z, dir2.z, epsilon = 1e-6);
    }

    #[test]
    fn test_point_light_different_directions() {
        let position = Vertex::new(0.0, 5.0, 0.0, 1.0);
        let intensity = Colour::new(1.0, 1.0, 1.0, 1.0);
        let light = Light::new_point(position, intensity);

        let surface1 = Vertex::new(5.0, 0.0, 0.0, 1.0);
        let surface2 = Vertex::new(-5.0, 0.0, 0.0, 1.0);

        let (_, dir1, _) = light.get_direction(surface1);
        let (_, dir2, _) = light.get_direction(surface2);

        // Directions should be different
        assert!(dir1.x != dir2.x);
    }
}
