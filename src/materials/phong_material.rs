use std::collections::HashSet;

use crate::{
    core::{environment::Environment, material::Material},
    environments::photon_scene::{PhotonMap, PhotonMaps, PHOTON_SEARCH_RADIUS},
    primitives::{colour::Colour, hit::Hit, photon::PhotonType, ray::Ray, vector::Vector},
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

        reflection.dot(&viewer).powf(self.control_factor) * self.specular
    }

    fn calculate_soft_indirect_illumination(&self, photon_maps: &PhotonMaps, hit: &Hit) -> Colour {
        photon_maps.radiance_estimate(
            hit,
            self,
            &HashSet::from([PhotonType::IndirectIllumination]),
        )
    }

    fn calculate_caustics(&self, photon_map: &PhotonMap, hit: &Hit) -> Colour {
        let nearby_photons = photon_map.within_radius(
            &[
                hit.position.vector.x,
                hit.position.vector.y,
                hit.position.vector.z,
            ],
            PHOTON_SEARCH_RADIUS,
        );

        let mut colour = Colour::default();
        for photon in nearby_photons {
            colour += hit.normal.dot(&photon.direction).abs() * photon.intensity;
        }

        colour
    }
}

impl Material for PhongMaterial {
    fn compute_once(
        &self,
        environment: &dyn Environment,
        _viewer: &Ray,
        hit: &Hit,
        _recurse: u8,
    ) -> Colour {
        let mut colour = Colour::default();

        colour += self.calculate_ambient();

        if let Some(photon_maps) = environment.get_photon_maps() {
            colour += self.calculate_soft_indirect_illumination(photon_maps, hit);
            colour += self.calculate_caustics(&photon_maps.caustic, hit);
        }

        colour
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
}
