use crate::{
    primitives::{ray::Ray, Colour, Hit, Vector},
    rendering::raytracer::Raytracer,
    shading::traits::{BRDF, Shader, SurfaceProperties},
};

use super::{
    ambient_occlusion::AmbientOcclusionMaterial, global::GlobalMaterial, phong::PhongMaterial,
};

/// Material ID - index into materials vec.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct MaterialId(pub usize);

#[derive(Clone, Copy, Debug)]
pub enum SceneMaterial {
    Phong(PhongMaterial),
    Global(GlobalMaterial),
    AmbientOcclusion(AmbientOcclusionMaterial),
}

impl SceneMaterial {
    pub fn phong(ambient: Colour, diffuse: Colour, specular: Colour, power: f32) -> Self {
        Self::Phong(PhongMaterial::new(ambient, diffuse, specular, power))
    }

    pub fn reflective(reflectivity: f32) -> Self {
        Self::Global(GlobalMaterial::reflective(reflectivity))
    }

    pub fn transparent(transparency: f32, ior: f32) -> Self {
        Self::Global(GlobalMaterial::transparent(transparency, ior))
    }

    pub fn global(reflect: Colour, refract: Colour, ior: f32) -> Self {
        Self::Global(GlobalMaterial::new(reflect, refract, ior))
    }

    pub fn ambient_occlusion(ambient: Colour, num_samples: u32, min_ambient: f32) -> Self {
        Self::AmbientOcclusion(AmbientOcclusionMaterial::new(
            ambient,
            num_samples,
            min_ambient,
        ))
    }

    /// Check if this material is specular (for photon mapping caustics)
    pub fn is_specular(&self) -> bool {
        match self {
            SceneMaterial::Phong(_) => false,
            SceneMaterial::Global(m) => {
                m.reflect_weight.r > 0.5 || m.reflect_weight.g > 0.5 || m.reflect_weight.b > 0.5
            }
            SceneMaterial::AmbientOcclusion(_) => false,
        }
    }

    /// Check if this material is transparent (for photon mapping)
    pub fn is_transparent(&self) -> bool {
        match self {
            SceneMaterial::Phong(_) => false,
            SceneMaterial::Global(m) => {
                m.refract_weight.r > 0.0 || m.refract_weight.g > 0.0 || m.refract_weight.b > 0.0
            }
            SceneMaterial::AmbientOcclusion(_) => false,
        }
    }

    /// Get index of refraction (for photon mapping transmission)
    pub fn index_of_refraction(&self) -> Option<f32> {
        match self {
            SceneMaterial::Phong(_) => None,
            SceneMaterial::Global(m) => {
                if m.refract_weight.r > 0.0 || m.refract_weight.g > 0.0 || m.refract_weight.b > 0.0
                {
                    Some(m.index_of_refraction)
                } else {
                    None
                }
            }
            SceneMaterial::AmbientOcclusion(_) => None,
        }
    }

    /// Get BRDF value (for photon mapping radiance estimation)
    pub fn brdf(&self, viewer: &Vector, light_direction: &Vector, hit: &Hit) -> Colour {
        self.eval(viewer, light_direction, hit)
    }
}

impl BRDF for SceneMaterial {
    fn eval(&self, viewer: &Vector, light_direction: &Vector, hit: &Hit) -> Colour {
        match self {
            SceneMaterial::Phong(m) => m.eval(viewer, light_direction, hit),
            SceneMaterial::Global(m) => m.eval(viewer, light_direction, hit),
            SceneMaterial::AmbientOcclusion(_) => Colour::default(), // AO doesn't use BRDF
        }
    }
}

impl<R: Raytracer> Shader<R> for SceneMaterial {
    fn shade_ambient(&self, ctx: &R, ray: &Ray, hit: &Hit, recurse_depth: u8) -> Colour {
        match self {
            SceneMaterial::Phong(m) => m.shade_ambient(ctx, ray, hit, recurse_depth),
            SceneMaterial::Global(m) => m.shade_ambient(ctx, ray, hit, recurse_depth),
            SceneMaterial::AmbientOcclusion(m) => m.shade_ambient(ctx, ray, hit, recurse_depth),
        }
    }

    fn shade_light(&self, ctx: &R, viewer: &Vector, light_direction: &Vector, hit: &Hit) -> Colour {
        match self {
            SceneMaterial::Phong(m) => m.shade_light(ctx, viewer, light_direction, hit),
            SceneMaterial::Global(m) => m.shade_light(ctx, viewer, light_direction, hit),
            SceneMaterial::AmbientOcclusion(_) => Colour::default(), // AO doesn't use direct lighting
        }
    }

    fn surface_properties(&self) -> SurfaceProperties {
        match self {
            SceneMaterial::Phong(m) => m.get_surface_properties(),
            SceneMaterial::Global(m) => m.get_surface_properties(),
            SceneMaterial::AmbientOcclusion(m) => m.get_surface_properties(),
        }
    }
}

/// Material storage - a Vec that allows material reuse.
#[derive(Default)]
pub struct MaterialStorage {
    materials: Vec<SceneMaterial>,
}

impl MaterialStorage {
    pub fn new() -> Self {
        Self {
            materials: Vec::new(),
        }
    }

    /// Add a material and return its ID
    pub fn add(&mut self, material: SceneMaterial) -> MaterialId {
        let id = self.materials.len();
        self.materials.push(material);
        MaterialId(id)
    }

    /// Get a material by ID
    pub fn get(&self, id: usize) -> Option<&SceneMaterial> {
        self.materials.get(id)
    }

    /// Get all materials
    pub fn materials(&self) -> &[SceneMaterial] {
        &self.materials
    }
}
