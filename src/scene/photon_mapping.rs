use kd_tree::KdTree;
use rand::Rng;
use std::{collections::HashSet, f32::consts::E};

use crate::{
    config::RaytracerConfig,
    geometry::traits::{Bounded, Intersection},
    primitives::{
        photon::{Photon, PhotonOutcome, PhotonType},
        ray::Ray,
        Colour, Hit, Vector, Vertex,
    },
    sampling::{traits::Sampler, MultiJitterSampler},
    shading::traits::Shader,
    Light, Material, Raytracer, SceneBuilder, SceneObject,
};

#[cfg(feature = "debugging")]
use crate::geometry::sphere::Sphere;

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
        material: &Material,
        photon_types: &HashSet<PhotonType>,
        photon_search_radius: f32,
    ) -> Colour {
        self.radiance_estimate(
            viewer,
            hit,
            material,
            photon_types,
            &self.global,
            photon_search_radius,
        )
    }

    pub fn caustic_radiance_estimate(
        &self,
        viewer: &Vector,
        hit: &Hit,
        material: &Material,
        photon_types: &HashSet<PhotonType>,
        photon_search_radius: f32,
    ) -> Colour {
        self.radiance_estimate(
            viewer,
            hit,
            material,
            photon_types,
            &self.caustic,
            photon_search_radius,
        )
    }

    fn radiance_estimate(
        &self,
        viewer: &Vector,
        hit: &Hit,
        material: &Material,
        photon_types: &HashSet<PhotonType>,
        photon_map: &PhotonMap,
        photon_search_radius: f32,
    ) -> Colour {
        let mut colour = Colour::default();

        let photons = photon_map.within_radius(
            &[
                hit.position.vector.x,
                hit.position.vector.y,
                hit.position.vector.z,
            ],
            photon_search_radius,
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
                                -BETA * (distance.powi(2) / (2.0 * photon_search_radius.powi(2))),
                            ))
                            / (1.0 - E.powf(-BETA))));
                colour += filter_weight
                    * photon.intensity
                    * material.brdf(viewer, &photon.direction, hit);
            }
        }

        colour
    }
}

