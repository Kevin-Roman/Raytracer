use crate::{
    config::RaytracerConfig,
    geometry::traits::Intersection,
    primitives::{ray::Ray, Colour, Hit, Vector, Vertex},
    shading::{traits::Shader, MaterialStorage},
    Light, MaterialId, Raytracer, SceneBuilder, SceneMaterial, SceneObject,
};

pub struct Scene<'a> {
    pub objects: Vec<SceneObject>,
    pub lights: Vec<Light>,
    pub materials: MaterialStorage,
    pub config: &'a RaytracerConfig,
}

impl<'a> Scene<'a> {
    pub fn new(config: &'a RaytracerConfig) -> Self {
        Self {
            objects: Vec::new(),
            lights: Vec::new(),
            materials: MaterialStorage::new(),
            config,
        }
    }

    /// Trace and determine the nearest ray's hit in front of the camera.
    /// Returns the hit and the material ID.
    fn find_hit(&self, ray: &Ray) -> Option<(Hit, MaterialId)> {
        let mut nearest_hit: Option<(Hit, MaterialId)> = None;

        for object in &self.objects {
            if let Some(hit) = object.first_hit(ray) {
                if nearest_hit.is_none() || hit.distance < nearest_hit.unwrap().0.distance {
                    nearest_hit = Some((hit, object.material_id()));
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
        let rounding_error = self.config.objects.rounding_error;
        let to_light_direction = light_direction.negate();

        // Move the shadow ray point slightly along the ray (towards the light) to avoid self-shadowing.
        let shadow_ray = Ray::new(
            hit_position + rounding_error * to_light_direction,
            to_light_direction,
        );

        let shadow_limit = light_position
            .map(|light_position| (light_position.vector - shadow_ray.position.vector).length())
            .unwrap_or(f32::INFINITY);

        self.is_occluded(&shadow_ray, shadow_limit)
    }

    /// Compute contribution of all lights to the hit point.
    fn compute_lighting(&self, hit: &Hit, material: &SceneMaterial) -> Colour {
        let mut colour = Colour::new(0.0, 0.0, 0.0, 0.0);

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
}

impl<'a> Raytracer for Scene<'a> {
    fn trace(&self, ray: &Ray, recurse_depth: u8) -> (Colour, f32) {
        // Stop recursion if we've exceeded the max depth
        if recurse_depth >= self.config.camera.raytrace_recurse {
            return (Colour::default(), 0.0);
        }

        let mut colour = Colour::new(0.0, 0.0, 0.0, 0.0);
        let mut depth = 0.0;

        if let Some((hit, material_id)) = self.find_hit(ray) {
            depth = hit.distance;

            if let Some(material) = self.materials.get(material_id.0) {
                // Compute direct material contribution (ambient/emission).
                colour += material.shade_ambient(self, ray, &hit, recurse_depth);

                // Calculate contributions from lights.
                colour += self.compute_lighting(&hit, material);
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

impl<'a> SceneBuilder for Scene<'a> {
    fn add_object(&mut self, object: SceneObject) {
        self.objects.push(object);
    }

    fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    fn add_material(&mut self, material: SceneMaterial) -> MaterialId {
        self.materials.add(material)
    }

    fn config(&self) -> &RaytracerConfig {
        self.config
    }
}
