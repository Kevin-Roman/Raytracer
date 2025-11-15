use crate::primitives::{ray::Ray, Colour, Hit, Vector};

/// Surface properties for rendering calculations.
#[derive(Debug, Clone, Copy)]
pub struct SurfaceProperties {
    pub reflectivity: f32,
    pub transparency: f32,
    pub index_of_refraction: f32,
    pub is_specular: bool,
}

impl Default for SurfaceProperties {
    fn default() -> Self {
        Self {
            reflectivity: 0.0,
            transparency: 0.0,
            index_of_refraction: 1.0,
            is_specular: false,
        }
    }
}

impl SurfaceProperties {
    pub fn reflective(reflectivity: f32) -> Self {
        Self {
            reflectivity,
            is_specular: true,
            ..Default::default()
        }
    }

    pub fn transparent(transparency: f32, ior: f32) -> Self {
        Self {
            transparency,
            index_of_refraction: ior,
            ..Default::default()
        }
    }

    pub fn reflective_and_transparent(reflectivity: f32, transparency: f32, ior: f32) -> Self {
        Self {
            reflectivity,
            transparency,
            index_of_refraction: ior,
            is_specular: true,
        }
    }
}

/// BRDF (Bidirectional Reflectance Distribution Function) computation.
pub trait BRDF: Sync + Send {
    fn eval(&self, viewer: &Vector, light_direction: &Vector, hit: &Hit) -> Colour;
}

/// Shader computes colour contributions.
pub trait Shader<Ctx> {
    /// Compute ambient/emission contribution (independent of lights)
    fn shade_ambient(&self, ctx: &Ctx, ray: &Ray, hit: &Hit, recurse_depth: u8) -> Colour {
        let _ = (ctx, ray, hit, recurse_depth);
        Colour::default()
    }

    /// Compute direct lighting contribution from a specific light
    fn shade_light(
        &self,
        _ctx: &Ctx,
        _viewer: &Vector,
        _light_direction: &Vector,
        _hit: &Hit,
    ) -> Colour {
        Colour::default()
    }

    /// Get surface properties for reflection/refraction calculations
    fn surface_properties(&self) -> SurfaceProperties {
        SurfaceProperties::default()
    }
}
