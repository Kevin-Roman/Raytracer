use crate::{
    core::environment::Environment,
    primitives::{colour::Colour, hit::Hit, ray::Ray, vector::Vector},
};

/// Material is the trait for object materials.
pub trait Material: Sync + Send {
    /// Compute the colour at the ray intersection with the object
    fn compute_once(
        &self,
        _environment: &dyn Environment,
        _viewer: &Ray,
        _hit: &Hit,
        _recurse: u8,
    ) -> Colour {
        Colour::default()
    }

    /// Compute the colour at the ray intersection with the object, taking into consideration the light.
    fn compute_per_light(
        &self,
        _environment: &dyn Environment,
        _viewer: &Vector,
        _light_direction: &Vector,
        _hit: &Hit,
        _recurse: u8,
    ) -> Colour {
        Colour::default()
    }

    fn brdf(&self, _viewer: &Vector, _light_direction: &Vector, _hit: &Hit) -> Colour {
        Colour::new(1.0, 1.0, 1.0, 1.0)
    }

    fn is_specular(&self) -> bool {
        false
    }

    fn is_transparent(&self) -> bool {
        false
    }

    fn get_index_of_refraction(&self) -> Option<f32> {
        None
    }
}
