use indicatif::ProgressBar;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

use crate::{
    primitives::{ray::Ray, Vector, Vertex},
    sampling::{traits::Sampler, MultiJitterSampler},
    Camera, FrameBuffer, Raytracer,
};

/// SamplingCamera with anti-aliasing through multiple samples per pixel.
/// Generic over Scene type - no trait objects needed!
pub struct SamplingCamera {
    pub width: u16,
    pub height: u16,
    /// Field of view. Distance from the camera to the image plane.
    pub fov: f32,
    /// Number of samples per pixel for anti-aliasing
    pub num_samples: u32,

    position: Vertex,
    /// Camera's forward vector.
    w: Vector,
    /// Camera's right vector.
    u: Vector,
    /// Camera's up vector.
    v: Vector,
}

impl SamplingCamera {
    pub fn new(fov: f32, position: Vertex, lookat: Vector, up: Vector, num_samples: u32) -> Self {
        let w = (position.vector - lookat).normalise();
        let u = w.cross(up).normalise();
        let v = u.cross(w);

        Self {
            width: u16::default(),
            height: u16::default(),
            fov,
            num_samples,
            position,
            w,
            u,
            v,
        }
    }

    fn get_pixel_ray(&self, x: f32, y: f32) -> Ray {
        // x and y are fractional pixel coordinates (can include jitter)
        // The camera (eye) is centred with the centre of the image, so we need
        // to shift the pixels up and left by half the width/height) to get the pixel
        // coordinates in respect to the camera.
        let mut x_v = (x + 0.5) - ((self.width as f32) / 2.0);
        let mut y_v = ((self.height as f32) / 2.0) - (y + 0.5);

        // Normalise.
        x_v /= self.width as f32;
        y_v /= self.height as f32;

        Ray::new(
            self.position,
            (x_v * self.u + y_v * self.v - self.fov * self.w).normalise(),
        )
    }
}

impl Default for SamplingCamera {
    fn default() -> Self {
        Self::new(
            0.5,
            Vertex::default(),
            Vector::new(0.0, 0.0, 1.0),
            Vector::new(0.0, 1.0, 0.0),
            16, // Default 16 samples (4x4 grid)
        )
    }
}

impl<S: Raytracer + Sync> Camera<S> for SamplingCamera {
    fn render(&mut self, scene: &S, fb: &mut FrameBuffer) {
        self.width = fb.width;
        self.height = fb.height;

        let fb = Arc::new(Mutex::new(fb));
        let num_samples = self.num_samples;

        let start_time = Instant::now();
        let pb = ProgressBar::new(self.height as u64);

        (0..self.height).into_par_iter().for_each(|y| {
            // Create sampler per thread
            let mut sampler = MultiJitterSampler::new(num_samples, 1.0, scene.config());

            for x in 0..self.width {
                let mut colour = crate::primitives::Colour::default();

                // Sample multiple times per pixel for anti-aliasing
                for _ in 0..num_samples {
                    let sample = sampler.sample_unit_square();
                    let ray = self.get_pixel_ray(x as f32 + sample.x, y as f32 + sample.y);

                    // Start recursion at depth 0
                    let (ray_colour, _depth) = scene.trace(&ray, 0);
                    colour += ray_colour;
                }

                // Average the samples
                colour /= num_samples as f32;

                let mut fb = fb.lock().unwrap();
                let _ = fb.plot_pixel(x as i32, y as i32, colour);
                let _ = fb.plot_depth(x as i32, y as i32, 0.0);
            }
            pb.inc(1);
        });

        pb.finish();
        let elapsed_time = start_time.elapsed();
        let total_seconds = elapsed_time.as_secs();
        println!(
            "Completed in {:02}:{:02}.{:03}",
            total_seconds / 60,
            total_seconds % 60,
            elapsed_time.subsec_millis()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_sampling_camera_fractional_coordinates() {
        let mut camera = SamplingCamera::new(
            0.5,
            Vertex::new(0.0, 0.0, 0.0, 1.0),
            Vector::new(0.0, 0.0, 1.0),
            Vector::new(0.0, 1.0, 0.0),
            4,
        );

        camera.width = 100;
        camera.height = 100;

        // Test fractional pixel coordinate (for jittered sampling)
        let ray = camera.get_pixel_ray(50.25, 50.75);

        assert_relative_eq!(ray.direction.length(), 1.0, epsilon = 1e-5);
        assert_eq!(ray.position.vector, camera.position.vector);
    }

    #[test]
    fn test_sampling_camera_multiple_samples_differ() {
        let mut camera = SamplingCamera::new(
            1.0,
            Vertex::new(0.0, 0.0, 0.0, 1.0),
            Vector::new(0.0, 0.0, 1.0),
            Vector::new(0.0, 1.0, 0.0),
            16,
        );

        camera.width = 100;
        camera.height = 100;

        // Different jittered positions should produce different rays
        let ray1 = camera.get_pixel_ray(50.0, 50.0);
        let ray2 = camera.get_pixel_ray(50.5, 50.5);

        // Rays should be slightly different due to jitter
        assert!(
            (ray1.direction.x - ray2.direction.x).abs() > 1e-6
                || (ray1.direction.y - ray2.direction.y).abs() > 1e-6
        );
    }
}
