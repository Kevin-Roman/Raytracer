use crate::{
    core::{environment::Environment, material::Material},
    environments::scene::ROUNDING_ERROR,
    primitives::{colour::Colour, hit::Hit, ray::Ray, vector::Vector},
};

/// GlobalMaterial is a Material that computes a reflection/refraction layer.
#[derive(Clone, Copy, Debug)]
pub struct GlobalMaterial {
    reflect_weight: Colour,
    refract_weight: Colour,
    index_of_refraction: f32,
}

impl GlobalMaterial {
    pub fn new(reflect_weight: Colour, refract_weight: Colour, index_of_refraction: f32) -> Self {
        Self {
            reflect_weight,
            refract_weight,
            index_of_refraction,
        }
    }

    /// Calculates the Fresnel reflection and transmission coefficients for the interaction
    /// between an incident vector and a surface normal based on the material's index of refraction.
    ///
    /// Returns (reflection_coefficient, transmission_coefficient)
    fn fresnel_coefficients(self, incident: &Vector, normal: &Vector) -> (f32, f32) {
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
            0.0 <= reflection_coefficient && reflection_coefficient <= 1.0,
            "Invalid Fresnel coefficient."
        );

        (reflection_coefficient, transmission_coefficient)
    }
}

impl Material for GlobalMaterial {
    fn compute_once(
        &self,
        environment: &dyn Environment,
        viewer: &Ray,
        hit: &Hit,
        recurse: u8,
    ) -> Colour {
        let mut colour = Colour::default();

        if recurse == 0 {
            return colour;
        }

        // Calculate reflection and refraction rays.
        let mut reflection_ray = Ray::default();
        reflection_ray.direction = viewer.direction.reflection(&hit.normal).normalise();
        reflection_ray.position = hit.position + ROUNDING_ERROR * reflection_ray.direction;

        let mut refract_ray = Ray::default();
        refract_ray.direction = viewer
            .direction
            .refraction(&hit.normal, self.index_of_refraction)
            .normalise();
        refract_ray.position = hit.position + ROUNDING_ERROR * refract_ray.direction;

        // Calculate reflection and refraction coefficients.
        let (reflection_coefficient, transmission_coefficient) =
            self.fresnel_coefficients(&viewer.direction, &hit.normal);

        // Recurse on reflection and refraction rays.
        colour += reflection_coefficient
            * self.reflect_weight
            * environment.raytrace(&reflection_ray, recurse - 1).0;
        colour += transmission_coefficient
            * self.refract_weight
            * environment.raytrace(&refract_ray, recurse - 1).0;

        colour
    }

    fn compute_per_light(&self, _viewer: &Vector, _light_direction: &Vector, _hit: &Hit) -> Colour {
        Colour::default()
    }
}
