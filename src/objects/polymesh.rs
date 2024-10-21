use crate::{
    core::{
        hit::Hit,
        material::Material,
        object::{BaseObject, Object},
        ray::Ray,
        transform::Transform,
        vertex::Vertex,
    },
    utilities::obj_reader::{ObjReader, VertexAndNormalIndices},
};
use std::io;

pub struct PolyMesh {
    base: BaseObject,
    _smooth: bool,

    pub vertices: Vec<Vertex>,
    pub vertices_normals: Vec<Vertex>,

    /// A vector of triangles represented as arrays of vertex indices.
    /// Each triangle is defined by three indices corresponding to the vertices in the `vertices` vector.
    /// For example, a triangle defined as `[0, 1, 2]` refers to the first three vertices.
    pub triangles: Vec<[VertexAndNormalIndices; 3]>,
}

impl PolyMesh {
    pub fn new(file_path: &str, _smooth: bool) -> io::Result<Self> {
        let obj_reader = ObjReader::new(file_path)?;

        Ok(Self {
            base: BaseObject::new(),
            _smooth,
            vertices: obj_reader.vertices(),
            vertices_normals: obj_reader.vertices_normals(),
            triangles: obj_reader.triangles(),
        })
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
        self.base.intersection(ray)
    }

    fn apply_transform(&mut self, trans: &Transform) {
        for vertex in self.vertices.iter_mut() {
            trans.apply_to_vertex(vertex);
        }

        for vertex_normal in self.vertices_normals.iter_mut() {
            trans.apply_to_vertex(vertex_normal);
        }
    }

    fn select_first_hit(&mut self) -> Option<Hit> {
        self.base.select_first_hit()
    }
}
