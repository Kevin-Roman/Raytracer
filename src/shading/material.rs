use crate::{
    primitives::{ray::Ray, Colour, Hit, Vector},
    rendering::raytracer::Raytracer,
    shading::traits::{Shader, SurfaceProperties, BRDF},
};

use super::{
    ambient_occlusion::AmbientOcclusionMaterial, global::GlobalMaterial, phong::PhongMaterial,
};

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Phong(PhongMaterial),
    Global(GlobalMaterial),
    AmbientOcclusion(AmbientOcclusionMaterial),
}

impl Material {
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
            Material::Phong(_) => false,
            Material::Global(m) => {
                m.reflect_weight.r > 0.5 || m.reflect_weight.g > 0.5 || m.reflect_weight.b > 0.5
            }
            Material::AmbientOcclusion(_) => false,
        }
    }

    /// Check if this material is transparent (for photon mapping)
    pub fn is_transparent(&self) -> bool {
        match self {
            Material::Phong(_) => false,
            Material::Global(m) => {
                m.refract_weight.r > 0.0 || m.refract_weight.g > 0.0 || m.refract_weight.b > 0.0
            }
            Material::AmbientOcclusion(_) => false,
        }
    }

    /// Get index of refraction (for photon mapping transmission)
    pub fn index_of_refraction(&self) -> Option<f32> {
        match self {
            Material::Phong(_) => None,
            Material::Global(m) => {
                if m.refract_weight.r > 0.0 || m.refract_weight.g > 0.0 || m.refract_weight.b > 0.0
                {
                    Some(m.index_of_refraction)
                } else {
                    None
                }
            }
            Material::AmbientOcclusion(_) => None,
        }
    }

    /// Get BRDF value (for photon mapping radiance estimation)
    pub fn brdf(&self, viewer: &Vector, light_direction: &Vector, hit: &Hit) -> Colour {
        self.eval(viewer, light_direction, hit)
    }
}

impl BRDF for Material {
    fn eval(&self, viewer: &Vector, light_direction: &Vector, hit: &Hit) -> Colour {
        match self {
            Material::Phong(m) => m.eval(viewer, light_direction, hit),
            Material::Global(m) => m.eval(viewer, light_direction, hit),
            Material::AmbientOcclusion(_) => Colour::default(), // AO doesn't use BRDF
        }
    }
}

impl<R: Raytracer> Shader<R> for Material {
    fn shade_ambient(&self, ctx: &R, ray: &Ray, hit: &Hit, recurse_depth: u8) -> Colour {
        match self {
            Material::Phong(m) => m.shade_ambient(ctx, ray, hit, recurse_depth),
            Material::Global(m) => m.shade_ambient(ctx, ray, hit, recurse_depth),
            Material::AmbientOcclusion(m) => m.shade_ambient(ctx, ray, hit, recurse_depth),
        }
    }

    fn shade_light(&self, ctx: &R, viewer: &Vector, light_direction: &Vector, hit: &Hit) -> Colour {
        match self {
            Material::Phong(m) => m.shade_light(ctx, viewer, light_direction, hit),
            Material::Global(m) => m.shade_light(ctx, viewer, light_direction, hit),
            Material::AmbientOcclusion(_) => Colour::default(), // AO doesn't use direct lighting
        }
    }

    fn surface_properties(&self) -> SurfaceProperties {
        match self {
            Material::Phong(m) => m.get_surface_properties(),
            Material::Global(m) => m.get_surface_properties(),
            Material::AmbientOcclusion(m) => m.get_surface_properties(),
        }
    }
}
