use rand::Rng;

use std::f32::consts::PI;

use crate::{
    core::sampler::{Point2D, Sampler},
    primitives::vector::Vector,
};

pub struct MultiJitterSampler {
    num_samples: u16,
    samples: Vec<Point2D>,
}

impl MultiJitterSampler {
    pub fn new(num_samples: u16) -> Self {
        assert!(
            ((num_samples as f64).sqrt() as u32).pow(2) == num_samples as u32,
            "Number of samples must be a square number."
        );

        let samples: Vec<Point2D> = MultiJitterSampler::samples(num_samples);

        Self {
            num_samples,
            samples,
        }
    }
}

impl Sampler for MultiJitterSampler {
    /// Multi-jittered sampling technique to generate a set of
    /// sample points that are evenly distributed within a unit square.
    fn samples(num_samples: u16) -> Vec<Point2D> {
        let mut rng = rand::thread_rng();
        let sqrt_samples = (num_samples as f32).sqrt() as u32;

        let mut points: Vec<Point2D> = Vec::with_capacity(num_samples as usize);
        let subcell_width = 1.0 / (num_samples as f32);

        // Generate initial points with jittering.
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

        // Shuffle x coordinates within each column.
        for i in 0..sqrt_samples {
            for j in 0..sqrt_samples {
                let k = rng.gen_range(j..sqrt_samples);
                let t = points[(i * sqrt_samples + j) as usize].x;
                points[(i * sqrt_samples + j) as usize].x =
                    points[(i * sqrt_samples + k) as usize].x;
                points[(i * sqrt_samples + k) as usize].x = t;
            }
        }

        // Shuffle y coordinates within each row.
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

    /// Converts 2D sample points into 3D vectors that are distributed over a hemisphere.
    /// The distribution is controlled by the exponent `e` (how sparse/dense the vectors should be).
    fn hemisphere_sampler(&self, e: f32) -> Vec<Vector> {
        let mut hemisphere_samples: Vec<Vector> = Vec::with_capacity(self.num_samples as usize);

        for sample in &self.samples {
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
