// simple Camera has a 90 degree field of view along the z axis.

use crate::core::{
    camera::Camera, environment::Environment, framebuffer::FrameBuffer, ray::Ray, vertex::Vertex,
};
use std::io::{self, Write};

pub struct SimpleCamera {
    pub width: i32,
    pub height: i32,
    pub fov: f32,
}

impl SimpleCamera {
    pub fn new(fov: f32) -> Self {
        Self {
            width: i32::default(),
            height: i32::default(),
            fov,
        }
    }

    fn get_ray_pixel(&self, x: i32, y: i32, ray: &mut Ray) {
        let fx = (x as f32 + 0.5) / (self.width as f32);
        let fy = (y as f32 + 0.5) / (self.height as f32);

        ray.position = Vertex::default();

        ray.direction.x = fx - 0.5;
        ray.direction.y = 0.5 - fy;
        ray.direction.z = self.fov;
        ray.direction = ray.direction.normalise();
    }
}

impl Default for SimpleCamera {
    fn default() -> Self {
        Self::new(0.5)
    }
}

impl Camera for SimpleCamera {
    fn render(&mut self, env: &mut dyn Environment, fb: &mut FrameBuffer) {
        self.width = fb.width;
        self.height = fb.height;

        for y in 0..self.height {
            for x in 0..self.width {
                let mut ray = Ray::default();
                self.get_ray_pixel(x, y, &mut ray);

                let (colour, depth) = env.raytrace(&ray, 5);

                let _ = fb.plot_pixel(x, y, colour.r, colour.g, colour.b);
                let _ = fb.plot_depth(x, y, depth);
            }

            print!("#");
            let _ = io::stdout().flush();
        }
    }
}
