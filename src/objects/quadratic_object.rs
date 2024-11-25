use std::rc::Rc;

use crate::{
    core::{
        material::Material,
        object::{BaseObject, HitPool, Object},
    },
    primitives::{hit::Hit, ray::Ray, transform::Transform, vector::Vector, vertex::Vertex},
};

pub struct Quadratic {
    base: BaseObject,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    e: f32,
    f: f32,
    g: f32,
    h: f32,
    i: f32,
    j: f32,
}

// Object defined by a quadratic.
impl Quadratic {
    /// Quadratic surface `ax^2 + 2bxy + 2cxz + 2dx + ey^2 + 2fyz + 2gy + hz^2 + 2iz + j = 0`.
    pub fn new(
        a: f32,
        b: f32,
        c: f32,
        d: f32,
        e: f32,
        f: f32,
        g: f32,
        h: f32,
        i: f32,
        j: f32,
    ) -> Self {
        Self {
            base: BaseObject::new(),
            a,
            b,
            c,
            d,
            e,
            f,
            g,
            h,
            i,
            j,
        }
    }

    fn add_hit(&mut self, hitpool: &mut HitPool, ray: &Ray, t: f32, entering: bool) {
        let hit_position: Vertex = ray.position + t * ray.direction;
        let mut hit_normal = Vector::new(
            self.a * hit_position.vector.x
                + self.b * hit_position.vector.y
                + self.c * hit_position.vector.z
                + self.d,
            self.b * hit_position.vector.x
                + self.e * hit_position.vector.y
                + self.f * hit_position.vector.z
                + self.g,
            self.c * hit_position.vector.x
                + self.f * hit_position.vector.y
                + self.h * hit_position.vector.z
                + self.i,
        );
        hit_normal = hit_normal.normalise();

        // Flip normal if pointing away from the surface we are looking at.
        if hit_normal.dot(&ray.direction) > 0.0 {
            hit_normal = hit_normal.negate();
        }

        hitpool.insert(Hit::new(t, entering, hit_position, hit_normal));
    }
}

impl Object for Quadratic {
    fn get_material(&self) -> Option<&Rc<dyn Material>> {
        self.base.get_material()
    }

    fn set_material(&mut self, material: Rc<dyn Material>) {
        self.base.set_material(material)
    }

    fn add_intersections(&mut self, hitpool: &mut HitPool, ray: &Ray) {
        let (dir_x, dir_y, dir_z) = (ray.direction.x, ray.direction.y, ray.direction.z);
        let (pos_x, pos_y, pos_z) = (
            ray.position.vector.x,
            ray.position.vector.y,
            ray.position.vector.z,
        );
        let a_quadratic = self.a * dir_x.powi(2)
            + 2.0 * self.b * dir_x * dir_y
            + 2.0 * self.c * dir_x * dir_z
            + self.e * dir_y.powi(2)
            + 2.0 * self.f * dir_y * dir_z
            + self.h * dir_z.powi(2);

        let b_quadratic = 2.0
            * (self.a * pos_x * dir_x
                + self.b * (pos_x * dir_y + dir_x * pos_y)
                + self.c * (pos_x * dir_z + dir_x * pos_z)
                + self.d * dir_x
                + self.e * pos_y * dir_y
                + self.f * (pos_y * dir_z + dir_y * pos_z)
                + self.g * dir_y
                + self.h * pos_z * dir_z
                + self.i * dir_z);

        let c_quadratic = self.a * pos_x.powi(2)
            + 2.0 * self.b * pos_x * pos_y
            + 2.0 * self.c * pos_x * pos_z
            + 2.0 * self.d * pos_x
            + self.e * pos_y.powi(2)
            + 2.0 * self.f * pos_y * pos_z
            + 2.0 * self.g * pos_y
            + self.h * pos_z.powi(2)
            + 2.0 * self.i * pos_z
            + self.j;

        let discriminant = b_quadratic.powi(2) - 4.0 * a_quadratic * c_quadratic;
        if discriminant < 0.0 || a_quadratic == 0.0 {
            // No intersection or ray is tangent to the surface.
            return;
        }

        let t0 = (-b_quadratic - (discriminant).sqrt()) / (2.0 * a_quadratic);
        let t1 = (-b_quadratic + (discriminant).sqrt()) / (2.0 * a_quadratic);
        if t0 < 0.0 && t1 < 0.0 {
            return;
        }

        self.add_hit(hitpool, ray, t0, true);
        self.add_hit(hitpool, ray, t1, false);
    }

    fn apply_transform(&mut self, trans: &Transform) {
        let quadratic = Transform::new([
            [self.a, self.b, self.c, self.d],
            [self.b, self.e, self.f, self.g],
            [self.c, self.f, self.h, self.i],
            [self.d, self.g, self.i, self.j],
        ]);
        let transformed_quadratic = trans.transpose() * (quadratic * *trans);

        (
            self.a, self.b, self.c, self.d, self.e, self.f, self.g, self.h, self.i, self.j,
        ) = (
            transformed_quadratic.matrix[0][0],
            transformed_quadratic.matrix[0][1],
            transformed_quadratic.matrix[0][2],
            transformed_quadratic.matrix[0][3],
            transformed_quadratic.matrix[1][1],
            transformed_quadratic.matrix[1][2],
            transformed_quadratic.matrix[1][3],
            transformed_quadratic.matrix[2][2],
            transformed_quadratic.matrix[2][3],
            transformed_quadratic.matrix[3][3],
        );
    }
}
