use crate::primitives::{colour::Colour, hit::Hit, ray::Ray, vector::Vector};

pub trait Material {
    fn compute_once(
        &self,
        environment: &mut dyn Environment,
        viewer: &Ray,
        hit: &Hit,
        recurse: u8,
    ) -> Colour;

    fn compute_per_light(&self, viewer: &Vector, light_direction: &Vector, hit: &Hit) -> Colour;
}
