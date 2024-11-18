use rand::Rng;

use core::f32;
use std::f32::consts::PI;

use crate::{
    core::{environment::Environment, material::Material},
    environments::scene::SMALL_ROUNDING_ERROR,
    primitives::{colour::Colour, hit::Hit, ray::Ray, vector::Vector},
};

const SHADOW_DISTANCE_LIMIT: f32 = 10.0;

pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

pub struct AmbientOcclusionMaterial {
    ambient: Colour,
    samples: usize,
    min_amount: f32,
}

impl AmbientOcclusionMaterial {
    pub fn new(ambient: Colour, samples: usize, min_amount: f32) -> Self {
        assert!(
            ((samples as f64).sqrt() as u32).pow(2) == samples as u32,
            "Number of samples must be a square number."
        );

        Self {
            ambient,
            samples,
            min_amount,
        }
    }

    fn multi_jitter(&self) -> Vec<Point2D> {
        let mut rng = rand::thread_rng();
        let sqrt_samples = (self.samples as f32).sqrt() as i32;

        let mut points: Vec<Point2D> = Vec::with_capacity(self.samples as usize);
        let subcell_width = 1.0 / (self.samples as f32);

        for i in 0..sqrt_samples {
            for j in 0..sqrt_samples {
                points.push(Point2D {
                    x: ((i * sqrt_samples) as f32) * subcell_width
                        + (j as f32) * subcell_width
                        + rng.gen_range(0.0..subcell_width),
                    y: ((j * sqrt_samples) as f32) * subcell_width
                        + (i as f32) * subcell_width
                        + rng.gen_range(0.0..subcell_width),
                })
            }
        }

        for i in 0..sqrt_samples {
            for j in 0..sqrt_samples {
                let k = rng.gen_range(j..sqrt_samples);
                let t = points[(i * sqrt_samples + j) as usize].x;
                points[(i * sqrt_samples + j) as usize].x =
                    points[(i * sqrt_samples + k) as usize].x;
                points[(i * sqrt_samples + k) as usize].x = t;
            }
        }

        for i in 0..sqrt_samples {
            for j in 0..sqrt_samples {
                let k = rng.gen_range(j..sqrt_samples);
                let t = points[(j * sqrt_samples + i) as usize].y;
                points[(j * sqrt_samples + i) as usize].y =
                    points[(k * sqrt_samples + i) as usize].y;
                points[(k * sqrt_samples + i) as usize].y = t;
            }
        }

        points
    }

    fn hemisphere_sampler(&self, e: f32) -> Vec<Vector> {
        let samples: Vec<Point2D> = self.multi_jitter();
        let mut hemisphere_samples: Vec<Vector> = Vec::with_capacity(self.samples as usize);

        for sample in &samples {
            let cos_phi = f32::cos(2.0 * PI * sample.x);
            let sin_phi = f32::sin(2.0 * PI * sample.x);
            let cos_theta = (1.0 - sample.y).powf(1.0 / (e + 1.0));
            let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
            let x = sin_theta * cos_phi;
            let y = sin_theta * sin_phi;
            let z = cos_theta;

            hemisphere_samples.push(Vector::new(x, y, z));
        }

        hemisphere_samples
    }
}

impl Material for AmbientOcclusionMaterial {
    fn compute_once(
        &self,
        environment: &mut dyn Environment,
        _viewer: &Ray,
        hit: &Hit,
        _recurse: u8,
    ) -> Colour {
        let samples = self.hemisphere_sampler(1.0);

        let mut ambient_occlusion_sum = 0.0;
        for sample in &samples {
            let sample_direction = (hit.normal + *sample).normalise();

            let shadow_ray = Ray::new(
                hit.position + SMALL_ROUNDING_ERROR * hit.normal,
                sample_direction,
            );
            if !environment.shadowtrace(&shadow_ray, SHADOW_DISTANCE_LIMIT) {
                ambient_occlusion_sum += 1.0;
            } else {
                ambient_occlusion_sum += self.min_amount;
            }
        }

        let ambient_occlusion = (ambient_occlusion_sum as f32) / (samples.len() as f32);

        ambient_occlusion * self.ambient
    }

    fn compute_per_light(&self, _viewer: &Vector, _light_direction: &Vector, _hit: &Hit) -> Colour {
        Colour::default()
    }
}
