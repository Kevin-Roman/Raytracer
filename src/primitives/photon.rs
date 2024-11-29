use kd_tree::{KdPoint, KdTree};

use super::vertex::Vertex;

pub struct Photon {
    position: Vertex,
}

impl KdPoint for Photon {
    type Scalar = f64;
    type Dim = typenum::U2; // 2 dimensional tree.
    fn at(&self, k: usize) -> f64 {
        0.0
    }
}
