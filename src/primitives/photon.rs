use kd_tree::KdPoint;

use super::{Colour, Vector, Vertex};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PhotonOutcome {
    Reflect,
    Absorb,
    Transmit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PhotonType {
    DirectionIllumination,
    IndirectIllumination,
    ShadowPhoton,
}

pub struct Photon {
    pub position: Vertex,
    pub direction: Vector,
    pub intensity: Colour,
    pub photon_type: PhotonType,
}

impl Photon {
    pub fn new(
        position: Vertex,
        direction: Vector,
        intensity: Colour,
        photon_type: PhotonType,
    ) -> Self {
        Self {
            position,
            direction,
            intensity,
            photon_type,
        }
    }
}

impl Default for Photon {
    fn default() -> Self {
        Photon::new(
            Vertex::default(),
            Vector::default(),
            Colour::default(),
            PhotonType::DirectionIllumination,
        )
    }
}

impl KdPoint for Photon {
    type Scalar = f32;
    // 3-dimensional tree.
    type Dim = typenum::U3;

    fn at(&self, index: usize) -> f32 {
        match index {
            0 => self.position.vector.x,
            1 => self.position.vector.y,
            2 => self.position.vector.z,
            _ => unreachable!(),
        }
    }
}
