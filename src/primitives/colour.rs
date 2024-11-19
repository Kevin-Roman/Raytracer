// Colour stores and manipulates an rgba colour.

use std::ops::{Add, AddAssign, Mul, MulAssign};

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
        Self::new(0.0, 0.0, 0.0, 1.0)
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
