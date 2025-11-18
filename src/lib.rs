pub mod config;
pub mod geometry;
pub mod primitives;
pub mod rendering;
pub mod sampling;
pub mod scene;
pub mod shading;
pub mod utilities;

pub use geometry::SceneObject;
pub use rendering::{Camera, FrameBuffer, Light, Raytracer};
pub use scene::{PhotonScene, Scene, SceneBuilder};
pub use shading::Material;
