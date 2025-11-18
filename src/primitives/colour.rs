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

    pub fn scale(&mut self, scaling: Self) {
        self.r *= scaling.r;
        self.g *= scaling.g;
        self.b *= scaling.b;
        self.a *= scaling.a;
    }

    pub fn add(&mut self, adjust: Self) {
        self.r += adjust.r;
        self.g += adjust.g;
        self.b += adjust.b;
        self.a += adjust.a;
    }

    /// Calculate the average of RGB components
    pub fn average(&self) -> f32 {
        (self.r + self.g + self.b) / 3.0
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
        self.add(other);
    }
}

impl MulAssign<Self> for Colour {
    fn mul_assign(&mut self, other: Self) {
        self.scale(other);
    }
}

impl DivAssign<f32> for Colour {
    fn div_assign(&mut self, denom: f32) {
        self.r /= denom;
        self.g /= denom;
        self.b /= denom;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_colour_scale() {
        let mut c = Colour::new(0.5, 0.6, 0.7, 1.0);
        c.scale(Colour::new(2.0, 2.0, 2.0, 0.5));
        assert_eq!(c.r, 1.0);
        assert_eq!(c.g, 1.2);
        assert_eq!(c.b, 1.4);
        assert_eq!(c.a, 0.5);
    }

    #[test]
    fn test_colour_add_method() {
        let mut c = Colour::new(0.1, 0.2, 0.3, 0.5);
        c += Colour::new(0.4, 0.3, 0.2, 0.1);
        assert_relative_eq!(c.r, 0.5, epsilon = 1e-6);
        assert_relative_eq!(c.g, 0.5, epsilon = 1e-6);
        assert_relative_eq!(c.b, 0.5, epsilon = 1e-6);
        assert_relative_eq!(c.a, 0.6, epsilon = 1e-6);
    }

    #[test]
    fn test_colour_average() {
        let c = Colour::new(0.3, 0.6, 0.9, 1.0);
        assert_relative_eq!(c.average(), 0.6, epsilon = 1e-6);
    }

    #[test]
    fn test_colour_multiplication() {
        let c1 = Colour::new(0.5, 0.6, 0.7, 1.0);
        let c2 = Colour::new(2.0, 2.0, 2.0, 0.5);
        let result = c1 * c2;
        assert_eq!(result.r, 1.0);
        assert_eq!(result.g, 1.2);
        assert_eq!(result.b, 1.4);
        assert_eq!(result.a, 0.5);
    }

    #[test]
    fn test_colour_addition() {
        let c1 = Colour::new(0.1, 0.2, 0.3, 0.5);
        let c2 = Colour::new(0.4, 0.3, 0.2, 0.1);
        let result = c1 + c2;
        assert_relative_eq!(result.r, 0.5, epsilon = 1e-6);
        assert_relative_eq!(result.g, 0.5, epsilon = 1e-6);
        assert_relative_eq!(result.b, 0.5, epsilon = 1e-6);
        assert_relative_eq!(result.a, 0.6, epsilon = 1e-6);
    }

    #[test]
    fn test_colour_scalar_multiplication() {
        let c = Colour::new(0.5, 0.6, 0.7, 1.0);
        let result = 2.0 * c;
        assert_eq!(result.r, 1.0);
        assert_eq!(result.g, 1.2);
        assert_eq!(result.b, 1.4);
        assert_eq!(result.a, 1.0);
    }

    #[test]
    fn test_colour_scalar_division() {
        let c = Colour::new(1.0, 1.2, 1.4, 1.0);
        let result = c / 2.0;
        assert_eq!(result.r, 0.5);
        assert_eq!(result.g, 0.6);
        assert_eq!(result.b, 0.7);
        assert_eq!(result.a, 1.0);
    }

    #[test]
    fn test_colour_add_assign() {
        let mut c = Colour::new(0.1, 0.2, 0.3, 0.5);
        c += Colour::new(0.4, 0.3, 0.2, 0.1);
        assert_relative_eq!(c.r, 0.5, epsilon = 1e-6);
        assert_relative_eq!(c.g, 0.5, epsilon = 1e-6);
        assert_relative_eq!(c.b, 0.5, epsilon = 1e-6);
        assert_relative_eq!(c.a, 0.6, epsilon = 1e-6);
    }

    #[test]
    fn test_colour_mul_assign() {
        let mut c = Colour::new(0.5, 0.6, 0.7, 1.0);
        c *= Colour::new(2.0, 2.0, 2.0, 0.5);
        assert_eq!(c.r, 1.0);
        assert_eq!(c.g, 1.2);
        assert_eq!(c.b, 1.4);
        assert_eq!(c.a, 0.5);
    }

    #[test]
    fn test_colour_div_assign() {
        let mut c = Colour::new(1.0, 1.2, 1.4, 1.0);
        c /= 2.0;
        assert_eq!(c.r, 0.5);
        assert_eq!(c.g, 0.6);
        assert_eq!(c.b, 0.7);
        assert_eq!(c.a, 1.0);
    }

    #[test]
    fn test_colour_negative_multiplication() {
        let c = Colour::new(1.0, 1.0, 1.0, 1.0);
        let result = -1.0 * c;
        assert_eq!(result.r, -1.0);
    }
}
