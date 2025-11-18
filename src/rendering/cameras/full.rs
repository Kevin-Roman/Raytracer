use indicatif::ProgressBar;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

use crate::{
    primitives::{ray::Ray, Vector, Vertex},
    Camera, FrameBuffer, Raytracer,
};

/// Full camera allows a camera to be placed in space with a look-at and up direction
/// as well as the field of view. It loops over the pixels in a framebuffer and computes
/// a ray that is then passed to the raytracer.
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
        let u = w.cross(up).normalise();
        let v = u.cross(w);

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

impl<S: Raytracer + Sync> Camera<S> for FullCamera {
    fn render(&mut self, scene: &S, fb: &mut FrameBuffer) {
        self.width = fb.width;
        self.height = fb.height;

        let fb = Arc::new(Mutex::new(fb));

        let start_time = Instant::now();
        let pb = ProgressBar::new(self.height as u64);

        (0..self.height).into_par_iter().for_each(|y| {
            for x in 0..self.width {
                let ray = self.get_pixel_ray(x, y);

                let (colour, depth) = scene.trace(&ray, 0);

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

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_full_camera_basis_vectors() {
        let camera = FullCamera::new(
            0.5,
            Vertex::new(0.0, 0.0, -10.0, 1.0),
            Vector::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        );

        // w should point backward (from lookat to position)
        assert_relative_eq!(camera.w.z, -1.0, epsilon = 1e-5);
        // u should be right vector (perpendicular to w and up)
        assert_relative_eq!(camera.u.length(), 1.0, epsilon = 1e-5);
        // v should be up vector
        assert_relative_eq!(camera.v.length(), 1.0, epsilon = 1e-5);
    }

    #[test]
    fn test_full_camera_ray_direction() {
        let mut camera = FullCamera::new(
            0.5,
            Vertex::new(0.0, 0.0, 0.0, 1.0),
            Vector::new(0.0, 0.0, 1.0),
            Vector::new(0.0, 1.0, 0.0),
        );

        camera.width = 100;
        camera.height = 100;

        let ray = camera.get_pixel_ray(50, 50);

        // Center ray should point forward
        assert_relative_eq!(ray.direction.length(), 1.0, epsilon = 1e-5);
        assert_eq!(ray.position.vector, camera.position.vector);
    }

    #[test]
    fn test_full_camera_corner_rays() {
        let mut camera = FullCamera::new(
            1.0,
            Vertex::new(0.0, 0.0, 0.0, 1.0),
            Vector::new(0.0, 0.0, 1.0),
            Vector::new(0.0, 1.0, 0.0),
        );

        camera.width = 100;
        camera.height = 100;

        // Top-left corner should have negative u, positive v
        let ray_tl = camera.get_pixel_ray(0, 0);
        assert!(ray_tl.direction.x < 0.0);
        assert!(ray_tl.direction.y > 0.0);

        // Bottom-right corner should have positive u, negative v
        let ray_br = camera.get_pixel_ray(99, 99);
        assert!(ray_br.direction.x > 0.0);
        assert!(ray_br.direction.y < 0.0);
    }
}
