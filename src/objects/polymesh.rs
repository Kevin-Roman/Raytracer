use crate::{
    core::{
        hit::Hit,
        material::Material,
        object::{BaseObject, Object},
        ray::Ray,
        tex_coords::TexCoords,
        transform::Transform,
        vector::Vector,
        vertex::Vertex,
    },
    utilities::obj_reader::{ObjReader, VertexAndNormalIndices},
};
use std::{f32::consts::E, io};

const EPSILON: f32 = 0.000001;

pub struct PolyMesh {
    base: BaseObject,
    smooth: bool,

    pub vertices: Vec<Vertex>,
    pub vertices_normals: Vec<Vertex>,

    /// A vector of triangles represented as arrays of vertex indices.
    /// Each triangle is defined by three indices corresponding to the vertices in the `vertices` vector.
    /// For example, a triangle defined as `[0, 1, 2]` refers to the first three vertices.
    pub triangles: Vec<[VertexAndNormalIndices; 3]>,
}

impl PolyMesh {
    pub fn new(file_path: &str, smooth: bool) -> io::Result<Self> {
        let obj_reader = ObjReader::new(file_path)?;

        Ok(Self {
            base: BaseObject::new(),
            smooth,
            vertices: obj_reader.vertices(),
            vertices_normals: obj_reader.vertices_normals(),
            triangles: obj_reader.triangles(),
        })
    }

    /// Face normal of a triangle is the normalised cross product of its two edge vectors.
    fn face_normal(&self, triangle_index: usize) -> Vector {
        let triangle = self.triangles[triangle_index];

        let vert0 = self.vertices[triangle[0].v_i];
        let vert1 = self.vertices[triangle[1].v_i];
        let vert2 = self.vertices[triangle[2].v_i];

        let edge1 = vert1.vector - vert0.vector;
        let edge2 = vert2.vector - vert0.vector;

        return edge1.cross(&edge2).normalise();
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
            let triangle = self.triangles[triangle_index];

            // Interpolate using N(t) = (1 - u - v)N_0 + uN_1 + vN_2
            // to get the normal at the specific point inside the triangle.
            (1.0 - u - v) * self.vertices_normals[triangle[0].vn_i].vector
                + u * self.vertices_normals[triangle[1].vn_i].vector
                + v * self.vertices_normals[triangle[2].vn_i].vector
        } else {
            self.face_normal(triangle_index)
        };

        hit_normal = hit_normal.normalise();

        // Orient normal to point outwards.
        if hit_normal.dot(&ray.direction) > 0.0 {
            hit_normal = hit_normal.negate();
        }

        self.base.hitpool.insert(Hit::new(
            t,
            entering,
            hit_position,
            hit_normal,
            TexCoords::default(),
        ));
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
        let triangle = self.triangles[triangle_index];
        let vert0 = self.vertices[triangle[0].v_i];
        let vert1 = self.vertices[triangle[1].v_i];
        let vert2 = self.vertices[triangle[2].v_i];

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
    fn get_material(&self) -> Option<&Box<dyn Material>> {
        self.base.get_material()
    }

    fn set_material(&mut self, material: Box<dyn Material>) {
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
        for vertex in self.vertices.iter_mut() {
            trans.apply_to_vertex(vertex);
        }

        for vertex_normal in self.vertices_normals.iter_mut() {
            // 1. Inverse - undo scaling.
            // 2. Transpose - preserve perpendicular relationship to the surface.
            trans.inverse().transpose().apply_to_vertex(vertex_normal);
        }
    }

    fn select_first_hit(&mut self) -> Option<Hit> {
        self.base.select_first_hit()
    }
}
