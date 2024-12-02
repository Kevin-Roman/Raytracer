use rand::{prelude::SliceRandom, Rng};
use std::f32::consts::PI;

use crate::{
    core::sampler::{Point2D, Sampler},
    primitives::vector::Vector,
};

const NUM_SETS: u32 = 4;

pub struct MultiJitterSampler {
    /// Must be a square number.
    num_samples: u32,
    num_sets: u32,
    samples: Vec<Point2D>,
    hemisphere_samples: Vec<Vector>,
    shuffled_indices: Vec<u32>,
    count: u32,
    jump: u32,
}

impl MultiJitterSampler {
    /// The distribution is controlled by the exponent `e` (how sparse/dense the vectors should be).
    /// The larger the value, the more vectors will be distributed towards the top of the hemisphere.
    pub fn new(num_samples: u32, e: f32) -> Self {
        assert!(
            ((num_samples as f64).sqrt() as u32).pow(2) == num_samples,
            "Number of samples must be a square number."
        );

        let samples: Vec<Point2D> = MultiJitterSampler::generate_samples(num_samples, NUM_SETS);
        let hemisphere_samples: Vec<Vector> =
            MultiJitterSampler::map_samples_hemisphere(&samples, e);
        let shuffled_indices = MultiJitterSampler::setup_shuffled_indices(num_samples, NUM_SETS);

        Self {
            num_samples,
            num_sets: NUM_SETS,
            samples,
            hemisphere_samples,
            shuffled_indices,
            count: 0,
            jump: 0,
        }
    }

    /// Multi-jittered sampling technique to generate a set of
    /// sample points that are evenly distributed within a unit square.
    ///
    /// Kenneth Chiu, Changyaw Wang, Peter Shirley,
    /// V.4. - Multi-Jittered Sampling,
    /// Editor(s): Paul S. Heckbert,
    /// Graphics Gems,
    /// Academic Press,
    /// 1994,
    /// Pages 370-374,
    /// ISBN 9780123361561,
    /// https://doi.org/10.1016/B978-0-12-336156-1.50045-8.
    /// (https://www.sciencedirect.com/science/article/pii/B9780123361561500458)
    /// Abstract: Jittered sampling patterns perform better than random sampling
    /// patterns because they limit the degree of clumping that can occur.
    /// Clumping can still be present in one-dimensional projections of jittered
    /// patterns, however. We present a simple method to reduce the clumping of the
    /// X-axis and Y-axis projections by imposing an additional N-rooks constraint
    /// on the jittered sampling pattern. The resulting sampling pattern can reduce
    /// the number of rays necessary for a satisfactory image when ray-tracing.
    fn multi_jitter_sampling(num_samples: u32) -> Vec<Point2D> {
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

    fn generate_samples(num_samples: u32, num_sets: u32) -> Vec<Point2D> {
        let mut samples: Vec<Point2D> = Vec::with_capacity((num_samples * num_sets) as usize);

        for _ in 0..num_sets {
            samples.extend(MultiJitterSampler::multi_jitter_sampling(num_samples));
        }

        samples
    }

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
    fn sample_index(&mut self) -> usize {
        let mut rng = rand::thread_rng();

        // Update the jump if the count is a multiple of num_samples
        if self.count % self.num_samples == 0 {
            self.jump = rng.gen_range(0..self.num_sets) * self.num_samples;
        }

        // Compute the sample index
        let index = (self.jump
            + self.shuffled_indices[(self.jump + self.count % self.num_samples) as usize])
            as usize;

        self.count += 1;

        index
    }
}

impl Sampler for MultiJitterSampler {
    fn sample_unit_square(&mut self) -> Point2D {
        let index = self.sample_index();
        self.samples[index]
    }

    fn sample_hemisphere(&mut self) -> Vector {
        let index = self.sample_index();
        self.hemisphere_samples[index]
    }
}
