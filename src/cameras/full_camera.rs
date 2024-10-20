// Full camera allows a camera to be placed in space with a lookat and up direction
// as well as the field of view. It loops over the pixels in a framebuffer and computes
// a ray that is then passed to the environment.

use crate::core::{
    camera::Camera, environment::Environment, framebuffer::FrameBuffer, ray::Ray, vector::Vector,
    vertex::Vertex,
};

pub struct FullCamera {
    pub width: i32,
    pub height: i32,
    pub fov: f32,

    position: Vertex,
    lookat: Vector,
    up: Vector,
    right: Vector,
}

impl FullCamera {
    pub fn new(fov: f32, position: Vertex, lookat: Vector, up: Vector) -> Self {
        Self {
            width: 0,
            height: 0,
            fov,
            position,
            lookat,
            up,
            right: Vector::default(),
        }
    }

    fn get_ray_offset(x: i32, y: i32, x_offset: f32, y_offset: f32, ray: &Ray) {
        todo!()
    }

    fn get_ray_pixel(x: i32, y: i32, ray: &Ray) {
        todo!()
    }
}

impl Default for FullCamera {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            fov: 0.5,
            position: Vertex::default(),
            lookat: Vector::default(),
            up: Vector::default(),
            right: Vector::default(),
        }
    }
}

impl Camera for FullCamera {
    fn render(&mut self, env: &mut dyn Environment, fb: &mut FrameBuffer) {
        todo!()
    }
}
