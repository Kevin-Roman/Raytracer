// The global material generates a reflection/refraction layer.

use crate::core::{
    colour::Colour, environment::Environment, hit::Hit, material::Material, ray::Ray,
    scene::SMALL_ROUNDING_ERROR, vector::Vector,
};

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
}

impl Material for GlobalMaterial {
    fn compute_once(
        &self,
        environment: &mut dyn Environment,
        viewer: &Ray,
        hit: &Hit,
        recurse: i32,
    ) -> Colour {
        let mut colour = Colour::default();

        if recurse == 0 {
            return colour;
        }

        let mut reflection_ray = Ray::default();
        reflection_ray.direction = viewer.direction.reflection(&hit.normal).normalise();
        reflection_ray.position = hit.position + SMALL_ROUNDING_ERROR * reflection_ray.direction;

        colour += self.reflect_weight * environment.raytrace(&reflection_ray, recurse - 1).0;

        let mut refract_ray = Ray::default();
        refract_ray.direction = viewer
            .direction
            .refraction(&hit.normal, self.index_of_refraction)
            .normalise();
        refract_ray.position = hit.position + SMALL_ROUNDING_ERROR * refract_ray.direction;

        colour += self.refract_weight * environment.raytrace(&refract_ray, recurse - 1).0;

        colour
    }

    fn compute_per_light(&self, _viewer: &Vector, _light_direction: &Vector, _hit: &Hit) -> Colour {
        Colour::default()
    }
}
