use std::f32::consts::PI;

use crate::{
    core::environment::Environment,
    primitives::{colour::Colour, hit::Hit, ray::Ray, vector::Vector},
};

/// Material is the trait for object materials.
pub trait Material: Sync + Send {
    /// Compute the colour at the ray intersection with the object
    fn compute_once(
        &self,
        environment: &dyn Environment,
        viewer: &Ray,
        hit: &Hit,
        recurse: u8,
    ) -> Colour;

    /// Compute the colour at the ray intersection with the object, taking into consideration the light.
    fn compute_per_light(&self, viewer: &Vector, light_direction: &Vector, hit: &Hit) -> Colour;

    // Diffuse BRDF.
    fn brdf(&self, normal: &Vector, light_dir: &Vector) -> Colour {
        let cos_theta = normal.dot(light_dir).abs();
        (cos_theta / PI) * Colour::new(1.0, 1.0, 1.0, 1.0)
    }

    fn is_specular(&self) -> bool {
        false
    }
}
