// Full camera allows a camera to be placed in space with a lookat and up direction
// as well as the field of view. It loops over the pixels in a framebuffer and computes
// a ray that is then passed to the environment.

use crate::{
    core::{camera::Camera, environment::Environment, framebuffer::FrameBuffer},
    primitives::{ray::Ray, vector::Vector, vertex::Vertex},
};

const RAYTRACE_RECURSE: u8 = 5;

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

    fn print_progress(&self, x: i32, y: i32) {
        let percentage = ((y * (self.width as i32) + x + 1) as f64
            / ((self.height as u32) * (self.width as u32)) as f64)
            * 100.0;
        print!("\rProgress: {:.2}%", percentage);
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

impl Camera for FullCamera {
    fn render(&mut self, env: &mut dyn Environment, fb: &mut FrameBuffer) {
        self.width = fb.width;
        self.height = fb.height;

        for y in 0..self.height {
            for x in 0..self.width {
                let ray = self.get_pixel_ray(x, y);

                let (colour, depth) = env.raytrace(&ray, RAYTRACE_RECURSE);

                let _ = fb.plot_pixel(x as i32, y as i32, colour.r, colour.g, colour.b);
                let _ = fb.plot_depth(x as i32, y as i32, depth);

                self.print_progress(x as i32, y as i32);
            }
        }
    }
}
