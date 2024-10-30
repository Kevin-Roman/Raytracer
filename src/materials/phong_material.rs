// Phong is Material and implements the simple Phong surface illumination model.

use crate::core::{
    colour::Colour, environment::Environment, hit::Hit, material::Material, ray::Ray,
    vector::Vector,
};

pub struct PhongMaterial {
    ambient: Colour,
    diffuse: Colour,
    specular: Colour,
    /// For sharpness of highlights.
    control_factor: f32,
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

    fn calculate_ambient(&self) -> Colour {
        self.ambient
    }

    fn calculate_diffuse(&self, light_direction: &Vector, hit: &Hit) -> Colour {
        let cosine_angle_of_incidence: f32 = light_direction.negate().dot(&hit.normal);

        cosine_angle_of_incidence * self.diffuse
    }

    fn calculate_specular(&self, viewer: &Vector, light_direction: &Vector, hit: &Hit) -> Colour {
        let reflection = light_direction.negate().reflection(&hit.normal);

        reflection.dot(&viewer).powf(self.control_factor) * self.specular
    }
}

impl Material for PhongMaterial {
    fn compute_once(
        &self,
        _environment: &mut dyn Environment,
        _viewer: &Ray,
        _hit: &Hit,
        _recurse: i32,
    ) -> Colour {
        self.calculate_ambient()
    }

    fn compute_per_light(&self, viewer: &Vector, light_direction: &Vector, hit: &Hit) -> Colour {
        self.calculate_diffuse(light_direction, hit)
            + self.calculate_specular(viewer, light_direction, hit)
    }
}
