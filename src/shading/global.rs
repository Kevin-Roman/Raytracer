use std::f32::consts::PI;

use crate::{
    primitives::{ray::Ray, Colour, Hit, Vector},
    shading::{Shader, SurfaceProperties, BRDF},
    Raytracer,
};

/// GlobalMaterial computes reflection/refraction effects.
#[derive(Clone, Copy, Debug)]
pub struct GlobalMaterial {
    pub reflect_weight: Colour,
    pub refract_weight: Colour,
    pub index_of_refraction: f32,
}

impl GlobalMaterial {
    pub fn new(reflect_weight: Colour, refract_weight: Colour, index_of_refraction: f32) -> Self {
        Self {
            reflect_weight,
            refract_weight,
            index_of_refraction,
        }
    }

    pub fn reflective(reflectivity: f32) -> Self {
        Self {
            reflect_weight: Colour::new(reflectivity, reflectivity, reflectivity, 1.0),
            refract_weight: Colour::new(0.0, 0.0, 0.0, 0.0),
            index_of_refraction: 1.0,
        }
    }

    pub fn transparent(transparency: f32, ior: f32) -> Self {
        Self {
            reflect_weight: Colour::new(0.0, 0.0, 0.0, 0.0),
            refract_weight: Colour::new(transparency, transparency, transparency, 1.0),
            index_of_refraction: ior,
        }
    }

    pub fn get_surface_properties(&self) -> SurfaceProperties {
        let reflectivity = self.reflect_weight.average();
        let transparency = self.refract_weight.average();

        SurfaceProperties::reflective_and_transparent(
            reflectivity,
            transparency,
            self.index_of_refraction,
        )
    }

    /// Calculates the Fresnel reflection and transmission coefficients for the interaction
    /// between an incident vector and a surface normal based on the material's index of refraction.
    ///
    /// Returns (reflection_coefficient, transmission_coefficient)
    fn fresnel_coefficients(&self, incident: Vector, normal: Vector) -> (f32, f32) {
        // Cosine of the angle of incidence.
        let cos_i = normal.dot(incident).abs();

        // Cosine of the angle of transmission.
        let cos_t = (1.0 - (1.0 / self.index_of_refraction.powi(2)) * (1.0 - cos_i.powi(2))).sqrt();

        // Total internal reflection occurs when the term that will be square rooted is a negative number.
        if cos_t.is_nan() {
            return (1.0, 1.0);
        }

        // Reflection parallel coefficient.
        let r_par =
            (self.index_of_refraction * cos_i - cos_t) / (self.index_of_refraction * cos_i + cos_t);
        // Reflection perpendicular coefficient.
        let r_per =
            (cos_i - self.index_of_refraction * cos_t) / (cos_i + self.index_of_refraction * cos_t);

        // Fresnel reflection coefficient.
        let reflection_coefficient: f32 = (r_par.powi(2) + r_per.powi(2)) / 2.0;
        // Fresnel transmission coefficient.
        let transmission_coefficient = 1.0 - reflection_coefficient;

        assert!(
            (0.0..=1.0).contains(&reflection_coefficient),
            "Invalid Fresnel coefficient."
        );

        (reflection_coefficient, transmission_coefficient)
    }
}

impl BRDF for GlobalMaterial {
    fn eval(&self, _viewer: &Vector, _light_direction: &Vector, _hit: &Hit) -> Colour {
        Colour::new(1.0, 1.0, 1.0, 1.0) / PI
    }
}

impl<R: Raytracer> Shader<R> for GlobalMaterial {
    fn shade_ambient(&self, ctx: &R, ray: &Ray, hit: &Hit, recurse_depth: u8) -> Colour {
        let config = ctx.config();

        // Check if we've exceeded maximum recursion depth
        if recurse_depth >= config.camera.raytrace_recurse {
            return Colour::default();
        }

        let mut colour = Colour::default();
        let rounding_error = config.objects.rounding_error;

        // Calculate reflection and refraction rays.
        let mut reflection_ray = Ray::default();
        reflection_ray.direction = ray.direction.reflection(hit.normal).normalise();
        reflection_ray.position = hit.position + rounding_error * reflection_ray.direction;

        let mut refract_ray = Ray::default();
        refract_ray.direction = ray
            .direction
            .refraction(hit.normal, self.index_of_refraction)
            .normalise();
        refract_ray.position = hit.position + rounding_error * refract_ray.direction;

        // Calculate reflection and refraction coefficients.
        let (reflection_coefficient, transmission_coefficient) =
            self.fresnel_coefficients(ray.direction, hit.normal);

        // Recurse on reflection and refraction rays with incremented depth.
        colour += reflection_coefficient
            * self.reflect_weight
            * ctx.trace(&reflection_ray, recurse_depth + 1).0;
        colour += transmission_coefficient
            * self.refract_weight
            * ctx.trace(&refract_ray, recurse_depth + 1).0;

        colour
    }

    fn surface_properties(&self) -> SurfaceProperties {
        SurfaceProperties {
            reflectivity: self.reflect_weight.average(),
            transparency: self.refract_weight.average(),
            index_of_refraction: self.index_of_refraction,
            is_specular: self.reflect_weight.average() > 0.0,
        }
    }
}
