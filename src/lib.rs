// New composition-based module structure
pub mod geometry; // Objects and intersection traits
pub mod rendering; // Rendering infrastructure, cameras, and light
pub mod sampling; // Sampling and sampler traits
pub mod scene; // Scene management
pub mod shading; // Materials and shader traits

// Fundamental types and utilities
pub mod config;
pub mod primitives;
pub mod utilities;

// Convenience re-exports for common patterns
pub use geometry::SceneObject;
pub use rendering::{Camera, FrameBuffer, Light, Raytracer};
pub use scene::{PhotonScene, Scene, SceneBuilder};
pub use shading::{MaterialId, SceneMaterial};
