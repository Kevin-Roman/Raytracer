use crate::core::{
    hit::Hit,
    material::Material,
    object::{BaseObject, Object},
    ray::Ray,
    transform::Transform,
};

pub struct PolyMesh {
    base: BaseObject,
    smooth: bool,
}

impl PolyMesh {
    pub fn new(_file: &str, smooth: bool) -> Self {
        Self {
            base: BaseObject::new(),
            smooth,
        }
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
