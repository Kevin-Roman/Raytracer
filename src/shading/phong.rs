use crate::{
    primitives::{ray::Ray, Colour, Hit, Vector},
    shading::traits::{BRDF, Shader, SurfaceProperties},
    Raytracer,
};

/// PhongMaterial implements the Phong surface illumination model.
#[derive(Clone, Copy, Debug)]
pub struct PhongMaterial {
    pub ambient: Colour,
    pub diffuse: Colour,
    pub specular: Colour,
    /// For sharpness of highlights.
    pub control_factor: f32,
}

impl PhongMaterial {
    pub fn new(ambient: Colour, diffuse: Colour, specular: Colour, control_factor: f32) -> Self {
        Self {
            ambient,
            diffuse,
            specular,
            control_factor,
        }
    }

    /// Get surface properties for this material (opaque with no special effects)
    pub fn get_surface_properties(&self) -> SurfaceProperties {
        SurfaceProperties::default()
    }

    fn calculate_ambient(&self) -> Colour {
        self.ambient
    }

    fn calculate_diffuse(&self, light_direction: &Vector, hit: &Hit) -> Colour {
        let cosine_angle_of_incidence: f32 = light_direction.negate().dot(hit.normal);
        cosine_angle_of_incidence * self.diffuse
    }

    fn calculate_specular(&self, viewer: &Vector, light_direction: &Vector, hit: &Hit) -> Colour {
        let reflection = light_direction.negate().reflection(hit.normal);
        reflection.dot(*viewer).powf(self.control_factor) * self.specular
    }
}

impl BRDF for PhongMaterial {
    fn eval(&self, viewer: &Vector, light_direction: &Vector, hit: &Hit) -> Colour {
        self.calculate_diffuse(light_direction, hit)
            + self.calculate_specular(viewer, light_direction, hit)
    }
}

impl<R: Raytracer> Shader<R> for PhongMaterial {
    fn shade_ambient(&self, _ctx: &R, _ray: &Ray, _hit: &Hit, _recurse_depth: u8) -> Colour {
        self.calculate_ambient()
    }

    fn shade_light(
        &self,
        _ctx: &R,
        viewer: &Vector,
        light_direction: &Vector,
        hit: &Hit,
    ) -> Colour {
        self.calculate_diffuse(light_direction, hit)
            + self.calculate_specular(viewer, light_direction, hit)
    }
}
