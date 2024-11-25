use crate::{
    core::{environment::Environment, material::Material},
    primitives::{colour::Colour, hit::Hit, ray::Ray, vector::Vector},
};

/// Material that maps the x, y, z components of the normal to arbitrary r, g, b components.
/// Used for debugging purposes.
pub struct FalseColourMaterial {}

impl FalseColourMaterial {
    pub fn new() -> Self {
        Self {}
    }
}

impl Material for FalseColourMaterial {
    fn compute_once(
        &self,
        _environment: &dyn Environment,
        _viewer: &Ray,
        hit: &Hit,
        _recurse: u8,
    ) -> Colour {
        Colour::new(
            (hit.normal.x + 1.0) * 0.5,
            (hit.normal.y + 1.0) * 0.5,
            (hit.normal.z + 1.0) * 0.5,
            1.0,
        )
    }

    fn compute_per_light(&self, _viewer: &Vector, _light_direction: &Vector, _hit: &Hit) -> Colour {
        Colour::default()
    }
}
