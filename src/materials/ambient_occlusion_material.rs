use crate::{
    core::{environment::Environment, material::Material, sampler::Sampler},
    primitives::{colour::Colour, hit::Hit, ray::Ray},
    samplers::multi_jitter_sampler::MultiJitterSampler,
};

/// AmbientOcclusionMaterial is a Material that computes ambient occlusion.
pub struct AmbientOcclusionMaterial {
    ambient: Colour,
    /// Number of samples to take for ambient occlusion.
    num_samples: u32,
    /// Minimum amount of ambient light to be considered.
    /// Value between 0.0 and 1.0.
    min_ambient_amount: f32,
}

impl AmbientOcclusionMaterial {
    pub fn new(ambient: Colour, num_samples: u32, min_ambient_amount: f32) -> Self {
        assert!(
            ((num_samples as f64).sqrt() as u32).pow(2) == num_samples,
            "Number of samples must be a square number."
        );

        Self {
            ambient,
            num_samples,
            min_ambient_amount,
        }
    }
}

impl Material for AmbientOcclusionMaterial {
    fn compute_once(
        &self,
        environment: &dyn Environment,
        _viewer: &Ray,
        hit: &Hit,
        _recurse: u8,
    ) -> Colour {
        let config = environment.config();
        let rounding_error = config.objects.rounding_error;
        let shadow_distance_limit = config.materials.shadow_distance_limit;

        let mut sampler: MultiJitterSampler = MultiJitterSampler::new(self.num_samples, 1.0);

        let mut ambient_occlusion_sum = 0.0;
        for _ in 0..self.num_samples {
            let sample = sampler.sample_hemisphere();
            let sample_direction = (hit.normal + sample).normalise();

            let shadow_ray = Ray::new(hit.position + rounding_error * hit.normal, sample_direction);
            if !environment.shadowtrace(&shadow_ray, shadow_distance_limit) {
                ambient_occlusion_sum += 1.0;
            } else {
                // If ray hits object, add only the minimum amount of ambient light.
                ambient_occlusion_sum += self.min_ambient_amount;
            }
        }

        let ambient_occlusion = ambient_occlusion_sum / (self.num_samples as f32);

        ambient_occlusion * self.ambient
    }
}