fn russian_roulette(is_specular: bool, is_transparent: bool) -> (PhotonOutcome, f32) {
    let (r, t, a) = if is_transparent {
        (0.05, 0.7, 0.25)
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

/// Modern idiomatic PhotonScene implementation.
/// Uses composition and concrete types instead of trait objects.
pub struct PhotonScene<'a> {
    pub objects: Vec<SceneObject>,
    pub lights: Vec<Light>,
    pub photon_maps: PhotonMaps,
    pub config: &'a RaytracerConfig,
}

impl<'a> PhotonScene<'a> {
    pub fn new(config: &'a RaytracerConfig) -> Self {
        Self {
            objects: Vec::new(),
            lights: Vec::new(),
            photon_maps: PhotonMaps {
                global: KdTree::default(),
                caustic: KdTree::default(),
            },
            config,
        }
    }

    /// Trace and determine the nearest ray's hit in front of the camera.
    /// Returns the hit and a reference to the material.
    fn find_hit(&self, ray: &Ray) -> Option<(Hit, &Material)> {
        let mut nearest_hit: Option<(Hit, &Material)> = None;

        for object in &self.objects {
            if let Some(hit) = object.first_hit(ray) {
                if nearest_hit.is_none() || hit.distance < nearest_hit.unwrap().0.distance {
                    nearest_hit = Some((hit, object.material()));
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
        let mut nearest_hit: Option<(Hit, &Material)> = None;

        for object in &self.objects {
            if let Some(hit) = object.first_hit(ray) {
                // Only consider interacts that are entering the object, or exiting if the photon
                // is transmitted.
                if (hit.entering
                    || (photon_outcome.is_some()
                        && photon_outcome.unwrap() == PhotonOutcome::Transmit))
                    && (nearest_hit.is_none() || hit.distance < nearest_hit.unwrap().0.distance)
                {
                    nearest_hit = Some((hit, object.material()));
                }
            }
        }

        if nearest_hit.is_none() {
            return;
        }

        let (hit, material) = nearest_hit.unwrap();

        photon_map.push(Photon::new(
            hit.position,
            ray.direction,
            photon_intensity,
            photon_type,
        ));

        if recurse == 0 {
            return;
        }

        let (photon_outcome, probability) =
            russian_roulette(material.is_specular(), material.is_transparent());
        match photon_outcome {
            PhotonOutcome::Reflect => {
                let reflection_direction = ray.direction.reflection(hit.normal).normalise();
                let reflected_ray = Ray::new(
                    hit.position + self.config.objects.rounding_error * reflection_direction,
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
                let absorbed_ray = Ray::new(
                    hit.position + self.config.objects.rounding_error * ray.direction,
                    ray.direction,
                );
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
                if let Some(index_of_refraction) = material.index_of_refraction() {
                    let mut transmitted_ray = Ray::default();
                    transmitted_ray.direction = ray
                        .direction
                        .refraction(hit.normal, index_of_refraction)
                        .normalise();
                    transmitted_ray.position = hit.position
                        + self.config.objects.rounding_error * transmitted_ray.direction;
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

    /// Determine if a hit point is in shadow.
    fn is_point_in_shadow(
        &self,
        hit_position: Vertex,
        light_position: Option<Vertex>,
        light_direction: Vector,
    ) -> bool {
        if self.config.photon_mapping.use_shadow_estimation {
            // Second pass - Shadow.
            let photons = self.photon_maps.global.within_radius(
                &[
                    hit_position.vector.x,
                    hit_position.vector.y,
                    hit_position.vector.z,
                ],
                self.config.photon_mapping.photon_search_radius,
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

            if num_direct_photons + num_shadow_photons
                >= self.config.photon_mapping.photon_search_count
            {
                let shadow_percent =
                    num_shadow_photons as f32 / (num_direct_photons + num_shadow_photons) as f32;
                if shadow_percent == 1.0 {
                    return true;
                } else if shadow_percent == 0.0 {
                    return false;
                }
            }
        }

        let to_light_direction = light_direction.negate();
        // Move the shadow ray point slightly along the ray (towards the light) to avoid self-shadowing.
        let shadow_ray = Ray::new(
            hit_position + self.config.objects.rounding_error * to_light_direction,
            to_light_direction,
        );

        let shadow_limit = light_position
            .map(|light_position| (light_position.vector - shadow_ray.position.vector).length())
            .unwrap_or(f32::INFINITY);

        self.is_occluded(&shadow_ray, shadow_limit)
    }

    /// Compute contribution of all lights to the hit point.
    fn compute_lighting(&self, hit: &Hit, material: &Material) -> Colour {
        let mut colour = Colour::default();

        for light in &self.lights {
            let viewer_direction = (-hit.position.vector).normalise();
            let (light_position, light_direction, is_lit) = light.get_direction(hit.position);

            // Skip lights that are facing the wrong direction.
            if light_direction.dot(hit.normal) > 0.0 {
                continue;
            }

            if is_lit && !self.is_point_in_shadow(hit.position, light_position, light_direction) {
                let intensity = light.get_intensity();
                colour += intensity
                    * material.shade_light(self, &viewer_direction, &light_direction, hit);
            }
        }

        colour
    }

    fn estimate_indirect_illumination(
        &self,
        viewer: &Vector,
        hit: &Hit,
        material: &Material,
    ) -> Colour {
        self.photon_maps.global_radiance_estimate(
            viewer,
            hit,
            material,
            &HashSet::from([PhotonType::IndirectIllumination]),
            self.config.photon_mapping.photon_search_radius,
        )
    }

    fn estimate_caustics(&self, viewer: &Vector, hit: &Hit, material: &Material) -> Colour {
        self.photon_maps.caustic_radiance_estimate(
            viewer,
            hit,
            material,
            &HashSet::from([PhotonType::IndirectIllumination]),
            self.config.photon_mapping.photon_search_radius,
        )
    }

    #[cfg(feature = "debugging")]
    fn display_photons(&mut self, photon_map: &Vec<Photon>) {
        // Testing purposes
        for photon in photon_map {
            let mat_id = match photon.photon_type {
                PhotonType::DirectionIllumination => self.add_material(Material::phong(
                    Colour::new(0.0, 1.0, 0.0, 1.0),
                    Colour::default(),
                    Colour::default(),
                    1.0,
                )),
                PhotonType::ShadowPhoton => self.add_material(Material::phong(
                    Colour::new(0.0, 0.0, 1.0, 1.0),
                    Colour::default(),
                    Colour::default(),
                    1.0,
                )),
                PhotonType::IndirectIllumination => self.add_material(Material::phong(
                    Colour::new(1.0, 0.0, 0.0, 1.0),
                    Colour::default(),
                    Colour::default(),
                    1.0,
                )),
            };

            let sphere = Sphere::new(photon.position, 0.1).with_material(mat_id);
            self.add_object(SceneObject::from(sphere));
        }
    }

    /// Pass 1: Constructing the Photon Maps.
    pub fn setup(&mut self) {
        let mut sampler =
            MultiJitterSampler::new(self.config.photon_mapping.num_photons, 1.0, self.config);

        let mut global_photon_map: Vec<Photon> = Vec::new();
        let mut caustic_photon_map: Vec<Photon> = Vec::new();

        for light in &self.lights {
            if let Some(light_position) = light.get_position() {
                // Create global map.
                for _ in 0..self.config.photon_mapping.num_photons {
                    let sample_direction = sampler.sample_hemisphere();
                    // Project samples onto a sphere, so that the photons are emitted in all directions.
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

                    let photon_power = 1.0 / self.config.photon_mapping.num_photons as f32;

                    self.photon_trace(
                        &mut global_photon_map,
                        &photon_ray,
                        PhotonType::DirectionIllumination,
                        photon_power * light.get_intensity(),
                        None,
                        self.config.photon_mapping.photon_recurse,
                    );
                }

                // Create caustic map.
                for object in &self.objects {
                    let material = object.material();
                    if !material.is_specular() {
                        continue;
                    }

                    if let Some(bounding_sphere) = object.bounding_sphere() {
                        for _ in 0..self.config.photon_mapping.num_photons {
                            let sample_direction = sampler.sample_hemisphere();
                            // Shoot photons towards the object.
                            let target_point =
                                bounding_sphere.0.vector + (bounding_sphere.1 * sample_direction);
                            let photon_direction =
                                (target_point - light_position.vector).normalise();
                            let photon_ray = Ray::new(light_position, photon_direction);

                            let photon_power = 1.0 / self.config.photon_mapping.num_photons as f32;

                            self.photon_trace(
                                &mut caustic_photon_map,
                                &photon_ray,
                                PhotonType::DirectionIllumination,
                                photon_power * light.get_intensity(),
                                None,
                                self.config.photon_mapping.photon_recurse,
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

        // Construct the kd-tree. It is an efficient data structure for nearest neighbour searches (O(log n)).
        self.photon_maps.global = KdTree::par_build_by_ordered_float(global_photon_map);
        self.photon_maps.caustic = KdTree::par_build_by_ordered_float(caustic_photon_map);
    }

    /// Get the photon maps
    pub fn get_photon_maps(&self) -> &PhotonMaps {
        &self.photon_maps
    }
}

impl<'a> Raytracer for PhotonScene<'a> {
    fn trace(&self, ray: &Ray, recurse_depth: u8) -> (Colour, f32) {
        // Stop recursion if we've exceeded the max depth
        if recurse_depth >= self.config.camera.raytrace_recurse {
            return (Colour::default(), 0.0);
        }

        let mut colour = Colour::default();
        let mut depth = 0.0;

        if let Some((hit, material)) = self.find_hit(ray) {
            depth = hit.distance;

            // Compute direct material contribution (ambient/emission/reflection/refraction).
            colour += material.shade_ambient(self, ray, &hit, recurse_depth);

            // Always compute direct lighting from light sources
            colour += self.compute_lighting(&hit, material);

            // Add photon contributions for global illumination effects
            if recurse_depth <= self.config.photon_mapping.recurse_approximate_threshold {
                // At shallow depths, add photon-based indirect illumination and caustics
                let viewer = &ray.direction.negate();
                colour += self.estimate_indirect_illumination(viewer, &hit, material);
                colour += self.estimate_caustics(viewer, &hit, material);
            }
        }

        (colour, depth)
    }

    fn is_occluded(&self, ray: &Ray, max_distance: f32) -> bool {
        for object in &self.objects {
            if let Some(hit) = object.first_hit(ray) {
                if 0.0 < hit.distance && hit.distance < max_distance {
                    return true;
                }
            }
        }

        false
    }

    fn config(&self) -> &RaytracerConfig {
        self.config
    }
}

impl<'a> SceneBuilder for PhotonScene<'a> {
    fn add_object(&mut self, object: SceneObject) {
        self.objects.push(object);
    }

    fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    fn config(&self) -> &RaytracerConfig {
        self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::sphere::Sphere;

    fn test_config() -> RaytracerConfig {
        RaytracerConfig::default()
    }

    #[test]
    fn test_photon_maps_initialization() {
        let config = test_config();
        let scene = PhotonScene::new(&config);

        let maps = scene.get_photon_maps();
        // Photon maps should be empty initially
        assert_eq!(maps.global.len(), 0);
        assert_eq!(maps.caustic.len(), 0);
    }

    #[test]
    fn test_russian_roulette_probabilities() {
        let (_, prob1) = russian_roulette(false, false);
        let (_, prob2) = russian_roulette(true, false);
        let (_, prob3) = russian_roulette(false, true);

        // Each probability should be valid
        assert!(prob1 > 0.0 && prob1 <= 1.0);
        assert!(prob2 > 0.0 && prob2 <= 1.0);
        assert!(prob3 > 0.0 && prob3 <= 1.0);
    }

    #[test]
    fn test_photon_scene_find_hit() {
        let config = test_config();
        let mut scene = PhotonScene::new(&config);

        let material = Material::phong(
            Colour::new(0.1, 0.1, 0.1, 1.0),
            Colour::new(0.8, 0.8, 0.8, 1.0),
            Colour::new(1.0, 1.0, 1.0, 1.0),
            32.0,
        );
        let sphere = Sphere::new(Vertex::new(0.0, 0.0, 5.0, 1.0), 1.0, material);
        scene.add_object(SceneObject::from(sphere));

        let ray = Ray::new(Vertex::new(0.0, 0.0, 0.0, 1.0), Vector::new(0.0, 0.0, 1.0));

        let result = scene.find_hit(&ray);
        assert!(result.is_some());
    }

    #[test]
    fn test_photon_scene_trace_returns_valid() {
        let config = test_config();
        let mut scene = PhotonScene::new(&config);

        let material = Material::phong(
            Colour::new(0.1, 0.1, 0.1, 1.0),
            Colour::new(0.8, 0.8, 0.8, 1.0),
            Colour::new(1.0, 1.0, 1.0, 1.0),
            32.0,
        );
        let sphere = Sphere::new(Vertex::new(0.0, 0.0, 5.0, 1.0), 1.0, material);
        scene.add_object(SceneObject::from(sphere));

        let ray = Ray::new(Vertex::new(0.0, 0.0, 0.0, 1.0), Vector::new(0.0, 0.0, 1.0));

        let (colour, depth) = scene.trace(&ray, 0);

        // Should hit something
        assert!(depth > 0.0);
        assert!(colour.r.is_finite());
        assert!(colour.g.is_finite());
        assert!(colour.b.is_finite());
    }

    #[test]
    fn test_photon_scene_recursion_limit() {
        let config = test_config();
        let scene = PhotonScene::new(&config);

        let ray = Ray::new(Vertex::new(0.0, 0.0, 0.0, 1.0), Vector::new(0.0, 0.0, 1.0));

        // Should stop at max recursion depth
        let max_depth = config.camera.raytrace_recurse;
        let (colour, _) = scene.trace(&ray, max_depth);

        // At max depth, should return default (black) colour
        assert_eq!(colour.r, 0.0);
        assert_eq!(colour.g, 0.0);
        assert_eq!(colour.b, 0.0);
    }
}
