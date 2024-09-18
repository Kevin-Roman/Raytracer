// Colour stores and manipulates an rgba colour.

use std::ops::{Add, AddAssign, Mul, MulAssign};

pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
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

impl Mul<&Self> for Colour {
    type Output = Self;

    fn mul(self, other: &Self) -> Self::Output {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
            a: self.a * other.a,
        }
    }
}

impl Add<&Self> for Colour {
    type Output = Self;

    fn add(self, other: &Self) -> Self::Output {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
            a: self.a + other.a,
        }
    }
}

impl Mul<f32> for Colour {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a,
        }
    }
}

impl AddAssign<&Self> for Colour {
    fn add_assign(&mut self, other: &Self) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
        self.a += other.a;
    }
}

impl MulAssign<&Self> for Colour {
    fn mul_assign(&mut self, other: &Self) {
        self.r *= other.r;
        self.g *= other.g;
        self.b *= other.b;
        self.a *= other.a;
    }
}
