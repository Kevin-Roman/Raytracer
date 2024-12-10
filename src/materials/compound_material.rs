use crate::{
    core::{environment::Environment, material::Material},
    primitives::{colour::Colour, hit::Hit, ray::Ray, vector::Vector},
};

/// CompoundMaterial is a Material that applies multiple materials to a surface.
pub struct CompoundMaterial {
    pub materials: Vec<Box<dyn Material>>,
}

impl CompoundMaterial {
    pub fn new(materials: Vec<Box<dyn Material>>) -> Self {
        Self { materials }
    }
}

impl Default for CompoundMaterial {
    fn default() -> Self {
        Self {
            materials: Vec::new(),
        }
    }
}

impl Material for CompoundMaterial {
    fn compute_once(
        &self,
        environment: &mut dyn Environment,
        viewer: &Ray,
        hit: &Hit,
        recurse: u8,
    ) -> Colour {
        self.materials
            .iter()
            .fold(Colour::default(), |acc, material| {
                acc + material.compute_once(environment, viewer, hit, recurse)
            })
    }

    fn compute_per_light(&self, viewer: &Vector, light_direction: &Vector, hit: &Hit) -> Colour {
        self.materials
            .iter()
            .fold(Colour::default(), |acc, material| {
                acc + material.compute_per_light(viewer, light_direction, hit)
            })
    }
}
