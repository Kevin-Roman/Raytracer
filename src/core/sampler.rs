use crate::primitives::vector::Vector;

pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

pub trait Sampler {
    fn get_samples(&self) -> &Vec<Point2D>;

    fn hemisphere_sampler(&self, e: f32) -> Vec<Vector>;
}
