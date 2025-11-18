use crate::{
    geometry::traits::{HitPool, Intersection, Transformable},
    primitives::{ray::Ray, Hit, Transform},
    shading::scene_material::MaterialId,
};

use super::scene_object::SceneObject;

#[derive(Clone, Copy, Debug)]
pub enum Mode {
    CsgUnion = 0,
    CsgInter,
    CsgDiff,
}

#[allow(clippy::enum_variant_names)]
enum Action {
    CsgAEnter,
    CsgAExit,
    CsgADrop,
    CsgBEnter,
    CsgBExit,
    CsgBDrop,
}

const ACTIONS: [[Action; 8]; 3] = [
    [
        Action::CsgADrop,
        Action::CsgBDrop,
        Action::CsgAExit,
        Action::CsgBDrop,
        Action::CsgADrop,
        Action::CsgBExit,
        Action::CsgAEnter,
        Action::CsgBEnter,
    ],
    [
        Action::CsgAExit,
        Action::CsgBExit,
        Action::CsgADrop,
        Action::CsgBEnter,
        Action::CsgAEnter,
        Action::CsgBDrop,
        Action::CsgADrop,
        Action::CsgBDrop,
    ],
    [
        Action::CsgADrop,
        Action::CsgBEnter,
        Action::CsgAExit,
        Action::CsgBExit,
        Action::CsgADrop,
        Action::CsgBDrop,
        Action::CsgAEnter,
        Action::CsgBDrop,
    ],
];

/// Constructive solid geometry.
///
/// CSG is an object that is built by Constructive Solid Geometry from two sub-objects.
/// It supports three operations Union, Intersection and Difference of the two sub-objects.
///
/// Uses composition - the CSG owns its child objects directly.
#[derive(Debug)]
pub struct CSG {
    pub mode: Mode,
    pub left_object: SceneObject,
    pub right_object: SceneObject,
    pub material_id: MaterialId,
}

impl CSG {
    pub fn new(mode: Mode, left_object: SceneObject, right_object: SceneObject) -> Self {
        Self {
            mode,
            left_object,
            right_object,
            material_id: MaterialId::default(),
        }
    }

    pub fn with_material(mut self, material_id: MaterialId) -> Self {
        self.material_id = material_id;
        self
    }
}

impl Intersection for CSG {
    fn intersect(&self, ray: &Ray, hitpool: &mut HitPool) {
        let mut result: Vec<Hit> = Vec::new();

        let mut left_hitpool = self.left_object.generate_hitpool(ray);
        let mut right_hitpool = self.right_object.generate_hitpool(ray);

        let mut left_index = 0;
        let mut right_index = 0;
        while left_index < left_hitpool.len() && right_index < right_hitpool.len() {
            let mut state = 0;

            if left_hitpool[left_index].entering {
                state += 4;
            }
            if right_hitpool[right_index].entering {
                state += 2;
            }
            if left_hitpool[left_index].distance > right_hitpool[right_index].distance {
                state += 1;
            }

            match ACTIONS[self.mode as usize][state] {
                Action::CsgAEnter => {
                    result.push(left_hitpool[left_index]);
                    result.last_mut().unwrap().entering = true;
                    left_index += 1;
                }
                Action::CsgAExit => {
                    result.push(left_hitpool[left_index]);
                    result.last_mut().unwrap().entering = false;
                    left_index += 1;
                }
                Action::CsgADrop => {
                    left_index += 1;
                }
                Action::CsgBEnter => {
                    result.push(right_hitpool[right_index]);
                    result.last_mut().unwrap().entering = true;
                    right_index += 1;
                }
                Action::CsgBExit => {
                    result.push(right_hitpool[right_index]);
                    result.last_mut().unwrap().entering = false;
                    right_index += 1;
                }
                Action::CsgBDrop => {
                    right_index += 1;
                }
            }
        }

        match self.mode {
            Mode::CsgDiff => {
                if left_index < left_hitpool.len() {
                    result.extend(left_hitpool.flatten().iter().skip(left_index).cloned());
                }
            }
            Mode::CsgUnion => {
                if left_index >= left_hitpool.len() {
                    result.extend(right_hitpool.flatten().iter().skip(right_index).cloned());
                } else {
                    result.extend(left_hitpool.flatten().iter().skip(left_index).cloned());
                }
            }
            Mode::CsgInter => {}
        }

        left_hitpool.clear();
        right_hitpool.clear();

        for hit in result {
            hitpool.insert(hit);
        }
    }
}

impl Transformable for CSG {
    fn transform(&mut self, trans: &Transform) {
        self.left_object.transform(trans);
        self.right_object.transform(trans);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{geometry::sphere::Sphere, primitives::Vertex};

    #[test]
    fn test_csg_union_creation() {
        let sphere1 = Sphere::new(Vertex::new(0.0, 0.0, 0.0, 1.0), 1.0);
        let sphere2 = Sphere::new(Vertex::new(2.0, 0.0, 0.0, 1.0), 1.0);

        let csg = CSG::new(Mode::CsgUnion, sphere1.into(), sphere2.into());

        match csg.mode {
            Mode::CsgUnion => {}
            _ => panic!("Expected CsgUnion mode"),
        }
    }

    #[test]
    fn test_csg_intersection_creation() {
        let sphere1 = Sphere::new(Vertex::new(0.0, 0.0, 0.0, 1.0), 1.0);
        let sphere2 = Sphere::new(Vertex::new(2.0, 0.0, 0.0, 1.0), 1.0);

        let csg = CSG::new(Mode::CsgInter, sphere1.into(), sphere2.into());

        match csg.mode {
            Mode::CsgInter => {}
            _ => panic!("Expected CsgInter mode"),
        }
    }

    #[test]
    fn test_csg_difference_creation() {
        let sphere1 = Sphere::new(Vertex::new(0.0, 0.0, 0.0, 1.0), 1.0);
        let sphere2 = Sphere::new(Vertex::new(2.0, 0.0, 0.0, 1.0), 1.0);

        let csg = CSG::new(Mode::CsgDiff, sphere1.into(), sphere2.into());

        match csg.mode {
            Mode::CsgDiff => {}
            _ => panic!("Expected CsgDiff mode"),
        }
    }
}
