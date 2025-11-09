use std::sync::Arc;

use crate::{
    core::{
        material::Material,
        object::{BaseObject, HitPool, Object},
    },
    primitives::{hit::Hit, ray::Ray, transform::Transform, vertex::Vertex},
};

pub struct Sphere {
    base: BaseObject,
    center: Vertex,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vertex, radius: f32) -> Self {
        Self {
            base: BaseObject::new(),
            center,
            radius,
        }
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

impl Object for Sphere {
    fn get_material(&self) -> Option<&Arc<dyn Material>> {
        self.base.get_material()
    }

    fn set_material(&mut self, material: Arc<dyn Material>) {
        self.base.set_material(material)
    }

    fn add_intersections(&self, hitpool: &mut HitPool, ray: &Ray) {
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

    fn apply_transform(&mut self, trans: &Transform) {
        trans.apply_to_vertex(&mut self.center);
    }

    fn bounding_sphere(&self) -> Option<(Vertex, f32)> {
        Some((self.center, self.radius))
    }
}
