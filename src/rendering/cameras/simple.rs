use crate::{primitives::ray::Ray, Camera, FrameBuffer, Raytracer};
use std::io::{self, Write};

/// Simple Camera has a 90 degree field of view along the z axis.
pub struct SimpleCamera {
    pub width: u16,
    pub height: u16,
    /// Field of view. Distance from the camera to the image plane.
    pub fov: f32,
}

impl SimpleCamera {
    pub fn new(fov: f32) -> Self {
        Self {
            width: u16::default(),
            height: u16::default(),
            fov,
        }
    }

    fn get_pixel_ray(&self, x: u16, y: u16) -> Ray {
        let fx = (x as f32 + 0.5) / (self.width as f32);
        let fy = (y as f32 + 0.5) / (self.height as f32);

        let mut ray = Ray::default();

        ray.direction.x = fx - 0.5;
        ray.direction.y = 0.5 - fy;
        ray.direction.z = self.fov;
        ray.direction = ray.direction.normalise();

        ray
    }
}

impl Default for SimpleCamera {
    fn default() -> Self {
        Self::new(0.5)
    }
}

impl<S: Raytracer + Sync> Camera<S> for SimpleCamera {
    fn render(&mut self, scene: &S, fb: &mut FrameBuffer) {
        self.width = fb.width;
        self.height = fb.height;

        for y in 0..self.height {
            for x in 0..self.width {
                let ray = self.get_pixel_ray(x, y);

                // Start recursion at depth 0
                let (colour, depth) = scene.trace(&ray, 0);

                let _ = fb.plot_pixel(x as i32, y as i32, colour);
                let _ = fb.plot_depth(x as i32, y as i32, depth);
            }

            print!("#");
            let _ = io::stdout().flush();
        }
    }
}
