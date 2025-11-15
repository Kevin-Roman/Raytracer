use sortedlist_rs::SortedList;

use crate::primitives::{ray::Ray, Hit, Transform, Vertex};

/// Sortedlist uses a list of sorted sublists to store elements.
/// It has three internal lists:
/// - `lists`: This is a list of sorted sublists. Each sublist contains a portion of the elements in sorted order.
///   This allows for insertion and deletion by operating on smaller sublists rather than a single large list.
/// - `maxes`: This list contains the maximum element of each sublist in `lists`.
///   It is used for binary search to locate the sublist that may contain a specific element.
/// - `index`: This is a tree of pair-wise sums of the lengths of the sublists in `lists`.
///   It is used for indexing, allowing quick computation of the overall position of an element within the entire sortedlist.
pub type HitPool = SortedList<Hit>;

/// Intersection trait that is focused on ray-object intersection calculations only.
pub trait Intersection {
    /// Computes and stores the intersections of a ray with this object.
    fn intersect(&self, ray: &Ray, hitpool: &mut HitPool);

    fn generate_hitpool(&self, ray: &Ray) -> HitPool {
        let mut hitpool = SortedList::new();
        self.intersect(ray, &mut hitpool);
        hitpool
    }

    /// Selects the first hit (with positive distance) from the hitpool.
    fn first_hit(&self, ray: &Ray) -> Option<Hit> {
        let mut hitpool = self.generate_hitpool(ray);
        if let Some(index) = hitpool.flatten().iter().position(|&hit| hit.distance > 0.0) {
            let hit = hitpool.remove(index);
            hitpool.clear();
            Some(hit)
        } else {
            None
        }
    }
}

/// Transformable trait for objects that can undergo geometric transformations.
pub trait Transformable {
    /// Applies a transformation to the object.
    fn transform(&mut self, trans: &Transform);
}

/// Bounded trait for objects with bounding volumes.
pub trait Bounded {
    /// Returns the bounding sphere (center, radius) if available.
    fn bounding_sphere(&self) -> Option<(Vertex, f32)>;
}
