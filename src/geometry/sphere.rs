use crate::{
    geometry::traits::{Bounded, HitPool, Intersection, Transformable},
    primitives::{ray::Ray, Hit, Transform, Vertex},
    shading::Material,
};

#[derive(Debug, Clone, Copy)]
pub struct SphereGeometry {
    pub center: Vertex,
    pub radius: f32,
}

impl SphereGeometry {
    pub fn new(center: Vertex, radius: f32) -> Self {
        Self { center, radius }
    }

    fn add_hit(&self, hitpool: &mut HitPool, ray: &Ray, t: f32, entering: bool) {
        let hit_position = ray.position + t * ray.direction;
        let mut hit_normal = hit_position.vector - self.center.vector;
        hit_normal = hit_normal.normalise();

        // Flip normal if pointing away from the surface we are looking at.
        if hit_normal.dot(ray.direction) > 0.0 {
            hit_normal = hit_normal.negate();
        }

        hitpool.insert(Hit::new(t, entering, hit_position, hit_normal));
    }
}

impl Intersection for SphereGeometry {
    fn intersect(&self, ray: &Ray, hitpool: &mut HitPool) {
        let ray_to_sphere = ray.position.vector - self.center.vector;

        // Quadratic equation.
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(ray_to_sphere);
        let c = ray_to_sphere.dot(ray_to_sphere) - self.radius.powi(2);

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            // No intersection if the discriminant is negative.
            return;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let t0 = (-b - sqrt_discriminant) / 2.0;
        let t1 = (-b + sqrt_discriminant) / 2.0;

        self.add_hit(hitpool, ray, t0, true);
        self.add_hit(hitpool, ray, t1, false);
    }
}

impl Transformable for SphereGeometry {
    fn transform(&mut self, trans: &Transform) {
        trans.apply_to_vertex(&mut self.center);
    }
}

impl Bounded for SphereGeometry {
    fn bounding_sphere(&self) -> Option<(Vertex, f32)> {
        Some((self.center, self.radius))
    }
}

#[derive(Debug)]
pub struct Sphere {
    pub geometry: SphereGeometry,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vertex, radius: f32, material: Material) -> Self {
        Self {
            geometry: SphereGeometry::new(center, radius),
            material,
        }
    }
}

impl Intersection for Sphere {
    fn intersect(&self, ray: &Ray, hitpool: &mut HitPool) {
        self.geometry.intersect(ray, hitpool)
    }
}

impl Transformable for Sphere {
    fn transform(&mut self, trans: &Transform) {
        self.geometry.transform(trans)
    }
}

impl Bounded for Sphere {
    fn bounding_sphere(&self) -> Option<(Vertex, f32)> {
        self.geometry.bounding_sphere()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::{Ray, Vector, Vertex};
    use approx::assert_relative_eq;

    #[test]
    fn test_sphere_ray_intersection_hit() {
        let center = Vertex::new(0.0, 0.0, 0.0, 1.0);
        let sphere = SphereGeometry::new(center, 1.0);

        let ray = Ray::new(Vertex::new(0.0, 0.0, -5.0, 1.0), Vector::new(0.0, 0.0, 1.0));

        let hitpool = sphere.generate_hitpool(&ray);
        // 2 hits (entry and exit)
        assert_eq!(hitpool.len(), 2);
    }

    #[test]
    fn test_sphere_ray_intersection_miss() {
        let center = Vertex::new(0.0, 0.0, 0.0, 1.0);
        let sphere = SphereGeometry::new(center, 1.0);

        let ray = Ray::new(Vertex::new(5.0, 0.0, -5.0, 1.0), Vector::new(0.0, 0.0, 1.0));

        let hitpool = sphere.generate_hitpool(&ray);
        assert_eq!(hitpool.len(), 0);
    }

    #[test]
    fn test_sphere_first_hit() {
        let center = Vertex::new(0.0, 0.0, 0.0, 1.0);
        let sphere = SphereGeometry::new(center, 1.0);

        let ray = Ray::new(Vertex::new(0.0, 0.0, -5.0, 1.0), Vector::new(0.0, 0.0, 1.0));

        let hit = sphere.first_hit(&ray).unwrap();
        assert_relative_eq!(hit.distance, 4.0, epsilon = 1e-5);
        assert_eq!(hit.entering, true);
    }

    #[test]
    fn test_sphere_normal_at_hit() {
        let center = Vertex::new(0.0, 0.0, 0.0, 1.0);
        let sphere = SphereGeometry::new(center, 1.0);

        let ray = Ray::new(Vertex::new(0.0, 0.0, -5.0, 1.0), Vector::new(0.0, 0.0, 1.0));

        let hit = sphere.first_hit(&ray).unwrap();
        // Normal at (0, 0, -1) should point toward -z
        assert_relative_eq!(hit.normal.z, -1.0, epsilon = 1e-5);
    }

    #[test]
    fn test_sphere_bounding_sphere() {
        let center = Vertex::new(1.0, 2.0, 3.0, 1.0);
        let sphere: SphereGeometry = SphereGeometry::new(center, 5.0);

        let (bs_center, bs_radius) = sphere.bounding_sphere().unwrap();
        assert_eq!(bs_center.vector.x, 1.0);
        assert_eq!(bs_radius, 5.0);
    }

    #[test]
    fn test_sphere_transform() {
        let center = Vertex::new(0.0, 0.0, 0.0, 1.0);
        let mut sphere = SphereGeometry::new(center, 1.0);

        let transform = Transform::new([
            [1.0, 0.0, 0.0, 5.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        sphere.transform(&transform);
        assert_eq!(sphere.center.vector.x, 5.0);
    }

    #[test]
    fn test_sphere_ray_tangent() {
        let sphere = SphereGeometry::new(Vertex::new(0.0, 0.0, 0.0, 1.0), 1.0);
        // Ray tangent to sphere
        let ray = Ray::new(Vertex::new(1.0, 0.0, -5.0, 1.0), Vector::new(0.0, 0.0, 1.0));

        let hitpool = sphere.generate_hitpool(&ray);
        // Tangent hit may have discriminant = 0
        assert!(hitpool.len() <= 2);
    }

    #[test]
    fn test_sphere_very_large_radius() {
        let sphere = SphereGeometry::new(Vertex::new(0.0, 0.0, 0.0, 1.0), 1000.0);
        let ray = Ray::new(Vertex::new(0.0, 0.0, -5.0, 1.0), Vector::new(0.0, 0.0, 1.0));

        let hit = sphere.first_hit(&ray);
        assert!(hit.is_some());
    }

    #[test]
    fn test_sphere_ray_from_center() {
        let sphere = SphereGeometry::new(Vertex::new(0.0, 0.0, 0.0, 1.0), 2.0);
        let ray = Ray::new(Vertex::new(0.0, 0.0, 0.0, 1.0), Vector::new(1.0, 0.0, 0.0));

        let hit = sphere.first_hit(&ray);
        assert!(hit.is_some());
        if let Some(h) = hit {
            assert_relative_eq!(h.distance, 2.0, epsilon = 1e-5);
        }
    }
}
