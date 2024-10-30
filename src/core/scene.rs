use core::f32;

use super::{
    colour::Colour, environment::Environment, hit::Hit, light::Light, object::Object, ray::Ray,
};

/// Small rounding error used to move start point of shadow ray along ray by a small amount
/// in case the shadow position is behind the hit (due to floating point precision).
const SMALL_ROUNDING_ERROR: f32 = 0.0001;

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

    pub fn trace(&mut self, ray: &Ray) -> Option<(Hit, usize)> {
        let mut best_hit: Option<(Hit, usize)> = None;

        for (i, object) in self.objects.iter_mut().enumerate() {
            object.intersection(ray);
            if let Some(hit) = object.select_first_hit() {
                if best_hit.is_none() || hit.t < best_hit.unwrap().0.t {
                    best_hit = Some((hit, i));
                }
            }
        }

        best_hit
    }
}

impl Environment for Scene {
    fn shadowtrace(&mut self, ray: &Ray, limit: f32) -> bool {
        for object in &mut self.objects {
            object.intersection(ray);
            if let Some(hit) = object.select_first_hit() {
                if 0.00000001 < hit.t && hit.t < limit {
                    return true;
                }
            }
        }

        false
    }

    fn raytrace(&mut self, ray: &Ray, recurse: i32) -> (Colour, f32) {
        let mut colour = Colour::new(0.0, 0.0, 0.0, 0.0);
        let mut depth = 0.0;

        if let Some((hit, object_index)) = self.trace(ray) {
            depth = hit.t;

            if let Some(material) = self.objects[object_index].get_material() {
                colour += material.compute_once(ray, &hit, recurse);
            }

            for light_index in 0..self.lights.len() {
                let viewer = (-hit.position.vector).normalise();

                let (light_direction, mut lit) =
                    self.lights[light_index].get_direction(hit.position);

                if light_direction.dot(&hit.normal) > 0.0 {
                    // Light is facing wrong way.
                    lit = false;
                }

                if lit {
                    let shadow_ray = Ray::new(
                        hit.position + SMALL_ROUNDING_ERROR * -light_direction,
                        -light_direction,
                    );

                    if self.shadowtrace(&shadow_ray, f32::INFINITY) {
                        lit = false;
                    }
                }

                if lit {
                    let intensity = self.lights[light_index].get_intensity(hit.position);

                    if let Some(material) = self.objects[object_index].get_material() {
                        colour +=
                            intensity * material.compute_per_light(&viewer, &light_direction, &hit);
                    }
                }
            }
        }

        (colour, depth)
    }
}
