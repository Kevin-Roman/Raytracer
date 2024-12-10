// Full camera allows a camera to be placed in space with a lookat and up direction
// as well as the field of view. It loops over the pixels in a framebuffer and computes
// a ray that is then passed to the environment.

use crate::core::{
    camera::Camera, environment::Environment, framebuffer::FrameBuffer, ray::Ray, vector::Vector,
    vertex::Vertex,
};

const RAYTRACE_RECURSE: i32 = 5;

pub struct FullCamera {
    pub width: i32,
    pub height: i32,
    pub fov: f32,

    position: Vertex,
    w: Vector,
    u: Vector,
    v: Vector,
}

impl FullCamera {
    pub fn new(fov: f32, position: Vertex, lookat: Vector, up: Vector) -> Self {
        let w = (position.vector - lookat).normalise();
        let u = w.cross(&up).normalise();
        let v = u.cross(&w);

        Self {
            width: i32::default(),
            height: i32::default(),
            fov,
            position,
            w,
            u,
            v,
        }
    }

    fn pixel_ray(&self, x: i32, y: i32, x_offset: f32, y_offset: f32) -> Ray {
        // Add 0.5 to pixel's x and y coordinates to get middle of the pixel.
        // The camera (eye) is centred with the centre of the image, so we need
        // to shift the pixels up and left by half the width/height) to get the pixel
        // coordinates in respect to the camera.
        let mut x_v = (x_offset + (x as f32) + 0.5) - ((self.width as f32) / 2.0);
        let mut y_v = ((self.height as f32) / 2.0) - (y_offset + (y as f32) + 0.5);

        // Normalise.
        x_v /= self.width as f32;
        y_v /= self.height as f32;

        Ray::new(
            self.position,
            (x_v * self.u + y_v * self.v - self.fov * self.w).normalise(),
        )
    }

    fn print_progress(&self, x: i32, y: i32) {
        let percentage =
            ((y * self.width + x + 1) as f64 / (self.height * self.width) as f64) * 100.0;
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
                let ray = self.pixel_ray(x, y, 0.0, 0.0);

                let (colour, depth) = env.raytrace(&ray, RAYTRACE_RECURSE);

                let _ = fb.plot_pixel(x, y, colour.r, colour.g, colour.b);
                let _ = fb.plot_depth(x, y, depth);

                self.print_progress(x, y);
            }
        }
    }
}
