// Environment is the trait for raytracing. We use this in material to do recursion as that allows
// Scene which is derived from this to depend (indirectly) on Material.

use crate::primitives::{colour::Colour, ray::Ray};

pub trait Environment {
    fn shadowtrace(&mut self, ray: &Ray, limit: f32) -> bool;

    fn raytrace(&mut self, ray: &Ray, recurse: u8) -> (Colour, f32);
}
