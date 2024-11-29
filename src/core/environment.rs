use crate::primitives::{colour::Colour, ray::Ray};

/// Environment is the trait for raytracing.
pub trait Environment {
    fn initialise(&mut self) {}

    /// Shadowtrace returns whether a ray intersects an object in the environment.
    fn shadowtrace(&self, ray: &Ray, limit: f32) -> bool;

    /// Raytrace returns the colour of a ray in the environment.
    /// Returns the colour of the ray and the distance to the intersection.
    fn raytrace(&self, ray: &Ray, recurse: u8) -> (Colour, f32);
}
