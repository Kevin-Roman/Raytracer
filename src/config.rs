use serde::{Deserialize, Serialize};

/// Core raytracing configuration
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct RaytracerConfig {
    pub camera: CameraConfig,
    pub photon_mapping: PhotonMappingConfig,
    pub materials: MaterialConfig,
    pub objects: ObjectConfig,
    pub sampler: SamplerConfig,
    pub framebuffer: FramebufferConfig,
    pub cornell_box: CornellBoxConfig,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CameraConfig {
    /// Maximum recursion depth for ray tracing
    pub raytrace_recurse: u8,

    /// Number of camera ray samples for anti-aliasing
    pub num_camera_ray_samples: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PhotonMappingConfig {
    /// Recursion threshold for approximate radiance estimation
    pub recurse_approximate_threshold: u8,

    /// Maximum recursion depth for photon tracing
    pub photon_recurse: u8,

    /// Total number of photons to emit
    pub num_photons: u32,

    /// Search radius for photon lookups
    pub photon_search_radius: f32,

    /// Number of photons to consider in radiance estimate
    pub photon_search_count: u32,

    pub use_shadow_estimation: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MaterialConfig {
    /// Maximum distance for ambient occlusion shadow rays
    pub shadow_distance_limit: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ObjectConfig {
    /// Small rounding error for shadow ray adjustment
    pub rounding_error: f32,

    /// Epsilon for polymesh intersection calculations
    pub polymesh_epsilon: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SamplerConfig {
    /// Number of sample sets to generate
    pub num_sets: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FramebufferConfig {
    pub width: u16,
    pub height: u16,

    pub max_width: u16,
    pub max_height: u16,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CornellBoxConfig {
    pub width: f32,
    pub length: f32,
    pub height: f32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            raytrace_recurse: 5,
            num_camera_ray_samples: 16,
        }
    }
}

impl Default for PhotonMappingConfig {
    fn default() -> Self {
        Self {
            recurse_approximate_threshold: 2,
            photon_recurse: 3,
            num_photons: 202_500,
            photon_search_radius: 5.0,
            photon_search_count: 100,
            use_shadow_estimation: false,
        }
    }
}

impl Default for MaterialConfig {
    fn default() -> Self {
        Self {
            shadow_distance_limit: 50.0,
        }
    }
}

impl Default for ObjectConfig {
    fn default() -> Self {
        Self {
            rounding_error: 0.001,
            polymesh_epsilon: 0.000001,
        }
    }
}

impl Default for SamplerConfig {
    fn default() -> Self {
        Self { num_sets: 4 }
    }
}

impl Default for FramebufferConfig {
    fn default() -> Self {
        Self {
            width: 512,
            height: 512,
            max_width: 2048,
            max_height: 2048,
        }
    }
}

impl Default for CornellBoxConfig {
    fn default() -> Self {
        Self {
            width: 100.0,
            length: 150.0,
            height: 90.0,
        }
    }
}

impl RaytracerConfig {
    pub fn new() -> Self {
        Self::from_toml_file("Config.toml").unwrap_or_else(|_| Self::default())
    }

    pub fn from_toml_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(path)?;
        let config: RaytracerConfig = toml::from_str(&contents)?;
        Ok(config)
    }

    pub fn to_toml_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let toml_string = toml::to_string_pretty(self)?;
        std::fs::write(path, toml_string)?;
        Ok(())
    }
}
