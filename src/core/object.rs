use std::rc::Rc;

use sortedlist_rs::SortedList;

use crate::{
    core::material::Material,
    primitives::{hit::Hit, ray::Ray, transform::Transform},
};

/// Object is the trait for objects in the environment.
pub trait Object {
    fn get_hitpool(&mut self) -> &mut SortedList<Hit>;

    fn get_material(&self) -> Option<&Rc<dyn Material>>;

    fn set_material(&mut self, material: Rc<dyn Material>);

    /// Selects the first hit (with positive distance) from the hitpool.
    /// This also clears the hitpool.
    fn select_first_hit(&mut self) -> Option<Hit>;

    /// Computes and stores the intersections of a ray with this object.
    fn add_intersections(&mut self, ray: &Ray);

    /// Applies a transformation to the object.
    fn apply_transform(&mut self, trans: &Transform);
}

pub struct BaseObject {
    pub material: Option<Rc<dyn Material>>,
    /// Sortedlist uses a list of sorted sublists to store elements.
    /// It has three internal lists:
    /// - `lists`: This is a list of sorted sublists. Each sublist contains a portion of the elements in sorted order.
    /// This allows for insertion and deletion by operating on smaller sublists rather than a single large list.
    /// - `maxes`: This list contains the maximum element of each sublist in `lists`.
    /// It is used for binary search to locate the sublist that may contain a specific element.
    /// - `index`: This is a tree of pair-wise sums of the lengths of the sublists in `lists`.
    /// It is used for indexing, allowing quick computation of the overall position of an element within the entire sortedlist.
    pub hitpool: SortedList<Hit>,
}

impl BaseObject {
    pub fn new() -> Self {
        Self {
            material: None,
            hitpool: SortedList::new(),
        }
    }
}

impl Object for BaseObject {
    fn get_hitpool(&mut self) -> &mut SortedList<Hit> {
        &mut self.hitpool
    }

    fn get_material(&self) -> Option<&Rc<dyn Material>> {
        self.material.as_ref()
    }

    fn set_material(&mut self, material: Rc<dyn Material>) {
        self.material = Some(material);
    }

    fn select_first_hit(&mut self) -> Option<Hit> {
        if let Some(index) = self
            .hitpool
            .flatten()
            .iter()
            .position(|&hit| hit.distance >= 0.0)
        {
            let hit = self.hitpool.remove(index);
            self.hitpool.clear();
            Some(hit)
        } else {
            None
        }
    }

    fn add_intersections(&mut self, _: &Ray) {}

    fn apply_transform(&mut self, _: &Transform) {}
}
