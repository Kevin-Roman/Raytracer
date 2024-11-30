use std::sync::Arc;

use kd_tree::KdTree;

use crate::{
    core::{
        environment::{Environment, ROUNDING_ERROR},
        light::Light,
        object::Object,
        sampler::Sampler,
    },
    materials::{falsecolour_material::FalseColourMaterial, phong_material::PhongMaterial},
    objects::sphere_object::Sphere,
    primitives::{
        colour::Colour,
        hit::Hit,
        photon::{Photon, PhotonType},
        ray::Ray,
    },
    samplers::multi_jitter_sampler::MultiJitterSampler,
};

const PHOTON_RECURSE: u8 = 1;
const RUSSION_ROULETTE_CHANCE: f32 = 0.5;

use rand::Rng;

enum PhotonOutcome {
    Reflect,
    Absorb,
    Transmit,
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

    fn ray_trace(&self, ray: &Ray) -> Option<(Hit, usize)> {
        let mut nearest_hit: Option<(Hit, usize)> = None;

        for (i, object) in self.objects.iter().enumerate() {
            if let Some(hit) = object.select_first_hit(ray) {
                if nearest_hit.is_none() || hit.distance < nearest_hit.unwrap().0.distance {
                    nearest_hit = Some((hit, i));
                }
            }
        }

        nearest_hit
    }

    fn photon_trace(
        &self,
        photon_map: &mut Vec<Photon>,
        ray: &Ray,
        photon_type: PhotonType,
        photon_intensity: Colour,
        recurse: u8,
    ) {
        let mut nearest_hit: Option<(Hit, usize)> = None;

        // TODO: those that are not specular, for global photon map.
        for (i, object) in self.objects.iter().enumerate() {
            // TODO: handle transmision and it's need for hit.entering = false (maybe).
            if let Some(hit) = object.select_first_hit(ray) {
                if 0.0 < hit.distance
                    && hit.entering
                    && (nearest_hit.is_none() || hit.distance < nearest_hit.unwrap().0.distance)
                {
                    nearest_hit = Some((hit, i));
                }
            }
        }

        if nearest_hit.is_none() {
            return;
        }

        let hit = nearest_hit.unwrap().0;

        photon_map.push(Photon::new(
            hit.position,
            ray.direction,
            photon_intensity,
            photon_type,
        ));

        if recurse == 0 {
            return;
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
                    photon_intensity,
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
                    photon_intensity,
                    recurse - 1,
                );
            }
            PhotonOutcome::Transmit => {}
        }
    }

    #[cfg(feature = "debugging")]
    fn display_photons(&mut self, global_photon_map: &Vec<Photon>) {
        // Testing purposes
        for photon in global_photon_map {
            let mut sphere = Sphere::new(photon.position, 0.1);
            match photon.photon_type {
                PhotonType::DirectionIllumination => {
                    sphere.set_material(Arc::new(PhongMaterial::new(
                        Colour::new(1.0, 0.0, 0.0, 1.0),
                        Colour::default(),
                        Colour::default(),
                        1.0,
                    )))
                }
                PhotonType::ShadowPhoton => sphere.set_material(Arc::new(PhongMaterial::new(
                    Colour::new(0.0, 0.0, 1.0, 1.0),
                    Colour::default(),
                    Colour::default(),
                    1.0,
                ))),
                PhotonType::IndirectIllumination => {
                    sphere.set_material(Arc::new(PhongMaterial::new(
                        Colour::new(0.0, 1.0, 0.0, 1.0),
                        Colour::default(),
                        Colour::default(),
                        1.0,
                    )))
                }
            }

            self.add_object(Box::new(sphere));
        }
    }
}

impl Environment for PhotonScene {
    /// Pass 1: Constructing the Photon Maps.
    fn initialise(&mut self) {
        let sampler = MultiJitterSampler::new(10000);

        let mut global_photon_map: Vec<Photon> = Vec::new();
        // TODO: let mut caustic_photon_map: Vec<Photon> = Vec::new();

        for light in &self.lights {
            let samples = sampler.hemisphere_sampler(1.0);

            if let Some(light_position) = light.get_position() {
                for sample_direction in &samples {
                    let photon_ray = Ray::new(light_position, *sample_direction);
                    let photon_power = 1.0 / samples.len() as f32;

                    // TODO: photon stores light intensity.

                    self.photon_trace(
                        &mut global_photon_map,
                        &photon_ray,
                        PhotonType::DirectionIllumination,
                        photon_power * light.get_intensity(),
                        PHOTON_RECURSE,
                    );
                }
            }
        }

        #[cfg(feature = "debugging")]
        self.display_photons(&global_photon_map);

        self.global_photon_map = KdTree::par_build_by_ordered_float(global_photon_map);
    }

    fn shadowtrace(&self, _ray: &Ray, _limit: f32) -> bool {
        false
    }

    fn raytrace(&self, ray: &Ray, recurse: u8) -> (Colour, f32) {
        let mut colour = Colour::new(0.0, 0.0, 0.0, 0.0);
        let mut depth = 0.0;

        if let Some((hit, object_index)) = self.ray_trace(ray) {
            depth = hit.distance;

            if let Some(material) = self.objects[object_index].get_material().cloned() {
                // Compute direct material contribution.
                colour += material.compute_once(self, ray, &hit, recurse);
            }

            let nearest_photon = self
                .global_photon_map
                .nearest(&[
                    hit.position.vector.x,
                    hit.position.vector.y,
                    hit.position.vector.z,
                ])
                .unwrap()
                .item;

            if let PhotonType::ShadowPhoton = nearest_photon.photon_type {
                colour = Colour::new(1.0, 1.0, 1.0, 1.0);
            }
        }

        (colour, depth)
    }

    fn add_object(&mut self, object: Box<dyn Object>) {
        self.objects.push(object);
    }

    fn add_light(&mut self, light: Box<dyn Light>) {
        self.lights.push(light);
    }
}
