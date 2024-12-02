use crate::{
    environments::photon_scene::PhotonMaps,
    primitives::{colour::Colour, ray::Ray},
};

use super::{light::Light, object::Object};

/// Small rounding error used to move shadow ray point along the ray by a small amount
/// in case the shadow position is behind the hit (due to floating point precision).
pub const ROUNDING_ERROR: f32 = 0.001;

/// Environment is the trait for raytracing.
pub trait Environment {
    fn setup(&mut self) {}

    /// Shadowtrace returns whether a ray intersects an object in the environment.
    fn shadowtrace(&self, ray: &Ray, limit: f32) -> bool;

    /// Raytrace returns the colour of a ray in the environment.
    /// Returns the colour of the ray and the distance to the intersection.
    fn raytrace(&self, ray: &Ray, recurse: u8) -> (Colour, f32);

    fn add_object(&mut self, object: Box<dyn Object>);

    fn add_light(&mut self, light: Box<dyn Light>);

    fn get_photon_maps(&self) -> Option<&PhotonMaps> {
        None
    }
}
