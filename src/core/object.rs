use super::{hit::Hit, material::Material, ray::Ray, transform::Transform};

// Object is the base trait for objects.
pub trait Object {
    // Specify the material this object uses.
    fn set_material(&mut self, material: Option<Box<dyn Material>>);

    // Given a ray, if this object intersects it, return all points of intersection.
    // Return None if no intersections.
    fn intersection(&mut self, ray: &Ray);

    // Apply a transform to this object.
    fn apply_transform(&mut self, trans: &Transform);
}

pub struct BaseObject {
    pub material: Option<Box<dyn Material>>,
    pub hitpool: Vec<Hit>,
}

impl BaseObject {
    pub fn new() -> Self {
        Self {
            material: None,
            hitpool: Vec::new(),
        }
    }
}

impl Object for BaseObject {
    fn set_material(&mut self, material: Option<Box<dyn Material>>) {
        self.material = material;
    }

    fn intersection(&mut self, _: &Ray) {}

    fn apply_transform(&mut self, _: &Transform) {}
}
