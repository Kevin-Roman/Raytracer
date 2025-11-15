use crate::{
    config::RaytracerConfig,
    primitives::{ray::Ray, Colour},
};

/// RenderContext provides all necessary data for shading calculations.
pub struct RenderContext<'a> {
    pub config: &'a RaytracerConfig,
    pub recurse_depth: u8,
}

impl<'a> RenderContext<'a> {
    pub fn new(config: &'a RaytracerConfig, recurse_depth: u8) -> Self {
        Self {
            config,
            recurse_depth,
        }
    }

    pub fn with_decreased_depth(&self) -> Self {
        Self {
            config: self.config,
            recurse_depth: self.recurse_depth.saturating_sub(1),
        }
    }
}

/// Raytracer trait - focused interface for ray tracing operations.
pub trait Raytracer: Sync {
    /// Trace a ray and return its colour and distance
    fn trace(&self, ray: &Ray, recurse_depth: u8) -> (Colour, f32);

    /// Test if a ray is occluded up to a certain distance
    fn is_occluded(&self, ray: &Ray, max_distance: f32) -> bool;

    /// Get the configuration
    fn config(&self) -> &RaytracerConfig;
}
