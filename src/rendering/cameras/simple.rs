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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixel_ray_center_pixel() {
        let mut camera = SimpleCamera::new(0.5);
        camera.width = 100;
        camera.height = 100;

        // Center pixel should have ray direction close to (0, 0, fov)
        let ray = camera.get_pixel_ray(50, 50);
        assert!(ray.direction.x.abs() < 0.1);
        assert!(ray.direction.y.abs() < 0.1);
        assert!(ray.direction.z > 0.0);
    }

    #[test]
    fn test_pixel_ray_normalized() {
        let mut camera = SimpleCamera::new(0.5);
        camera.width = 100;
        camera.height = 100;

        let ray = camera.get_pixel_ray(25, 25);
        let length = (ray.direction.x * ray.direction.x
            + ray.direction.y * ray.direction.y
            + ray.direction.z * ray.direction.z)
            .sqrt();

        assert!((length - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_pixel_ray_corners() {
        let mut camera = SimpleCamera::new(0.5);
        camera.width = 100;
        camera.height = 100;

        let top_left = camera.get_pixel_ray(0, 0);
        assert!(top_left.direction.x < 0.0);
        assert!(top_left.direction.y > 0.0);

        let bottom_right = camera.get_pixel_ray(99, 99);
        assert!(bottom_right.direction.x > 0.0);
        assert!(bottom_right.direction.y < 0.0);
    }
}
