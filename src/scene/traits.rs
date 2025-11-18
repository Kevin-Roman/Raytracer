use crate::{config::RaytracerConfig, Light, SceneObject};

pub trait SceneBuilder {
    /// Add an object to the scene
    fn add_object(&mut self, object: SceneObject);

    /// Add a light to the scene
    fn add_light(&mut self, light: Light);

    /// Get a reference to the raytracer configuration
    fn config(&self) -> &RaytracerConfig;
}
