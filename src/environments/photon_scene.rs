use kd_tree::KdTree;
use rand::Rng;
use std::sync::Arc;

use crate::{
    core::{
        environment::{Environment, ROUNDING_ERROR},
        light::Light,
        material::Material,
        object::Object,
        sampler::Sampler,
    },
    materials::phong_material::PhongMaterial,
    objects::sphere_object::Sphere,
    primitives::{
        colour::Colour,
        hit::Hit,
        photon::{Photon, PhotonType},
        ray::Ray,
        vector::Vector,
        vertex::Vertex,
    },
    samplers::multi_jitter_sampler::MultiJitterSampler,
};

const RECURSE_APPROXIMATE_DEPTH: u8 = 2;
const PHOTON_RECURSE: u8 = 1;
// const NUM_PHOTONS: u32 = 10_000;
const NUM_PHOTONS: u32 = 2500;
const PHOTON_SEARCH_RADIUS: f32 = 7.5;
const RUSSION_ROULETTE_CHANCE: f32 = 0.5;

type PhotonMap = KdTree<Photon>;

pub struct PhotonMaps {
    global: PhotonMap,
    caustic: PhotonMap,
}

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
    pub photon_maps: PhotonMaps,
}

impl PhotonScene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            lights: Vec::new(),
            photon_maps: PhotonMaps {
                global: KdTree::default(),
                caustic: KdTree::default(),
            },
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
                let reflection_direction = ray.direction.reflection(&hit.normal).normalise();
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

    // fn compute_lighting_with_photon_map(&self, hit: &Hit, material: &Arc<dyn Material>) -> Colour {
    //     let mut colour = Colour::default();

    //     let photons = self.photon_maps.global.within_radius(
    //         &[
    //             hit.position.vector.x,
    //             hit.position.vector.y,
    //             hit.position.vector.z,
    //         ],
    //         PHOTON_SEARCH_RADIUS,
    //     );
    //     for photon in photons {
    //         colour += photon.intensity * material.brdf(&hit.normal, &photon.direction);
    //     }

    //     colour
    // }

    // fn compute_lighting(&self, hit: &Hit, material: &Arc<dyn Material>) -> Colour {
    //     let mut colour = Colour::new(0.0, 0.0, 0.0, 0.0);

    //     for light in &self.lights {
    //         let viewer_direction = (-hit.position.vector).normalise();
    //         let (light_position, light_direction, is_lit) = light.get_direction(hit.position);

    //         // Skip lights that are facing the wrong direction.
    //         if light_direction.dot(&hit.normal) > 0.0 {
    //             continue;
    //         }

    //         let photons = self.global_photon_map.within_radius(
    //             &[
    //                 hit.position.vector.x,
    //                 hit.position.vector.y,
    //                 hit.position.vector.z,
    //             ],
    //             PHOTON_SEARCH_RADIUS,
    //         );

    //         let mut visible_photons = photons.len();
    //         let photons_len = photons.len();
    //         for photon in photons.into_iter() {
    //             if let PhotonType::ShadowPhoton = photon.photon_type {
    //                 visible_photons -= 1;
    //             }
    //         }

    //         let percent = visible_photons as f32 / photons_len as f32;

    //         if is_lit && percent > 0.7 {
    //             let intensity = light.get_intensity();
    //             colour += intensity
    //                 * material.compute_per_light(&viewer_direction, &light_direction, &hit);
    //         }
    //     }

    //     colour
    // }

    /// Determine if a hit point is in shadow.
    fn is_point_in_shadow(
        &self,
        hit_position: Vertex,
        light_position: Option<Vertex>,
        light_direction: Vector,
    ) -> bool {
        let to_light_direction = light_direction.negate();
        // Move the shadow ray point slightly along the ray (towards the light) to avoid self-shadowing.
        let shadow_ray = Ray::new(
            hit_position + ROUNDING_ERROR * to_light_direction,
            to_light_direction,
        );

        let shadow_limit = light_position
            .map(|light_position| (light_position.vector - shadow_ray.position.vector).length())
            .unwrap_or(f32::INFINITY);

        self.shadowtrace(&shadow_ray, shadow_limit)
    }

    /// Compute contribution of all lights to the hit point.
    fn compute_lighting(&self, hit: &Hit, material: &Arc<dyn Material>) -> Colour {
        let mut colour = Colour::new(0.0, 0.0, 0.0, 0.0);

        for light in &self.lights {
            let viewer_direction = (-hit.position.vector).normalise();
            let (light_position, light_direction, is_lit) = light.get_direction(hit.position);

            // Skip lights that are facing the wrong direction.
            if light_direction.dot(&hit.normal) > 0.0 {
                continue;
            }

            if is_lit && !self.is_point_in_shadow(hit.position, light_position, light_direction) {
                let intensity = light.get_intensity();
                colour += intensity
                    * material.compute_per_light(&viewer_direction, &light_direction, &hit);
            }
        }

        colour
    }

    #[cfg(feature = "debugging")]
    fn display_photons(&mut self, photon_map: &Vec<Photon>) {
        // Testing purposes
        for photon in photon_map {
            let mut sphere = Sphere::new(photon.position, 0.1);
            match photon.photon_type {
                PhotonType::DirectionIllumination => {
                    sphere.set_material(Arc::new(PhongMaterial::new(
                        Colour::new(0.0, 1.0, 0.0, 1.0),
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
                        Colour::new(1.0, 0.0, 0.0, 1.0),
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
        let sampler = MultiJitterSampler::new(NUM_PHOTONS);
        let samples = sampler.hemisphere_sampler(1.0);

        let mut global_photon_map: Vec<Photon> = Vec::new();
        let mut caustic_photon_map: Vec<Photon> = Vec::new();

        for light in &self.lights {
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

                for object in &self.objects {
                    if let Some(material) = object.get_material().cloned() {
                        if !material.is_specular() {
                            continue;
                        }
                    } else {
                        continue;
                    }

                    if let Some(bounding_sphere) = object.bounding_sphere() {
                        for sample_direction in &samples {
                            let target_point =
                                bounding_sphere.0 + (bounding_sphere.1 * *sample_direction);
                            let photon_direction =
                                (target_point.vector - light_position.vector).normalise();
                            let photon_ray = Ray::new(light_position, photon_direction);

                            let photon_power = 1.0 / samples.len() as f32;

                            self.photon_trace(
                                &mut caustic_photon_map,
                                &photon_ray,
                                PhotonType::DirectionIllumination,
                                photon_power * light.get_intensity(),
                                PHOTON_RECURSE,
                            );
                        }
                    }
                }
            }
        }

        #[cfg(feature = "debugging")]
        self.display_photons(&global_photon_map);
        #[cfg(feature = "debugging")]
        self.display_photons(&caustic_photon_map);

        self.photon_maps.global = KdTree::par_build_by_ordered_float(global_photon_map);
        self.photon_maps.caustic = KdTree::par_build_by_ordered_float(caustic_photon_map);
    }

    fn shadowtrace(&self, ray: &Ray, limit: f32) -> bool {
        for object in &self.objects {
            if let Some(hit) = object.select_first_hit(ray) {
                if 0.0 < hit.distance && hit.distance < limit {
                    return true;
                }
            }
        }

        false
    }

    // Pass 2: Rendering the scene.
    fn raytrace(&self, ray: &Ray, recurse: u8) -> (Colour, f32) {
        let mut colour = Colour::new(0.0, 0.0, 0.0, 1.0);
        let mut depth = 0.0;

        if let Some((hit, object_index)) = self.ray_trace(ray) {
            depth = hit.distance;

            if let Some(material) = self.objects[object_index].get_material().cloned() {
                // Compute direct material contribution.
                colour += material.compute_once(self, ray, &hit, recurse);

                // Calculate contributions from lights.
                colour += self.compute_lighting(&hit, &material);
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

    fn get_photon_maps(&self) -> Option<&PhotonMaps> {
        Some(&self.photon_maps)
    }
}
