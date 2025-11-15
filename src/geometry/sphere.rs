use crate::{
    geometry::traits::{Bounded, HitPool, Intersection, Transformable},
    primitives::{ray::Ray, Hit, Transform, Vertex},
    shading::scene_material::MaterialId,
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
    pub material_id: MaterialId,
}

impl Sphere {
    pub fn new(center: Vertex, radius: f32) -> Self {
        Self {
            geometry: SphereGeometry::new(center, radius),
            material_id: MaterialId::default(),
        }
    }

    pub fn with_material(mut self, material_id: MaterialId) -> Self {
        self.material_id = material_id;
        self
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
