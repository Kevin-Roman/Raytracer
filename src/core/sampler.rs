use rand::{prelude::SliceRandom, Rng};
use std::f32::consts::PI;

use crate::primitives::vector::Vector;

#[derive(Clone, Copy, Debug)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

pub trait Sampler {
    fn sample_unit_square(&mut self) -> Point2D;

    fn sample_hemisphere(&mut self) -> Vector;

    /// Converts 2D sample points into 3D vectors that are distributed over a hemisphere.
    ///
    /// Suffern, K. (2016) Ray Tracing from the Ground Up. CRC Press.
    /// Chapter 7.3: Mapping Samples to a Hemisphere - Implementation.
    /// Page 129,
    /// ISBN 9781568812724,
    fn map_samples_hemisphere(samples: &Vec<Point2D>, e: f32) -> Vec<Vector> {
        let mut hemisphere_samples: Vec<Vector> = Vec::with_capacity(samples.len());

        for sample in samples {
            let cos_phi = f32::cos(2.0 * PI * sample.x);
            let sin_phi = f32::sin(2.0 * PI * sample.x);
            let cos_theta = (1.0 - sample.y).powf(1.0 / (e + 1.0));
            let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
            let x = sin_theta * cos_phi;
            let y = cos_theta;
            let z = sin_theta * sin_phi;

            hemisphere_samples.push(Vector::new(x, y, z));
        }

        hemisphere_samples
    }

    /// Shuffles the indices of the samples.
    ///
    /// Suffern, K. (2016) Ray Tracing from the Ground Up. CRC Press.
    /// Chapter 7.3: Shuffling the indices.
    /// Page 111,
    /// ISBN 9781568812724,
    fn setup_shuffled_indices(num_samples: u32, num_sets: u32) -> Vec<u32> {
        let mut shuffled_indices: Vec<u32> = Vec::with_capacity((num_samples * num_sets) as usize);

        let mut indices: Vec<u32> = (0..num_samples).collect();

        for _ in 0..num_sets {
            indices.shuffle(&mut rand::thread_rng());

            for j in 0..num_samples {
                shuffled_indices.push(indices[j as usize]);
            }
        }

        shuffled_indices
    }

    /// Returns the index of the sample to be used.
    ///
    /// Suffern, K. (2016) Ray Tracing from the Ground Up. CRC Press.
    /// Chapter 7.3: Mapping Samples to a Hemisphere - Implementation.
    /// Page 111,
    /// ISBN 9781568812724,
    fn sample_index(
        num_samples: u32,
        num_sets: u32,
        shuffled_indices: &Vec<u32>,
        count: &mut u32,
        jump: &mut u32,
    ) -> usize {
        let mut rng = rand::thread_rng();

        // Update the jump if the count is a multiple of num_samples.
        if *count % num_samples == 0 {
            *jump = rng.gen_range(0..num_sets) * num_samples;
        }

        // Compute the sample index.
        let index = (*jump + shuffled_indices[(*jump + *count % num_samples) as usize]) as usize;

        *count += 1;

        index
    }
}
