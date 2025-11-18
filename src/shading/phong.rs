use crate::{
    primitives::{ray::Ray, Colour, Hit, Vector},
    shading::traits::{Shader, SurfaceProperties, BRDF},
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::{Vector, Vertex};
    use approx::assert_relative_eq;

    #[test]
    fn test_phong_ambient() {
        let ambient = Colour::new(0.2, 0.3, 0.4, 1.0);
        let diffuse = Colour::new(0.5, 0.5, 0.5, 1.0);
        let specular = Colour::new(0.3, 0.3, 0.3, 1.0);
        let material = PhongMaterial::new(ambient, diffuse, specular, 32.0);

        let result = material.calculate_ambient();
        assert_eq!(result.r, 0.2);
        assert_eq!(result.g, 0.3);
        assert_eq!(result.b, 0.4);
    }

    #[test]
    fn test_phong_diffuse() {
        let ambient = Colour::new(0.1, 0.1, 0.1, 1.0);
        let diffuse = Colour::new(1.0, 1.0, 1.0, 1.0);
        let specular = Colour::new(0.3, 0.3, 0.3, 1.0);
        let material = PhongMaterial::new(ambient, diffuse, specular, 32.0);

        let light_direction = Vector::new(0.0, -1.0, 0.0);
        let hit = Hit::new(
            1.0,
            true,
            Vertex::new(0.0, 0.0, 0.0, 1.0),
            Vector::new(0.0, 1.0, 0.0),
        );

        let result = material.calculate_diffuse(&light_direction, &hit);
        // Light directly above, normal pointing up -> max diffuse
        assert_relative_eq!(result.r, 1.0, epsilon = 1e-5);
    }

    #[test]
    fn test_phong_diffuse_angle() {
        let ambient = Colour::new(0.1, 0.1, 0.1, 1.0);
        let diffuse = Colour::new(1.0, 1.0, 1.0, 1.0);
        let specular = Colour::new(0.3, 0.3, 0.3, 1.0);
        let material = PhongMaterial::new(ambient, diffuse, specular, 32.0);

        // Light at 45 degrees
        let light_direction = Vector::new(0.0, -0.707, 0.707).normalise();
        let hit = Hit::new(
            1.0,
            true,
            Vertex::new(0.0, 0.0, 0.0, 1.0),
            Vector::new(0.0, 1.0, 0.0),
        );

        let result = material.calculate_diffuse(&light_direction, &hit);
        assert!(result.r > 0.6 && result.r < 0.8);
    }
}
