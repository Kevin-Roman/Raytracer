use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign};

/// RGBA colour with components in the range [0.0, 1.0].
#[derive(Clone, Copy, Debug)]
pub struct Colour {
    /// Red.
    pub r: f32,
    /// Green.
    pub g: f32,
    /// Blue.
    pub b: f32,
    /// Alpha.
    pub a: f32,
}

impl Colour {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn scale(&mut self, scaling: &Self) {
        self.r *= scaling.r;
        self.g *= scaling.g;
        self.b *= scaling.b;
        self.a *= scaling.a;
    }

    pub fn add(&mut self, adjust: &Self) {
        self.r += adjust.r;
        self.g += adjust.g;
        self.b += adjust.b;
        self.a += adjust.a;
    }
}

impl Default for Colour {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

impl Mul<Self> for Colour {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::new(
            self.r * other.r,
            self.g * other.g,
            self.b * other.b,
            self.a * other.a,
        )
    }
}

impl Add<Self> for Colour {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(
            self.r + other.r,
            self.g + other.g,
            self.b + other.b,
            self.a + other.a,
        )
    }
}

impl Mul<Colour> for f32 {
    type Output = Colour;

    fn mul(self, colour: Colour) -> Self::Output {
        Colour::new(self * colour.r, self * colour.g, self * colour.b, colour.a)
    }
}

impl Div<f32> for Colour {
    type Output = Self;

    fn div(self, denom: f32) -> Self::Output {
        Colour::new(self.r / denom, self.g / denom, self.b / denom, self.a)
    }
}

impl AddAssign<Self> for Colour {
    fn add_assign(&mut self, other: Self) {
        self.add(&other);
    }
}

impl MulAssign<Self> for Colour {
    fn mul_assign(&mut self, other: Self) {
        self.scale(&other);
    }
}

impl DivAssign<f32> for Colour {
    fn div_assign(&mut self, denom: f32) {
        self.r /= denom;
        self.g /= denom;
        self.b /= denom;
    }
}
