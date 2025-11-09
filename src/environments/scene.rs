use core::f32;
use std::sync::Arc;

use crate::{
    core::{
        environment::{Environment, ROUNDING_ERROR},
        light::Light,
        material::Material,
        object::Object,
    },
    primitives::{colour::Colour, hit::Hit, ray::Ray, vector::Vector, vertex::Vertex},
};

#[derive(Default)]
pub struct Scene {
    pub objects: Vec<Box<dyn Object>>,
    pub lights: Vec<Box<dyn Light>>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }

    /// Trace and determine the nearest ray's hit in front of the camera.
    ///
    /// Returns the hit and the index of the object that was hit.
    fn trace(&self, ray: &Ray) -> Option<(Hit, usize)> {
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
    fn compute_lighting(
        &self,
        hit: &Hit,
        material: &Arc<dyn Material>,
        recurse_depth: u8,
    ) -> Colour {
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
                    * material.compute_per_light(
                        self,
                        &viewer_direction,
                        &light_direction,
                        hit,
                        recurse_depth,
                    );
            }
        }

        colour
    }
}

impl Environment for Scene {
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
                colour += self.compute_lighting(&hit, &material, recurse);
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
