use kd_tree::KdTree;

use crate::{
    core::{
        environment::{Environment, ROUNDING_ERROR},
        light::Light,
        object::{HitPool, Object},
        sampler::Sampler,
    },
    primitives::{
        colour::Colour,
        photon::{Photon, PhotonType},
        ray::Ray,
    },
    samplers::multi_jitter_sampler::MultiJitterSampler,
};

const PHOTON_RECURSE: u8 = 2;
const RUSSION_ROULETTE_CHANCE: f32 = 0.5;

use rand::Rng;

enum PhotonOutcome {
    Reflect,
    Absorb,
}

fn russian_roulette() -> PhotonOutcome {
    let mut rng = rand::thread_rng();
    let chance: f32 = rng.gen();

    if chance < RUSSION_ROULETTE_CHANCE {
        PhotonOutcome::Reflect
    } else {
        PhotonOutcome::Absorb
    }
}

pub struct PhotonScene {
    pub objects: Vec<Box<dyn Object>>,
    pub lights: Vec<Box<dyn Light>>,
    pub global_photon_map: KdTree<Photon>,
    pub caustic_photon_map: KdTree<Photon>,
}

impl PhotonScene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            lights: Vec::new(),
            global_photon_map: KdTree::default(),
            caustic_photon_map: KdTree::default(),
        }
    }

    pub fn photon_trace(
        &self,
        photon_map: &mut Vec<Photon>,
        ray: &Ray,
        photon_type: PhotonType,
        recurse: u8,
    ) {
        let mut surface_hits = HitPool::new();

        // TODO: those that are not specular, for global photon map.
        for object in self.objects.iter() {
            let hitpool = object.generate_hitpool(ray);
            for hit in hitpool.flatten() {
                if hit.distance > 0.0 && hit.entering {
                    surface_hits.insert(*hit);
                }
            }
        }

        for hit in surface_hits.to_vec() {
            photon_map.push(Photon::new(
                hit.position,
                ray.direction,
                Colour::default(),
                photon_type,
            ));

            if recurse == 0 {
                continue;
            }

            match russian_roulette() {
                PhotonOutcome::Reflect => {
                    // TODO: BRDF.
                    let reflection_direction = ray.direction.reflection(&hit.normal);
                    let reflected_ray = Ray::new(
                        hit.position + ROUNDING_ERROR * reflection_direction,
                        reflection_direction,
                    );
                    self.photon_trace(
                        photon_map,
                        &reflected_ray,
                        PhotonType::IndirectIllumination,
                        recurse - 1,
                    );
                }
                PhotonOutcome::Absorb => {
                    let absorbed_ray =
                        Ray::new(hit.position + ROUNDING_ERROR * ray.direction, ray.direction);
                    self.photon_trace(
                        photon_map,
                        &absorbed_ray,
                        PhotonType::ShadowPhoton,
                        recurse - 1,
                    );
                }
            }
        }
    }
}

impl Environment for PhotonScene {
    /// Pass 1: Constructing the Photon Maps.
    fn initialise(&mut self) {
        let sampler = MultiJitterSampler::new(64);

        let mut global_photon_map: Vec<Photon> = Vec::new();
        // TODO: let mut caustic_photon_map: Vec<Photon> = Vec::new();

        for light_index in 0..self.lights.len() {
            let samples = sampler.hemisphere_sampler(1.0);

            if let Some(light_position) = self.lights[light_index].get_position() {
                for sample_direction in &samples {
                    let photon_ray = Ray::new(light_position, *sample_direction);

                    // TODO: photon stores light intensity.

                    self.photon_trace(
                        &mut global_photon_map,
                        &photon_ray,
                        PhotonType::DirectionIllumination,
                        PHOTON_RECURSE,
                    );
                }
            }
        }

        self.global_photon_map = KdTree::par_build_by_ordered_float(global_photon_map);
    }

    fn shadowtrace(&self, _ray: &Ray, _limit: f32) -> bool {
        todo!()
    }

    fn raytrace(&self, _ray: &Ray, _recurse: u8) -> (Colour, f32) {
        todo!()
    }

    fn add_object(&mut self, object: Box<dyn Object>) {
        self.objects.push(object);
    }

    fn add_light(&mut self, light: Box<dyn Light>) {
        self.lights.push(light);
    }
}
