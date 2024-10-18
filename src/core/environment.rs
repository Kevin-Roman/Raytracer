// Environment is the trait for raytracing. We use this in material to do recursion as that allows
// Scene which is derived from this to depend (indirectly) on Material.

use super::{colour::Colour, ray::Ray};

pub trait Environment {
    fn shadowtrace(&mut self, ray: &Ray, limit: f32) -> bool;

    fn raytrace(&mut self, ray: &Ray, recurse: i32) -> (Colour, f32);
}

pub struct BaseEnvironment {}

impl BaseEnvironment {
    pub fn new() -> Self {
        Self {}
    }
}

impl Environment for BaseEnvironment {
    fn shadowtrace(&mut self, _ray: &Ray, _limit: f32) -> bool {
        false
    }

    fn raytrace(&mut self, _ray: &Ray, _recurse: i32) -> (Colour, f32) {
        let colour = Colour::default();
        let depth = f32::INFINITY;
        (colour, depth)
    }
}
