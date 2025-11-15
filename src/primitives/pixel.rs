use super::Colour;

#[derive(Clone, Copy, Debug)]
pub struct Pixel {
    pub colour: Colour,
    pub depth: f32,
}

impl Pixel {
    pub fn new(colour: Colour, depth: f32) -> Self {
        Self { colour, depth }
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self::new(Colour::default(), 0.0)
    }
}
