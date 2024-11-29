use core::f32;
use std::sync::Arc;

use kd_tree::KdTree;

use crate::{
    core::{environment::Environment, light::Light, material::Material, object::Object},
    primitives::{
        colour::Colour, hit::Hit, photon::Photon, ray::Ray, vector::Vector, vertex::Vertex,
    },
};

/// Small rounding error used to move shadow ray point along the ray by a small amount
/// in case the shadow position is behind the hit (due to floating point precision).
pub const ROUNDING_ERROR: f32 = 0.001;

pub struct PhotonScene {
    pub objects: Vec<Box<dyn Object>>,
    pub lights: Vec<Box<dyn Light>>,
    pub global_photon_map: Option<KdTree<Photon>>,
    pub caustic_photon_map: Option<KdTree<Photon>>,
}

impl PhotonScene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            lights: Vec::new(),
            global_photon_map: None,
            caustic_photon_map: None,
        }
    }

    /// Trace and determine the nearest ray's hit in front of the camera.
    ///
    /// Returns the hit and the index of the object that was hit.
    pub fn trace(&self, ray: &Ray) -> Option<(Hit, usize)> {
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

        for light_index in 0..self.lights.len() {
            let viewer_direction = (-hit.position.vector).normalise();
            let (light_position, light_direction, is_lit) =
                self.lights[light_index].get_direction(hit.position);

            // Skip lights that are facing the wrong direction.
            if light_direction.dot(&hit.normal) > 0.0 {
                continue;
            }

            if is_lit && !self.is_point_in_shadow(hit.position, light_position, light_direction) {
                let intensity = self.lights[light_index].get_intensity(hit.position);
                colour += intensity
                    * material.compute_per_light(&viewer_direction, &light_direction, &hit);
            }
        }

        colour
    }
}

impl Environment for PhotonScene {
    /// Pass 1: Constructing the Photon Maps.
    fn initialise(&mut self) {}

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

    fn raytrace(&self, ray: &Ray, recurse: u8) -> (Colour, f32) {
        let mut colour = Colour::new(0.0, 0.0, 0.0, 0.0);
        let mut depth = 0.0;

        if let Some((hit, object_index)) = self.trace(ray) {
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
}
