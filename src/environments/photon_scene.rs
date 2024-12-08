use kd_tree::KdTree;
use rand::Rng;
use std::{collections::HashSet, f32::consts::E};

use crate::{
    core::{
        environment::{Environment, ROUNDING_ERROR},
        light::Light,
        material::Material,
        object::Object,
        sampler::Sampler,
    },
    primitives::{
        colour::Colour,
        hit::Hit,
        photon::{Photon, PhotonOutcome, PhotonType},
        ray::Ray,
        vector::Vector,
        vertex::Vertex,
    },
    samplers::multi_jitter_sampler::MultiJitterSampler,
};

#[cfg(feature = "debugging")]
use crate::{materials::phong_material::PhongMaterial, objects::sphere_object::Sphere};
#[cfg(feature = "debugging")]
use std::sync::Arc;

const RECURSE_APPROXIMATE_THRESHOLD: u8 = 0;
const PHOTON_RECURSE: u8 = 3;
// const NUM_PHOTONS: u32 = 1_000_000;
// const NUM_PHOTONS: u32 = 202_500;
const NUM_PHOTONS: u32 = 90_000;
// const NUM_PHOTONS: u32 = 22_500;
// const NUM_PHOTONS: u32 = 2500;
const RADIANCE_MULTIPLIER: f32 = 1.0;
pub const PHOTON_SEARCH_RADIUS: f32 = 5.0;
const PHOTON_SEARCH_COUNT: u32 = 300;

pub type PhotonMap = KdTree<Photon>;

pub struct PhotonMaps {
    pub global: PhotonMap,
    pub caustic: PhotonMap,
}

impl PhotonMaps {
    pub fn global_radiance_estimate(
        &self,
        viewer: &Vector,
        hit: &Hit,
        material: &dyn Material,
        photon_types: &HashSet<PhotonType>,
    ) -> Colour {
        self.radiance_estimate(viewer, hit, material, photon_types, &self.global)
    }

    pub fn caustic_radiance_estimate(
        &self,
        viewer: &Vector,
        hit: &Hit,
        material: &dyn Material,
        photon_types: &HashSet<PhotonType>,
    ) -> Colour {
        self.radiance_estimate(viewer, hit, material, photon_types, &self.caustic)
    }

    fn radiance_estimate(
        &self,
        viewer: &Vector,
        hit: &Hit,
        material: &dyn Material,
        photon_types: &HashSet<PhotonType>,
        photon_map: &PhotonMap,
    ) -> Colour {
        let mut colour = Colour::default();

        let photons = photon_map.within_radius(
            &[
                hit.position.vector.x,
                hit.position.vector.y,
                hit.position.vector.z,
            ],
            PHOTON_SEARCH_RADIUS,
        );

        for photon in photons {
            if photon_types.contains(&photon.photon_type) {
                let distance = (photon.position.vector - hit.position.vector).length();
                const ALPHA: f32 = 0.918;
                const BETA: f32 = 1.953;
                let filter_weight = ALPHA
                    * (1.0
                        - ((1.0
                            - E.powf(
                                -BETA * (distance.powi(2) / (2.0 * PHOTON_SEARCH_RADIUS.powi(2))),
                            ))
                            / (1.0 - E.powf(-BETA))));
                colour += filter_weight
                    * photon.intensity
                    * material.brdf(viewer, &photon.direction, hit);
            }
        }

        RADIANCE_MULTIPLIER * colour
    }
}

