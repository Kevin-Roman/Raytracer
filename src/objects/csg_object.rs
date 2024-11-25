use std::sync::Arc;

use crate::{
    core::{
        material::Material,
        object::{BaseObject, HitPool, Object},
    },
    primitives::{hit::Hit, ray::Ray, transform::Transform},
};

#[derive(Clone, Copy, Debug)]
pub enum Mode {
    CsgUnion = 0,
    CsgInter,
    CsgDiff,
}

pub enum Action {
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
pub struct CSG {
    base: BaseObject,
    mode: Mode,
    left_object: Box<dyn Object>,
    right_object: Box<dyn Object>,
}

impl CSG {
    pub fn new(mode: Mode, left_object: Box<dyn Object>, right_object: Box<dyn Object>) -> Self {
        Self {
            base: BaseObject::new(),
            mode,
            left_object,
            right_object,
        }
    }
}

impl Object for CSG {
    fn get_material(&self) -> Option<&Arc<dyn Material>> {
        self.base.get_material()
    }

    fn set_material(&mut self, material: Arc<dyn Material>) {
        self.base.set_material(material)
    }

    fn add_intersections(&self, hitpool: &mut HitPool, ray: &Ray) {
        let mut result: Option<Hit> = None;

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
                    if result.is_none() {
                        result = Some(left_hitpool[left_index]);
                    }
                    left_index += 1;
                }
                Action::CsgAExit => {
                    if result.is_none() {
                        result = Some(left_hitpool[left_index]);
                    }
                    left_index += 1;
                }
                Action::CsgADrop => {
                    left_index += 1;
                }
                Action::CsgBEnter => {
                    if result.is_none() {
                        result = Some(right_hitpool[right_index]);
                    }
                    right_index += 1;
                }
                Action::CsgBExit => {
                    if result.is_none() {
                        result = Some(right_hitpool[right_index]);
                    }
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
                    if result.is_none() {
                        result = Some(left_hitpool[left_index]);
                    }
                }
            }
            Mode::CsgUnion => {
                if left_index >= left_hitpool.len() {
                    if result.is_none() && right_index < right_hitpool.len() {
                        result = Some(right_hitpool[right_index]);
                    }
                } else {
                    if result.is_none() && left_index < left_hitpool.len() {
                        result = Some(left_hitpool[left_index]);
                    }
                }
            }
            Mode::CsgInter => {}
        }

        left_hitpool.clear();
        right_hitpool.clear();

        if let Some(hit) = result {
            hitpool.insert(hit);
        }
    }

    fn apply_transform(&mut self, trans: &Transform) {
        self.left_object.apply_transform(trans);
        self.right_object.apply_transform(trans);
    }
}
