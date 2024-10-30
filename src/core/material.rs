use super::{colour::Colour, hit::Hit, ray::Ray, vector::Vector};

pub trait Material {
    fn compute_once(&self, viewer: &Ray, hit: &Hit, recurse: i32) -> Colour;

    fn compute_per_light(&self, viewer: &Vector, light_direction: &Vector, hit: &Hit) -> Colour;
}
