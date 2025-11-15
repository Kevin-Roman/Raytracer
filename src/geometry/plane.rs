use crate::{
    geometry::traits::{HitPool, Intersection, Transformable},
    primitives::{ray::Ray, Hit, Transform, Vector, Vertex},
    shading::scene_material::MaterialId,
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
    pub material_id: MaterialId,
}

impl Plane {
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Self {
            geometry: PlaneGeometry::new(a, b, c, d),
            material_id: MaterialId::default(),
        }
    }

    pub fn with_material(mut self, material_id: MaterialId) -> Self {
        self.material_id = material_id;
        self
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
