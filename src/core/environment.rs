use crate::primitives::{colour::Colour, ray::Ray};

/// Environment is the trait for raytracing.
pub trait Environment {
    /// Shadowtrace returns whether a ray intersects an object in the environment.
    fn shadowtrace(&mut self, ray: &Ray, limit: f32) -> bool;

    /// Raytrace returns the colour of a ray in the environment.
    ///
    fn raytrace(&mut self, ray: &Ray, recurse: u8) -> (Colour, f32);
}
