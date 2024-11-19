use crate::{
    core::environment::Environment,
    primitives::{colour::Colour, hit::Hit, ray::Ray, vector::Vector},
};

/// Material is the trait for object materials.
pub trait Material {
    /// Compute the colour at the ray intersection with the object
    fn compute_once(
        &self,
        environment: &mut dyn Environment,
        viewer: &Ray,
        hit: &Hit,
        recurse: u8,
    ) -> Colour;

    /// Compute the colour at the ray intersection with the object, taking into consideration the light.
    fn compute_per_light(&self, viewer: &Vector, light_direction: &Vector, hit: &Hit) -> Colour;
}
