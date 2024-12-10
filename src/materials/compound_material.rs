// CompoundMaterial is a Material that applies multiple other materials to a surface.
// it can be used to combine a PhongMaterial and a GlobalMaterial on a single surface.

use crate::core::{
    colour::Colour, environment::Environment, hit::Hit, material::Material, ray::Ray,
    vector::Vector,
};

pub struct CompoundMaterial {
    materials: Vec<Box<dyn Material>>,
}

impl CompoundMaterial {
    pub fn new(materials: Vec<Box<dyn Material>>) -> Self {
        Self { materials }
    }

    pub fn include_material(&mut self, material: Box<dyn Material>) {
        self.materials.push(material);
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
        recurse: i32,
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
