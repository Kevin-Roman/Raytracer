use std::rc::Rc;

use sortedlist_rs::SortedList;

use crate::core::{
    hit::Hit,
    material::Material,
    object::{BaseObject, Object},
    ray::Ray,
    tex_coords::TexCoords,
    transform::Transform,
    vector::Vector,
    vertex::Vertex,
};

pub struct Plane {
    base: BaseObject,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
}

impl Plane {
    /// Plane `ax + by + cz + d = 0`.
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Self {
            base: BaseObject::new(),
            a,
            b,
            c,
            d,
        }
    }
}

impl Object for Plane {
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

    fn intersection(&mut self, ray: &Ray) {
        let distance_to_plane = self.a * ray.position.vector.x
            + self.b * ray.position.vector.y
            + self.c * ray.position.vector.z
            + self.d;

        let direction_dot_normal =
            self.a * ray.direction.x + self.b * ray.direction.y + self.c * ray.direction.z;

        if direction_dot_normal == 0.0 {
            // Ray is parallel to the plane.
            if distance_to_plane < 0.0 {
                // The ray starts outside the plane and will never intersect.
                self.base.hitpool.insert(Hit::new(
                    f32::NEG_INFINITY,
                    true,
                    Vertex::default(),
                    Vector::default(),
                    TexCoords::default(),
                ));
                self.base.hitpool.insert(Hit::new(
                    f32::INFINITY,
                    false,
                    Vertex::default(),
                    Vector::default(),
                    TexCoords::default(),
                ));
            }

            return;
        }

        let t = distance_to_plane / -direction_dot_normal;
        let hit_position = ray.position + t * ray.direction;
        let mut hit_normal = Vector::new(self.a, self.b, self.c);

        // Flip normal if pointing away from the surface we are looking at.
        if hit_normal.dot(&ray.direction) > 0.0 {
            hit_normal = hit_normal.negate();
        }

        if direction_dot_normal > 0.0 {
            // Ray comes from outside to inside.
            self.base.hitpool.insert(Hit::new(
                f32::NEG_INFINITY,
                true,
                Vertex::default(),
                Vector::default(),
                TexCoords::default(),
            ));
            self.base.hitpool.insert(Hit::new(
                t,
                false,
                hit_position,
                hit_normal,
                TexCoords::default(),
            ));
        } else {
            // Ray comes from inside to outside.
            self.base.hitpool.insert(Hit::new(
                t,
                true,
                hit_position,
                hit_normal,
                TexCoords::default(),
            ));
            self.base.hitpool.insert(Hit::new(
                f32::INFINITY,
                false,
                Vertex::default(),
                Vector::default(),
                TexCoords::default(),
            ));
        }
    }

    fn apply_transform(&mut self, trans: &Transform) {
        let mut v = Vertex::new(self.a, self.b, self.c, self.d);

        trans.inverse().transpose().apply_to_vertex(&mut v);

        self.a = v.vector.x;
        self.b = v.vector.y;
        self.c = v.vector.z;
        self.d = v.w;
    }
}
