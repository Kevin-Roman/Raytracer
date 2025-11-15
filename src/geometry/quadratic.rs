use crate::{
    geometry::traits::{HitPool, Intersection, Transformable},
    primitives::{ray::Ray, Hit, Transform, Vector, Vertex},
    shading::scene_material::MaterialId,
};

/// Coefficients for a quadratic surface: ax² + 2bxy + 2cxz + 2dx + ey² + 2fyz + 2gy + hz² + 2iz + j = 0
#[derive(Clone, Copy, Debug)]
pub struct QuadraticCoefficients {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
    pub g: f32,
    pub h: f32,
    pub i: f32,
    pub j: f32,
}

/// Quadratic surface geometry - pure data structure
#[derive(Clone, Copy, Debug)]
pub struct QuadraticGeometry {
    coeffs: QuadraticCoefficients,
}

impl QuadraticGeometry {
    /// Quadratic surface `ax^2 + 2bxy + 2cxz + 2dx + ey^2 + 2fyz + 2gy + hz^2 + 2iz + j = 0`.
    pub fn new(coefficients: QuadraticCoefficients) -> Self {
        Self {
            coeffs: coefficients,
        }
    }

    fn add_hit(&self, hitpool: &mut HitPool, ray: &Ray, t: f32, entering: bool) {
        let hit_position: Vertex = ray.position + t * ray.direction;
        let mut hit_normal = Vector::new(
            self.coeffs.a * hit_position.vector.x
                + self.coeffs.b * hit_position.vector.y
                + self.coeffs.c * hit_position.vector.z
                + self.coeffs.d,
            self.coeffs.b * hit_position.vector.x
                + self.coeffs.e * hit_position.vector.y
                + self.coeffs.f * hit_position.vector.z
                + self.coeffs.g,
            self.coeffs.c * hit_position.vector.x
                + self.coeffs.f * hit_position.vector.y
                + self.coeffs.h * hit_position.vector.z
                + self.coeffs.i,
        );
        hit_normal = hit_normal.normalise();

        // Flip normal if pointing away from the surface we are looking at.
        if hit_normal.dot(ray.direction) > 0.0 {
            hit_normal = hit_normal.negate();
        }

        hitpool.insert(Hit::new(t, entering, hit_position, hit_normal));
    }
}

impl Intersection for QuadraticGeometry {
    fn intersect(&self, ray: &Ray, hitpool: &mut HitPool) {
        let (dir_x, dir_y, dir_z) = (ray.direction.x, ray.direction.y, ray.direction.z);
        let (pos_x, pos_y, pos_z) = (
            ray.position.vector.x,
            ray.position.vector.y,
            ray.position.vector.z,
        );
        let a_quadratic = self.coeffs.a * dir_x.powi(2)
            + 2.0 * self.coeffs.b * dir_x * dir_y
            + 2.0 * self.coeffs.c * dir_x * dir_z
            + self.coeffs.e * dir_y.powi(2)
            + 2.0 * self.coeffs.f * dir_y * dir_z
            + self.coeffs.h * dir_z.powi(2);
        let b_quadratic = 2.0
            * (self.coeffs.a * pos_x * dir_x
                + self.coeffs.b * (pos_x * dir_y + dir_x * pos_y)
                + self.coeffs.c * (pos_x * dir_z + dir_x * pos_z)
                + self.coeffs.d * dir_x
                + self.coeffs.e * pos_y * dir_y
                + self.coeffs.f * (pos_y * dir_z + dir_y * pos_z)
                + self.coeffs.g * dir_y
                + self.coeffs.h * pos_z * dir_z
                + self.coeffs.i * dir_z);

        let c_quadratic = self.coeffs.a * pos_x.powi(2)
            + 2.0 * self.coeffs.b * pos_x * pos_y
            + 2.0 * self.coeffs.c * pos_x * pos_z
            + 2.0 * self.coeffs.d * pos_x
            + self.coeffs.e * pos_y.powi(2)
            + 2.0 * self.coeffs.f * pos_y * pos_z
            + 2.0 * self.coeffs.g * pos_y
            + self.coeffs.h * pos_z.powi(2)
            + 2.0 * self.coeffs.i * pos_z
            + self.coeffs.j;

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
}

impl Transformable for QuadraticGeometry {
    fn transform(&mut self, trans: &Transform) {
        let quadratic = Transform::new([
            [self.coeffs.a, self.coeffs.b, self.coeffs.c, self.coeffs.d],
            [self.coeffs.b, self.coeffs.e, self.coeffs.f, self.coeffs.g],
            [self.coeffs.c, self.coeffs.f, self.coeffs.h, self.coeffs.i],
            [self.coeffs.d, self.coeffs.g, self.coeffs.i, self.coeffs.j],
        ]);
        let transformed_quadratic = trans.transpose() * (quadratic * *trans);

        (
            self.coeffs.a,
            self.coeffs.b,
            self.coeffs.c,
            self.coeffs.d,
            self.coeffs.e,
            self.coeffs.f,
            self.coeffs.g,
            self.coeffs.h,
            self.coeffs.i,
            self.coeffs.j,
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

#[derive(Debug)]
pub struct Quadratic {
    pub geometry: QuadraticGeometry,
    pub material_id: MaterialId,
}

impl Quadratic {
    pub fn new(coefficients: QuadraticCoefficients) -> Self {
        Self {
            geometry: QuadraticGeometry::new(coefficients),
            material_id: MaterialId::default(),
        }
    }

    pub fn with_material(mut self, material_id: MaterialId) -> Self {
        self.material_id = material_id;
        self
    }
}

impl Intersection for Quadratic {
    fn intersect(&self, ray: &Ray, hitpool: &mut HitPool) {
        self.geometry.intersect(ray, hitpool)
    }
}

impl Transformable for Quadratic {
    fn transform(&mut self, trans: &Transform) {
        self.geometry.transform(trans)
    }
}
