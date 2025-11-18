use super::{Vector, Vertex};

/// Ray consisting of a position and a (normalised) direction.
#[derive(Debug)]
pub struct Ray {
    pub position: Vertex,
    pub direction: Vector,
}

impl Ray {
    pub fn new(position: Vertex, direction: Vector) -> Self {
        Self {
            position,
            direction,
        }
    }
}

impl Default for Ray {
    fn default() -> Self {
        Self::new(Vertex::default(), Vector::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_point_at_parameter() {
        let pos = Vertex::new(0.0, 0.0, 0.0, 1.0);
        let dir = Vector::new(1.0, 0.0, 0.0);
        let ray = Ray::new(pos, dir);

        // Point at t=5 should be at (5, 0, 0)
        let point = ray.position + 5.0 * ray.direction;
        assert_eq!(point.vector.x, 5.0);
        assert_eq!(point.vector.y, 0.0);
        assert_eq!(point.vector.z, 0.0);
    }
}
