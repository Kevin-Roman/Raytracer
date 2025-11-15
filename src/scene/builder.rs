use crate::{config::RaytracerConfig, Light, MaterialId, SceneMaterial, SceneObject};

pub trait SceneBuilder {
    /// Add an object to the scene
    fn add_object(&mut self, object: SceneObject);

    /// Add a light to the scene
    fn add_light(&mut self, light: Light);

    /// Add a material to the scene and get its ID
    fn add_material(&mut self, material: SceneMaterial) -> MaterialId;

    /// Get a reference to the raytracer configuration
    fn config(&self) -> &RaytracerConfig;
}
