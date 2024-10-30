use super::{
    colour::Colour, environment::Environment, hit::Hit, light::Light, object::Object, ray::Ray,
};

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
        for object in self.objects.iter_mut() {
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

            let material = self.objects[object_index].get_material().unwrap();

            colour += material.compute_once(ray, &hit, recurse);

            for light in &self.lights {
                let viewer = (-hit.position.vector).normalise();

                let (ldir, mut lit) = light.get_direction(hit.position);

                if ldir.dot(&hit.normal) > 0.0 {
                    // Light is facing wrong way.
                    lit = false;
                }

                // Put the shadow check here, if lit==true and in shadow, set lit=false.

                if lit {
                    let intensity = light.get_intensity(hit.position);
                    colour += intensity + material.compute_per_light(viewer, &hit, ldir);
                }
            }
        }

        (colour, depth)
    }
}
