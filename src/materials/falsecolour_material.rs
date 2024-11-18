// This Material class maps the x,y,z components of the normal to the r,g,b components
// of the returned colour. A useful debug tool.

use crate::{
    core::{environment::Environment, material::Material},
    primitives::{colour::Colour, hit::Hit, ray::Ray, vector::Vector},
};

pub struct FalseColourMaterial {}

impl FalseColourMaterial {
    pub fn new() -> Self {
        Self {}
    }
}

impl Material for FalseColourMaterial {
    fn compute_once(
        &self,
        _environment: &mut dyn Environment,
        _viewer: &Ray,
        hit: &Hit,
        _recurse: u8,
    ) -> Colour {
        let mut result = Colour::default();
        result.r = (hit.normal.x + 1.0) * 0.5;
        result.g = (hit.normal.y + 1.0) * 0.5;
        result.b = (hit.normal.z + 1.0) * 0.5;
        return result;
    }

    fn compute_per_light(&self, _viewer: &Vector, _light_direction: &Vector, _hit: &Hit) -> Colour {
        Colour::default()
    }
}
