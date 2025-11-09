use std::{io, sync::Arc};

use crate::{
    core::{
        material::Material,
        object::{BaseObject, HitPool, Object},
    },
    primitives::{hit::Hit, ray::Ray, transform::Transform, vector::Vector, vertex::Vertex},
    utilities::obj_reader::{ObjReader, Triangle},
};

const EPSILON: f32 = 0.000001;

struct IntersectionData {
    t: f32,
    u: f32,
    v: f32,
    entering: bool,
}

pub struct PolyMesh {
    base: BaseObject,
    smooth: bool,

    pub vertices: Vec<Vertex>,
    pub vertex_normals: Vec<Vertex>,
    pub triangles: Vec<Triangle>,
}

impl PolyMesh {
    pub fn new(file_path: &str, smooth: bool) -> io::Result<Self> {
        let obj_reader = ObjReader::new(file_path)?;

        Ok(Self {
            base: BaseObject::new(),
            smooth,
            vertices: obj_reader.vertices().to_vec(),
            vertex_normals: obj_reader.vertex_normals().to_vec(),
            triangles: obj_reader.triangles(),
        })
    }

    fn add_hit(
        &self,
        hitpool: &mut HitPool,
        triangle_index: usize,
        ray: &Ray,
        intersection: IntersectionData,
    ) {
        let hit_position = ray.position + intersection.t * ray.direction;

        let mut hit_normal = if self.smooth {
            let triangle = &self.triangles[triangle_index];

            // Interpolate using N(t) = (1 - u - v)N_0 + uN_1 + vN_2
            // to get the normal at the specific point inside the triangle.
            (1.0 - intersection.u - intersection.v)
                * self.vertex_normals[triangle.vertex_normal_indices[0]].vector
                + intersection.u * self.vertex_normals[triangle.vertex_normal_indices[1]].vector
                + intersection.v * self.vertex_normals[triangle.vertex_normal_indices[2]].vector
        } else {
            self.triangles[triangle_index].face_normal
        };

        hit_normal = hit_normal.normalise();

        // Flip normal if pointing away from the surface we are looking at.
        if hit_normal.dot(ray.direction) > 0.0 {
            hit_normal = hit_normal.negate();
        }

        hitpool.insert(Hit::new(
            intersection.t,
            intersection.entering,
            hit_position,
            hit_normal,
        ));
    }

    /// Triangle intersection using the `Möller–Trumbore intersection algorithm`.
    /// Non-culling.
    ///
    /// Tomas Möller and Ben Trumbore. 2005. Fast, minimum storage ray/triangle intersection.
    /// In ACM SIGGRAPH 2005 Courses (SIGGRAPH '05). Association for Computing Machinery,
    /// New York, NY, USA, 7–es. https://doi.org/10.1145/1198555.1198746
    fn triangle_intersection(&self, ray: &Ray, triangle_index: usize) -> Option<IntersectionData> {
        // Retrieve the triangle and its vertices.
        let triangle = &self.triangles[triangle_index];
        let vert0 = &self.vertices[triangle.vertex_indices[0]];
        let vert1 = &self.vertices[triangle.vertex_indices[1]];
        let vert2 = &self.vertices[triangle.vertex_indices[2]];

        // Calculate the edges of the triangle.
        let edge1 = vert1.vector - vert0.vector;
        let edge2 = vert2.vector - vert0.vector;

        // Calculate the determinant.
        let p_vec = ray.direction.cross(edge2);
        let det = edge1.dot(p_vec);

        // If the determinant is near zero, the ray lies in the plane of the triangle.
        if -EPSILON < det && det < EPSILON {
            return None;
        }

        // Calculate the inverse of the determinant.
        let inv_det = 1.0 / det;

        let t_vec = ray.position.vector - vert0.vector;

        // Test bounds.
        let u = t_vec.dot(p_vec) * inv_det;
        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let q_vec = t_vec.cross(edge1);

        // Test bounds for the barycentric coordinate.
        let v = ray.direction.dot(q_vec) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        // Calculate the distance from the ray origin to the intersection point.
        let t = edge2.dot(q_vec) * inv_det;

        // Determine if the intersection is entering or exiting the triangle.
        let entering = det < 0.0;

        Some(IntersectionData { t, u, v, entering })
    }
}

impl Object for PolyMesh {
    fn get_material(&self) -> Option<&Arc<dyn Material>> {
        self.base.get_material()
    }

    fn set_material(&mut self, material: Arc<dyn Material>) {
        self.base.set_material(material)
    }

    fn add_intersections(&self, hitpool: &mut HitPool, ray: &Ray) {
        // For each triangle in the model.
        for i in 0..self.triangles.len() {
            if let Some(intersection) = self.triangle_intersection(ray, i) {
                self.add_hit(hitpool, i, ray, intersection);
            }
        }
    }

    fn apply_transform(&mut self, trans: &Transform) {
        for vertex in &mut self.vertices {
            trans.apply_to_vertex(vertex);
        }

        for vertex_normal in &mut self.vertex_normals {
            // 1. Inverse - undo scaling.
            // 2. Transpose - preserve perpendicular relationship to the surface.
            trans.inverse().transpose().apply_to_vertex(vertex_normal);
        }

        for triangle in &mut self.triangles {
            trans
                .inverse()
                .transpose()
                .apply_to_vector(&mut triangle.face_normal)
        }
    }

    fn bounding_sphere(&self) -> Option<(Vertex, f32)> {
        // Get min and max for each all components:
        let min = self
            .vertices
            .iter()
            .fold(self.vertices[0].vector, |acc, vertex| {
                Vector::new(
                    acc.x.min(vertex.vector.x),
                    acc.y.min(vertex.vector.y),
                    acc.z.min(vertex.vector.z),
                )
            });

        let max = self
            .vertices
            .iter()
            .fold(self.vertices[0].vector, |acc, vertex| {
                Vector::new(
                    acc.x.max(vertex.vector.x),
                    acc.y.max(vertex.vector.y),
                    acc.z.max(vertex.vector.z),
                )
            });

        let center = (min + max) / 2.0;

        Some((
            Vertex::new(center.x, center.y, center.z, 1.0),
            self.vertices
                .iter()
                .map(|vertex| (vertex.vector - center).length())
                .fold(0.0, |acc, length| acc.max(length)),
        ))
    }
}
