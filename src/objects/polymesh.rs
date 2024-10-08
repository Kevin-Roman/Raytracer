use crate::{
    core::{
        hit::Hit,
        material::Material,
        object::{BaseObject, Object},
        ray::Ray,
        transform::Transform,
        vertex::Vertex,
    },
    utilities::obj_reader::ObjReader,
};
use std::io;

pub struct PolyMesh {
    base: BaseObject,
    smooth: bool,

    pub vertices: Vec<Vertex>,

    /// A vector of triangles represented as arrays of vertex indices.
    /// Each triangle is defined by three indices corresponding to the vertices in the `vertices` vector.
    /// For example, a triangle defined as `[0, 1, 2]` refers to the first three vertices.
    pub triangles: Vec<[usize; 3]>,
}

impl PolyMesh {
    pub fn new(file_path: &str, smooth: bool) -> io::Result<Self> {
        let obj_reader = ObjReader::new(file_path)?;

        Ok(Self {
            base: BaseObject::new(),
            smooth,
            vertices: obj_reader.vertices(),
            triangles: obj_reader.triangles(),
        })
    }
}

impl Object for PolyMesh {
    fn set_material(&mut self, material: Option<Box<dyn Material>>) {
        self.base.set_material(material);
    }

    fn intersection(&self, ray: Ray) -> Option<Hit> {
        self.base.intersection(ray)
    }

    fn apply_transform(&mut self, trans: &Transform) {
        self.base.apply_transform(trans);
    }
}
