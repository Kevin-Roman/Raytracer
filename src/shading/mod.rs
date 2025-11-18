pub mod ambient_occlusion;
pub mod global;
pub mod material;
pub mod phong;
pub mod traits;

pub use ambient_occlusion::AmbientOcclusionMaterial;
pub use global::GlobalMaterial;
pub use material::Material;
pub use phong::PhongMaterial;
pub use traits::{Shader, SurfaceProperties, BRDF};
