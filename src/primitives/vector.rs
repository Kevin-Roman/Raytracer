use std::{
    cmp::PartialEq,
    ops::{Add, Div, Mul, Neg, Sub},
};

/// A 3D vector.
#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// The squared length of the vector.
    pub fn len_sqr(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn length(&self) -> f32 {
        self.len_sqr().sqrt()
    }

    pub fn normalise(&self) -> Self {
        let len = self.length();
        if len > 0.0 {
            Self::new(self.x / len, self.y / len, self.z / len)
        } else {
            Self::default()
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn reflection(&self, normal: &Self) -> Self {
        let d = 2.0 * self.dot(normal);

        Self::new(
            self.x - d * normal.x,
            self.y - d * normal.y,
            self.z - d * normal.z,
        )
    }

    /// The refraction of a vector based on the normal of the surface it is hitting
    /// and the index of refraction of the material.
    /// Using Snell's Law.
    pub fn refraction(&self, normal: &Self, index_of_refraction: f32) -> Self {
        let incident = self;
        let cos_theta_i = normal.dot(incident).abs();

        let cos_theta_t =
            (1.0 - (1.0 / index_of_refraction.powi(2)) * (1.0 - cos_theta_i.powi(2))).sqrt();

        // Total internal reflection occurs when the term that will be square rooted is a negative number.
        if cos_theta_t.is_nan() {
            return incident.reflection(normal);
        }

        (1.0 / index_of_refraction) * *incident
            - (cos_theta_t - (1.0 / index_of_refraction) * cos_theta_i) * *normal
    }

    pub fn negate(&self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }

    /// The cross product (vector that is perpendicular to two given vectors).
    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl Default for Vector {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Mul<Self> for Vector {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Sub<Self> for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Add<Self> for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, vector: Vector) -> Self::Output {
        Vector {
            x: self * vector.x,
            y: self * vector.y,
            z: self * vector.z,
        }
    }
}

impl Div<f32> for Vector {
    type Output = Self;

    fn div(self, scalar: f32) -> Self::Output {
        Self::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}
