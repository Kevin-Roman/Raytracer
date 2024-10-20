use super::{hit::Hit, material::Material, ray::Ray, transform::Transform};

// Object is the base trait for objects.
pub trait Object {
    fn get_material(&self) -> Option<&Box<dyn Material>>;

    // Specify the material this object uses.
    fn set_material(&mut self, material: Box<dyn Material>);

    // Given a ray, if this object intersects it, return all points of intersection.
    // Return None if no intersections.
    fn intersection(&mut self, ray: &Ray);

    // Apply a transform to this object.
    fn apply_transform(&mut self, trans: &Transform);

    // Retrieve the first valid hit.
    fn select_first_hit(&mut self) -> Option<Hit>;
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
    fn get_material(&self) -> Option<&Box<dyn Material>> {
        self.material.as_ref()
    }

    fn set_material(&mut self, material: Box<dyn Material>) {
        self.material = Some(material);
    }

    fn intersection(&mut self, _: &Ray) {}

    fn apply_transform(&mut self, _: &Transform) {}

    fn select_first_hit(&mut self) -> Option<Hit> {
        if let Some(index) = self.hitpool.iter().position(|&hit| hit.t >= 0.0) {
            let hit = self.hitpool.swap_remove(index);
            self.hitpool.clear();
            Some(hit)
        } else {
            None
        }
    }
}
