use std::sync::Arc;

use sortedlist_rs::SortedList;

use crate::{
    core::material::Material,
    primitives::{hit::Hit, ray::Ray, transform::Transform},
};

pub type HitPool = SortedList<Hit>;

/// Object is the trait for objects in the environment.
pub trait Object: Sync {
    /// Computes and stores the intersections of a ray with this object.
    fn add_intersections(&self, _hitpool: &mut HitPool, _ray: &Ray) {}

    fn generate_hitpool(&self, ray: &Ray) -> HitPool {
        let mut hitpool = SortedList::new();
        self.add_intersections(&mut hitpool, ray);
        hitpool
    }

    /// Selects the first hit (with positive distance) from the hitpool.
    /// This also clears the hitpool.
    fn select_first_hit(&self, ray: &Ray) -> Option<Hit> {
        let mut hitpool = self.generate_hitpool(ray);
        if let Some(index) = hitpool
            .flatten()
            .iter()
            .position(|&hit| hit.distance >= 0.0)
        {
            let hit = hitpool.remove(index);
            hitpool.clear();
            Some(hit)
        } else {
            None
        }
    }

    fn get_material(&self) -> Option<&Arc<dyn Material>>;

    fn set_material(&mut self, material: Arc<dyn Material>);

    /// Applies a transformation to the object.
    fn apply_transform(&mut self, _trans: &Transform) {}
}

pub struct BaseObject {
    pub material: Option<Arc<dyn Material>>,
}

impl BaseObject {
    pub fn new() -> Self {
        Self { material: None }
    }
}

impl Object for BaseObject {
    fn get_material(&self) -> Option<&Arc<dyn Material>> {
        self.material.as_ref()
    }

    fn set_material(&mut self, material: Arc<dyn Material>) {
        self.material = Some(material);
    }
}
