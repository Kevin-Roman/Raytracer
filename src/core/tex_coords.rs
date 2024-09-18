// TexCoords stores the texture coordinates.

pub struct TexCoords {
    pub u: f32,
    pub v: f32,
    pub s: f32,
    pub t: f32,
}

impl TexCoords {
    pub fn new() -> Self {
        Self {
            u: 0.0,
            v: 0.0,
            s: 0.0,
            t: 0.0,
        }
    }
}