fn russian_roulette(is_specular: bool, is_transparent: bool) -> (PhotonOutcome, f32) {
    let (r, t, a) = if is_transparent {
        (0.05, 0.65, 0.25)
    } else if is_specular {
        (0.95, 0.0, 0.05)
    } else {
        (0.2, 0.0, 0.8)
    };

    let mut rng = rand::thread_rng();
    let chance: f32 = rng.gen();

    if chance <= r {
        (PhotonOutcome::Reflect, r)
    } else if chance <= r + t {
        (PhotonOutcome::Transmit, t)
    } else {
        (PhotonOutcome::Absorb, a)
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
        photon_outcome: Option<PhotonOutcome>,
        recurse: u8,
    ) {
        let mut nearest_hit: Option<(Hit, usize)> = None;

        for (i, object) in self.objects.iter().enumerate() {
            if let Some(hit) = object.select_first_hit(ray) {
                if ((photon_outcome.is_some()
                    && photon_outcome.unwrap() == PhotonOutcome::Transmit)
                    || hit.entering)
                    && (nearest_hit.is_none() || hit.distance < nearest_hit.unwrap().0.distance)
                {
                    nearest_hit = Some((hit, i));
                }
            }
        }

        if nearest_hit.is_none() {
            return;
        }

        let (hit, object) = nearest_hit.unwrap();

        photon_map.push(Photon::new(
            hit.position,
            ray.direction,
            photon_intensity,
            photon_type,
        ));

        if recurse == 0 {
            return;
        }

        if let Some(object) = self.objects[object].get_material().clone() {
            let (photon_outcome, probability) =
                russian_roulette(object.is_specular(), object.is_transparent());
            match photon_outcome {
                PhotonOutcome::Reflect => {
                    let reflection_direction = ray.direction.reflection(&hit.normal).normalise();
                    let reflected_ray = Ray::new(
                        hit.position + ROUNDING_ERROR * reflection_direction,
                        reflection_direction,
                    );
                    self.photon_trace(
                        photon_map,
                        &reflected_ray,
                        PhotonType::IndirectIllumination,
                        photon_intensity / probability,
                        Some(PhotonOutcome::Reflect),
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
                        photon_intensity / probability,
                        Some(PhotonOutcome::Absorb),
                        recurse - 1,
                    );
                }
                PhotonOutcome::Transmit => {
                    if let Some(index_of_refraction) = object.get_index_of_refraction() {
                        let mut transmitted_ray = Ray::default();
                        transmitted_ray.direction = ray
                            .direction
                            .refraction(&hit.normal, index_of_refraction)
                            .normalise();
                        transmitted_ray.position =
                            hit.position + ROUNDING_ERROR * transmitted_ray.direction;
                        self.photon_trace(
                            photon_map,
                            &transmitted_ray,
                            PhotonType::IndirectIllumination,
                            photon_intensity / probability,
                            Some(PhotonOutcome::Transmit),
                            recurse - 1,
                        );
                    }
                }
            }
        }
    }

    /// Determine if a hit point is in shadow.
    fn is_point_in_shadow(
        &self,
        hit_position: Vertex,
        light_position: Option<Vertex>,
        light_direction: Vector,
    ) -> bool {
        // Second pass - Shadow.
        let photons = self.photon_maps.global.within_radius(
            &[
                hit_position.vector.x,
                hit_position.vector.y,
                hit_position.vector.z,
            ],
            PHOTON_SEARCH_RADIUS,
        );

        let mut num_direct_photons: u32 = 0;
        let mut num_shadow_photons: u32 = 0;
        for photon in photons {
            match photon.photon_type {
                PhotonType::ShadowPhoton => {
                    num_shadow_photons += 1;
                }
                PhotonType::DirectionIllumination => {
                    num_direct_photons += 1;
                }
                _ => {}
            }
        }

        if num_direct_photons + num_shadow_photons >= PHOTON_SEARCH_COUNT {
            let shadow_percent =
                num_shadow_photons as f32 / (num_direct_photons + num_shadow_photons) as f32;
            if shadow_percent == 1.0 {
                return true;
            } else if shadow_percent == 0.0 {
                return false;
            }
        }

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
    fn compute_lighting(&self, hit: &Hit, material: &dyn Material, recurse_depth: u8) -> Colour {
        let mut colour = Colour::default();

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
                    * material.compute_per_light(
                        self,
                        &viewer_direction,
                        &light_direction,
                        &hit,
                        recurse_depth,
                    );
            }
        }

        colour
    }

    fn estimate_direct_illumination(
        &self,
        viewer: &Vector,
        hit: &Hit,
        material: &dyn Material,
    ) -> Colour {
        self.photon_maps.global_radiance_estimate(
            viewer,
            hit,
            material,
            &HashSet::from([PhotonType::DirectionIllumination]),
        )
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
    fn setup(&mut self) {
        let mut sampler = MultiJitterSampler::new(NUM_PHOTONS, 1.0);

        let mut global_photon_map: Vec<Photon> = Vec::new();
        let mut caustic_photon_map: Vec<Photon> = Vec::new();

        let mut num_specular_objects: u32 = 0;
        for object in &self.objects {
            if let Some(material) = object.get_material().cloned() {
                if material.is_specular() {
                    num_specular_objects += 1;
                }
            }
        }

        for light in &self.lights {
            if let Some(light_position) = light.get_position() {
                for _ in 0..NUM_PHOTONS {
                    let sample_direction = sampler.sample_hemisphere();
                    let sign = if rand::random::<f32>() > 0.5 {
                        1.0
                    } else {
                        -1.0
                    };

                    let photon_direction = Vector::new(
                        sample_direction.x,
                        sample_direction.y * sign,
                        sample_direction.z,
                    );
                    let photon_ray = Ray::new(light_position, photon_direction);

                    let photon_power = 1.0 / NUM_PHOTONS as f32;

                    self.photon_trace(
                        &mut global_photon_map,
                        &photon_ray,
                        PhotonType::DirectionIllumination,
                        photon_power * light.get_intensity(),
                        None,
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
                        for _ in 0..(NUM_PHOTONS / num_specular_objects) {
                            let sample_direction = sampler.sample_hemisphere();
                            let target_point =
                                bounding_sphere.0 + (bounding_sphere.1 * sample_direction);
                            let photon_direction =
                                (target_point.vector - light_position.vector).normalise();
                            let photon_ray = Ray::new(light_position, photon_direction);

                            let photon_power = 1.0 / NUM_PHOTONS as f32;

                            self.photon_trace(
                                &mut caustic_photon_map,
                                &photon_ray,
                                PhotonType::DirectionIllumination,
                                photon_power * light.get_intensity(),
                                None,
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
        let mut colour = Colour::default();
        let mut depth = 0.0;

        if let Some((hit, object_index)) = self.ray_trace(ray) {
            depth = hit.distance;

            if let Some(material) = self.objects[object_index].get_material().cloned() {
                // Compute direct material contribution.
                colour += material.compute_once(self, ray, &hit, recurse);

                if recurse > RECURSE_APPROXIMATE_THRESHOLD {
                    colour += self.compute_lighting(&hit, material.as_ref(), recurse);
                } else {
                    // Second pass - direction illumination.
                    // At deeper levels, use photons to estimate direct light contribution.
                    colour +=
                        self.estimate_direct_illumination(&ray.direction, &hit, material.as_ref());
                }
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
