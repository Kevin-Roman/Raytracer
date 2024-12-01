use crate::primitives::vector::Vector;

#[derive(Clone, Copy, Debug)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

pub trait Sampler {
    fn sample_unit_square(&mut self) -> Point2D;

    fn sample_hemisphere(&mut self) -> Vector;
}
