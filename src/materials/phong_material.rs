use crate::{
    core::{environment::Environment, material::Material},
    primitives::{colour::Colour, hit::Hit, ray::Ray, vector::Vector},
};

/// PhongMaterial is a Material that implements the Phong surface illumination model.
#[derive(Clone, Copy)]
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

        reflection.dot(viewer).powf(self.control_factor) * self.specular
    }
}

impl Material for PhongMaterial {
    fn compute_once(
        &self,
        _environment: &dyn Environment,
        _viewer: &Ray,
        _hit: &Hit,
        _recurse: u8,
    ) -> Colour {
        self.calculate_ambient()
    }

    fn compute_per_light(
        &self,
        _environment: &dyn Environment,
        viewer: &Vector,
        light_direction: &Vector,
        hit: &Hit,
        _recurse: u8,
    ) -> Colour {
        self.calculate_diffuse(light_direction, hit)
            + self.calculate_specular(viewer, light_direction, hit)
    }

    fn brdf(&self, viewer: &Vector, light_direction: &Vector, hit: &Hit) -> Colour {
        self.calculate_diffuse(light_direction, hit)
            + self.calculate_specular(viewer, light_direction, hit)
    }
}
