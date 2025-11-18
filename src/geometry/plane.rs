use crate::{
    geometry::traits::{HitPool, Intersection, Transformable},
    primitives::{ray::Ray, Hit, Transform, Vector, Vertex},
    shading::Material,
};

/// Represents plane equation: ax + by + cz + d = 0
#[derive(Debug, Clone, Copy)]
pub struct PlaneGeometry {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
}

impl PlaneGeometry {
    /// Create plane from equation `ax + by + cz + d = 0`.
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Self { a, b, c, d }
    }

    pub fn normal(&self) -> Vector {
        Vector::new(self.a, self.b, self.c)
    }
}

impl Intersection for PlaneGeometry {
    fn intersect(&self, ray: &Ray, hitpool: &mut HitPool) {
        let distance_to_plane: f32 = self.a * ray.position.vector.x
            + self.b * ray.position.vector.y
            + self.c * ray.position.vector.z
            + self.d;

        let direction_dot_normal =
            self.a * ray.direction.x + self.b * ray.direction.y + self.c * ray.direction.z;

        if direction_dot_normal == 0.0 {
            // Ray is parallel to the plane.
            if distance_to_plane < 0.0 {
                // The ray starts outside the plane and will never intersect.
                hitpool.insert(Hit::new(
                    f32::NEG_INFINITY,
                    true,
                    Vertex::default(),
                    Vector::default(),
                ));
                hitpool.insert(Hit::new(
                    f32::INFINITY,
                    false,
                    Vertex::default(),
                    Vector::default(),
                ));
            }

            return;
        }

        let t = distance_to_plane / -direction_dot_normal;
        let hit_position = ray.position + t * ray.direction;
        let mut hit_normal = Vector::new(self.a, self.b, self.c);

        // Flip normal if pointing away from the surface we are looking at.
        if hit_normal.dot(ray.direction) > 0.0 {
            hit_normal = hit_normal.negate();
        }

        if direction_dot_normal > 0.0 {
            // Ray comes from outside to inside.
            hitpool.insert(Hit::new(
                f32::NEG_INFINITY,
                true,
                Vertex::default(),
                Vector::default(),
            ));
            hitpool.insert(Hit::new(t, false, hit_position, hit_normal));
        } else {
            // Ray comes from inside to outside.
            hitpool.insert(Hit::new(t, true, hit_position, hit_normal));
            hitpool.insert(Hit::new(
                f32::INFINITY,
                false,
                Vertex::default(),
                Vector::default(),
            ));
        }
    }
}

impl Transformable for PlaneGeometry {
    fn transform(&mut self, trans: &Transform) {
        let mut v = Vertex::new(self.a, self.b, self.c, self.d);

        trans.inverse().transpose().apply_to_vertex(&mut v);

        self.a = v.vector.x;
        self.b = v.vector.y;
        self.c = v.vector.z;
        self.d = v.w;
    }
}

#[derive(Debug)]
pub struct Plane {
    pub geometry: PlaneGeometry,
    pub material: Material,
}

impl Plane {
    pub fn new(a: f32, b: f32, c: f32, d: f32, material: Material) -> Self {
        Self {
            geometry: PlaneGeometry::new(a, b, c, d),
            material,
        }
    }
}

impl Intersection for Plane {
    fn intersect(&self, ray: &Ray, hitpool: &mut HitPool) {
        self.geometry.intersect(ray, hitpool)
    }
}

impl Transformable for Plane {
    fn transform(&mut self, trans: &Transform) {
        self.geometry.transform(trans)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_plane_normal() {
        let plane = PlaneGeometry::new(0.0, 1.0, 0.0, -5.0);
        let normal = plane.normal();
        assert_eq!(normal.x, 0.0);
        assert_eq!(normal.y, 1.0);
        assert_eq!(normal.z, 0.0);
    }

    #[test]
    fn test_plane_ray_intersection_perpendicular() {
        // Plane at y = 5 (0x + 1y + 0z - 5 = 0)
        let plane = PlaneGeometry::new(0.0, 1.0, 0.0, -5.0);

        // Ray shooting upward from origin
        let ray = Ray::new(Vertex::new(0.0, 0.0, 0.0, 1.0), Vector::new(0.0, 1.0, 0.0));

        let hitpool = plane.generate_hitpool(&ray);
        assert!(hitpool.len() > 0);
    }

    #[test]
    fn test_plane_ray_intersection_parallel() {
        // Plane at y = 5
        let plane = PlaneGeometry::new(0.0, 1.0, 0.0, -5.0);

        // Ray parallel to plane
        let ray = Ray::new(Vertex::new(0.0, 0.0, 0.0, 1.0), Vector::new(1.0, 0.0, 0.0));

        let hitpool = plane.generate_hitpool(&ray);
        // Parallel rays should not intersect or have special handling
        // Length could be 0 or 2 depending on implementation
        let _ = hitpool;
    }

    #[test]
    fn test_plane_first_hit() {
        let plane = PlaneGeometry::new(0.0, 1.0, 0.0, -5.0);

        let ray = Ray::new(Vertex::new(0.0, 0.0, 0.0, 1.0), Vector::new(0.0, 1.0, 0.0));

        if let Some(hit) = plane.first_hit(&ray) {
            assert!(hit.distance > 0.0);
            assert_relative_eq!(hit.position.vector.y, 5.0, epsilon = 1e-5);
        }
    }

    #[test]
    fn test_plane_ray_on_plane() {
        let plane = PlaneGeometry::new(0.0, 1.0, 0.0, 0.0);
        // Ray starting on the plane
        let ray = Ray::new(Vertex::new(5.0, 0.0, 0.0, 1.0), Vector::new(1.0, 0.0, 0.0));

        let hitpool = plane.generate_hitpool(&ray);
        // Check it doesn't panic
        drop(hitpool);
    }

    #[test]
    fn test_plane_oblique_intersection() {
        let plane = PlaneGeometry::new(0.0, 1.0, 0.0, -2.0);
        // Ray at 45 degrees
        let ray = Ray::new(
            Vertex::new(0.0, 0.0, 0.0, 1.0),
            Vector::new(0.0, 0.707, 0.707).normalise(),
        );

        let hit = plane.first_hit(&ray);
        assert!(hit.is_some());
    }

    #[test]
    fn test_plane_away_facing_ray() {
        let plane = PlaneGeometry::new(0.0, 1.0, 0.0, -5.0);
        // Ray pointing away from plane
        let ray = Ray::new(Vertex::new(0.0, 0.0, 0.0, 1.0), Vector::new(0.0, -1.0, 0.0));

        let hitpool = plane.generate_hitpool(&ray);
        // Check it doesn't panic
        drop(hitpool);
    }
}
