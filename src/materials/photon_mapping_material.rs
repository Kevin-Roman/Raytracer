use std::collections::HashSet;

use crate::{
    core::{environment::Environment, material::Material},
    environments::photon_scene::{PhotonMap, PhotonMaps, PHOTON_SEARCH_RADIUS},
    primitives::{colour::Colour, hit::Hit, photon::PhotonType, ray::Ray},
};

#[derive(Clone, Copy)]
pub struct PhotonMappingMaterial {}

impl PhotonMappingMaterial {
    pub fn new() -> Self {
        Self {}
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

impl Material for PhotonMappingMaterial {
    fn compute_once(
        &self,
        environment: &dyn Environment,
        _viewer: &Ray,
        hit: &Hit,
        _recurse: u8,
    ) -> Colour {
        let mut colour = Colour::default();

        if let Some(photon_maps) = environment.get_photon_maps() {
            colour += self.calculate_soft_indirect_illumination(photon_maps, hit);
            colour += self.calculate_caustics(&photon_maps.caustic, hit);
        }

        colour
    }
}
