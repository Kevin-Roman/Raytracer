use indicatif::ProgressBar;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

use crate::{
    core::{
        camera::{Camera, RAYTRACE_RECURSE},
        environment::Environment,
        framebuffer::FrameBuffer,
    },
    primitives::{ray::Ray, vector::Vector, vertex::Vertex},
};

/// Full camera allows a camera to be placed in space with a lookat and up direction
/// as well as the field of view. It loops over the pixels in a framebuffer and computes
/// a ray that is then passed to the environment.
pub struct FullCamera {
    pub width: u16,
    pub height: u16,
    /// Field of view. Distance from the camera to the image plane.
    pub fov: f32,

    position: Vertex,
    /// Camera's forward vector.
    w: Vector,
    /// Camera's right vector.
    u: Vector,
    /// Camera's up vector.
    v: Vector,
}

impl FullCamera {
    pub fn new(fov: f32, position: Vertex, lookat: Vector, up: Vector) -> Self {
        let w = (position.vector - lookat).normalise();
        let u = w.cross(&up).normalise();
        let v = u.cross(&w);

        Self {
            width: u16::default(),
            height: u16::default(),
            fov,
            position,
            w,
            u,
            v,
        }
    }

    fn get_pixel_ray(&self, x: u16, y: u16) -> Ray {
        // Add 0.5 to pixel's x and y coordinates to get middle of the pixel.
        // The camera (eye) is centred with the centre of the image, so we need
        // to shift the pixels up and left by half the width/height) to get the pixel
        // coordinates in respect to the camera.
        let mut x_v = ((x as f32) + 0.5) - ((self.width as f32) / 2.0);
        let mut y_v = ((self.height as f32) / 2.0) - ((y as f32) + 0.5);

        // Normalise.
        x_v /= self.width as f32;
        y_v /= self.height as f32;

        Ray::new(
            self.position,
            (x_v * self.u + y_v * self.v - self.fov * self.w).normalise(),
        )
    }
}

impl Default for FullCamera {
    fn default() -> Self {
        Self::new(
            0.5,
            Vertex::default(),
            Vector::new(0.0, 0.0, 1.0),
            Vector::new(0.0, 1.0, 0.0),
        )
    }
}

impl<T: Environment + Sync> Camera<T> for FullCamera {
    fn render(&mut self, env: &mut T, fb: &mut FrameBuffer) {
        self.width = fb.width;
        self.height = fb.height;

        let fb = Arc::new(Mutex::new(fb));

        let start_time = Instant::now();
        let pb = ProgressBar::new(self.height as u64);

        (0..self.height).into_par_iter().for_each(|y| {
            for x in 0..self.width {
                let ray = self.get_pixel_ray(x, y);

                let (colour, depth) = env.raytrace(&ray, RAYTRACE_RECURSE);

                let mut fb = fb.lock().unwrap();
                let _ = fb.plot_pixel(x as i32, y as i32, colour);
                let _ = fb.plot_depth(x as i32, y as i32, depth);
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
