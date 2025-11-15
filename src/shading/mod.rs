pub mod ambient_occlusion;
pub mod global;
pub mod phong;
pub mod scene_material;
pub mod traits;

pub use ambient_occlusion::AmbientOcclusionMaterial;
pub use global::GlobalMaterial;
pub use phong::PhongMaterial;
pub use scene_material::{MaterialId, MaterialStorage, SceneMaterial};
pub use traits::{BRDF, Shader, SurfaceProperties};
