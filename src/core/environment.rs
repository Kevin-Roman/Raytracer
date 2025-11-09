use crate::{
    config::RaytracerConfig,
    core::light::Light,
    environments::photon_scene::PhotonMaps,
    primitives::{colour::Colour, ray::Ray},
};

use super::object::Object;

/// Environment is the trait for raytracing.
pub trait Environment {
    fn setup(&mut self) {}

    /// Get the configuration for this environment
    fn config(&self) -> &RaytracerConfig;

    /// Shadowtrace returns whether a ray intersects an object in the environment.
    fn shadowtrace(&self, ray: &Ray, limit: f32) -> bool;

    /// Raytrace returns the colour of a ray in the environment.
    /// Returns the colour of the ray and the distance to the intersection.
    fn raytrace(&self, ray: &Ray, recurse: u8) -> (Colour, f32);

    fn add_object(&mut self, object: Box<dyn Object>);

    fn add_light(&mut self, light: Light);

    fn get_photon_maps(&self) -> Option<&PhotonMaps> {
        None
    }
}
