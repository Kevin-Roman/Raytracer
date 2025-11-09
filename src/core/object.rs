use std::sync::Arc;

use sortedlist_rs::SortedList;

use crate::{
    core::material::Material,
    primitives::{hit::Hit, ray::Ray, transform::Transform, vertex::Vertex},
};

/// Sortedlist uses a list of sorted sublists to store elements.
/// It has three internal lists:
/// - `lists`: This is a list of sorted sublists. Each sublist contains a portion of the elements in sorted order.
///   This allows for insertion and deletion by operating on smaller sublists rather than a single large list.
/// - `maxes`: This list contains the maximum element of each sublist in `lists`.
///   It is used for binary search to locate the sublist that may contain a specific element.
/// - `index`: This is a tree of pair-wise sums of the lengths of the sublists in `lists`.
///   It is used for indexing, allowing quick computation of the overall position of an element within the entire sortedlist.
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
        if let Some(index) = hitpool.flatten().iter().position(|&hit| hit.distance > 0.0) {
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

    fn bounding_sphere(&self) -> Option<(Vertex, f32)> {
        None
    }
}

pub struct BaseObject {
    pub material: Option<Arc<dyn Material>>,
}

impl Default for BaseObject {
    fn default() -> Self {
        Self::new()
    }
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
