use std::rc::Rc;

use sortedlist_rs::SortedList;

use crate::{
    core::{
        material::Material,
        object::{BaseObject, Object},
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

    fn add_hit(&mut self, ray: &Ray, t: f32, entering: bool) {
        let hit_position = ray.position + t * ray.direction;
        let mut hit_normal = hit_position.vector - self.center.vector;
        hit_normal = hit_normal.normalise();

        // Flip normal if pointing away from the surface we are looking at.
        if hit_normal.dot(&ray.direction) > 0.0 {
            hit_normal = hit_normal.negate();
        }

        self.base
            .hitpool
            .insert(Hit::new(t, entering, hit_position, hit_normal));
    }
}

impl Object for Sphere {
    fn get_hitpool(&mut self) -> &mut SortedList<Hit> {
        self.base.get_hitpool()
    }

    fn select_first_hit(&mut self) -> Option<Hit> {
        self.base.select_first_hit()
    }

    fn get_material(&self) -> Option<&Rc<dyn Material>> {
        self.base.get_material()
    }

    fn set_material(&mut self, material: Rc<dyn Material>) {
        self.base.set_material(material)
    }

    fn add_intersections(&mut self, ray: &Ray) {
        let ray_to_sphere = ray.position.vector - self.center.vector;

        // Quadratic equation.
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&ray_to_sphere);
        let c = ray_to_sphere.dot(&ray_to_sphere) - self.radius.powi(2);

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            // No intersection if the discriminant is negative.
            return;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let t0 = (-b - sqrt_discriminant) / 2.0;
        let t1 = (-b + sqrt_discriminant) / 2.0;

        self.add_hit(ray, t0, true);
        self.add_hit(ray, t1, false);
    }

    fn apply_transform(&mut self, trans: &Transform) {
        trans.apply_to_vertex(&mut self.center);
    }
}
