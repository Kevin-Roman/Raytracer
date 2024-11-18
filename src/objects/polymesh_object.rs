use std::{io, rc::Rc};

use sortedlist_rs::SortedList;

use crate::{
    core::{
        hit::Hit,
        material::Material,
        object::{BaseObject, Object},
        ray::Ray,
        transform::Transform,
        vertex::Vertex,
    },
    utilities::obj_reader::{ObjReader, Triangle},
};

const EPSILON: f32 = 0.000001;

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
            vertices: obj_reader.vertices(),
            vertex_normals: obj_reader.vertex_normals(),
            triangles: obj_reader.triangles(),
        })
    }

    fn add_hit(
        &mut self,
        triangle_index: usize,
        ray: &Ray,
        t: f32,
        u: f32,
        v: f32,
        entering: bool,
    ) {
        let hit_position = ray.position + t * ray.direction;

        let mut hit_normal = if self.smooth {
            let triangle = &self.triangles[triangle_index];

            // Interpolate using N(t) = (1 - u - v)N_0 + uN_1 + vN_2
            // to get the normal at the specific point inside the triangle.
            (1.0 - u - v) * self.vertex_normals[triangle.vertex_normal_indices[0]].vector
                + u * self.vertex_normals[triangle.vertex_normal_indices[1]].vector
                + v * self.vertex_normals[triangle.vertex_normal_indices[2]].vector
        } else {
            self.triangles[triangle_index].face_normal
        };

        hit_normal = hit_normal.normalise();

        // Flip normal if pointing away from the surface we are looking at.
        if hit_normal.dot(&ray.direction) > 0.0 {
            hit_normal = hit_normal.negate();
        }

        self.base
            .hitpool
            .insert(Hit::new(t, entering, hit_position, hit_normal));
    }

    /// Triangle intersection using the `Möller–Trumbore intersection algorithm`.
    /// Non-culling.
    ///
    /// Tomas Möller and Ben Trumbore. 2005. Fast, minimum storage ray/triangle intersection.
    /// In ACM SIGGRAPH 2005 Courses (SIGGRAPH '05). Association for Computing Machinery,
    /// New York, NY, USA, 7–es. https://doi.org/10.1145/1198555.1198746
    fn triangle_intersection(
        &self,
        ray: &Ray,
        triangle_index: usize,
    ) -> Option<((f32, f32, f32), bool)> {
        let triangle = &self.triangles[triangle_index];
        let vert0 = self.vertices[triangle.vertex_indices[0]];
        let vert1 = self.vertices[triangle.vertex_indices[1]];
        let vert2 = self.vertices[triangle.vertex_indices[2]];

        let edge1 = vert1.vector - vert0.vector;
        let edge2 = vert2.vector - vert0.vector;

        let p_vec = ray.direction.cross(&edge2);
        let det = edge1.dot(&p_vec);

        if -EPSILON < det && det < EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;

        let t_vec = ray.position.vector - vert0.vector;

        let u = t_vec.dot(&p_vec) * inv_det;
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q_vec = t_vec.cross(&edge1);

        let v = ray.direction.dot(&q_vec) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = edge2.dot(&q_vec) * inv_det;

        // Negative determinant indicates a back facing triangle.
        let entering = det >= 0.0;
        return Some(((t, u, v), entering));
    }
}

impl Object for PolyMesh {
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
        // For each triangle in the model.
        for i in 0..self.triangles.len() {
            if let Some(((t, u, v), entering)) = self.triangle_intersection(ray, i) {
                self.add_hit(i, ray, t, u, v, entering);
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
}
