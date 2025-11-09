use std::collections::HashSet;

use crate::{
    config::RaytracerConfig,
    core::{environment::Environment, material::Material},
    environments::photon_scene::PhotonMaps,
    primitives::{colour::Colour, hit::Hit, photon::PhotonType, ray::Ray, vector::Vector},
};

#[derive(Clone, Copy, Default)]
pub struct PhotonMappingMaterial {}

impl PhotonMappingMaterial {
    pub fn new() -> Self {
        Self {}
    }

    fn calculate_soft_indirect_illumination(
        &self,
        viewer: &Vector,
        photon_maps: &PhotonMaps,
        hit: &Hit,
        config: &RaytracerConfig,
    ) -> Colour {
        photon_maps.global_radiance_estimate(
            viewer,
            hit,
            self,
            &HashSet::from([PhotonType::IndirectIllumination]),
            config,
        )
    }

    fn calculate_caustics(
        &self,
        viewer: &Vector,
        photon_maps: &PhotonMaps,
        hit: &Hit,
        config: &RaytracerConfig,
    ) -> Colour {
        photon_maps.caustic_radiance_estimate(
            viewer,
            hit,
            self,
            &HashSet::from([PhotonType::IndirectIllumination]),
            config,
        )
    }
}

impl Material for PhotonMappingMaterial {
    fn compute_once(
        &self,
        environment: &dyn Environment,
        viewer: &Ray,
        hit: &Hit,
        _recurse: u8,
    ) -> Colour {
        let mut colour = Colour::default();
        let config = environment.config();

        if let Some(photon_maps) = environment.get_photon_maps() {
            colour += self.calculate_soft_indirect_illumination(
                &viewer.direction,
                photon_maps,
                hit,
                config,
            );
            colour += self.calculate_caustics(&viewer.direction, photon_maps, hit, config);
        }

        colour
    }
}
