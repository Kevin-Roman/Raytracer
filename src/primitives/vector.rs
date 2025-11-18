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
    pub fn len_sqr(self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn length(self) -> f32 {
        self.len_sqr().sqrt()
    }

    pub fn normalise(self) -> Self {
        let len = self.length();
        if len > 0.0 {
            Self::new(self.x / len, self.y / len, self.z / len)
        } else {
            Self::default()
        }
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn reflection(self, normal: Self) -> Self {
        let incident = self;
        incident - 2.0 * incident.dot(normal) * normal
    }

    /// The refraction of a vector based on the normal of the surface it is hitting
    /// and the index of refraction of the material.
    /// Using Snell's Law.
    pub fn refraction(self, normal: Self, index_of_refraction: f32) -> Self {
        let incident = self;
        let cos_theta_i = normal.dot(incident).abs();

        let cos_theta_t =
            (1.0 - (1.0 / index_of_refraction.powi(2)) * (1.0 - cos_theta_i.powi(2))).sqrt();

        // Total internal reflection occurs when the term that will be square rooted is a negative number.
        if cos_theta_t.is_nan() {
            return incident.reflection(normal);
        }

        (1.0 / index_of_refraction) * incident
            - (cos_theta_t - (1.0 / index_of_refraction) * cos_theta_i) * normal
    }

    pub fn negate(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }

    /// The cross product (vector that is perpendicular to two given vectors).
    pub fn cross(self, other: Self) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_vector_length() {
        let v = Vector::new(3.0, 4.0, 0.0);
        assert_relative_eq!(v.length(), 5.0, epsilon = 1e-6);
    }

    #[test]
    fn test_vector_len_sqr() {
        let v = Vector::new(3.0, 4.0, 0.0);
        assert_relative_eq!(v.len_sqr(), 25.0, epsilon = 1e-6);
    }

    #[test]
    fn test_vector_normalise() {
        let v = Vector::new(3.0, 4.0, 0.0);
        let normalized = v.normalise();
        assert_relative_eq!(normalized.length(), 1.0, epsilon = 1e-6);
        assert_relative_eq!(normalized.x, 0.6, epsilon = 1e-6);
        assert_relative_eq!(normalized.y, 0.8, epsilon = 1e-6);
    }

    #[test]
    fn test_vector_normalise_zero() {
        let v = Vector::new(0.0, 0.0, 0.0);
        let normalized = v.normalise();
        assert_eq!(normalized.x, 0.0);
        assert_eq!(normalized.y, 0.0);
        assert_eq!(normalized.z, 0.0);
    }

    #[test]
    fn test_vector_dot_product() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(4.0, 5.0, 6.0);
        assert_relative_eq!(v1.dot(v2), 32.0, epsilon = 1e-6);
    }

    #[test]
    fn test_vector_dot_product_perpendicular() {
        let v1 = Vector::new(1.0, 0.0, 0.0);
        let v2 = Vector::new(0.0, 1.0, 0.0);
        assert_relative_eq!(v1.dot(v2), 0.0, epsilon = 1e-6);
    }

    #[test]
    fn test_vector_cross_product() {
        let v1 = Vector::new(1.0, 0.0, 0.0);
        let v2 = Vector::new(0.0, 1.0, 0.0);
        let cross = v1.cross(v2);
        assert_relative_eq!(cross.x, 0.0, epsilon = 1e-6);
        assert_relative_eq!(cross.y, 0.0, epsilon = 1e-6);
        assert_relative_eq!(cross.z, 1.0, epsilon = 1e-6);
    }

    #[test]
    fn test_vector_addition() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(4.0, 5.0, 6.0);
        let result = v1 + v2;
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 7.0);
        assert_eq!(result.z, 9.0);
    }

    #[test]
    fn test_vector_subtraction() {
        let v1 = Vector::new(5.0, 7.0, 9.0);
        let v2 = Vector::new(1.0, 2.0, 3.0);
        let result = v1 - v2;
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 5.0);
        assert_eq!(result.z, 6.0);
    }

    #[test]
    fn test_vector_scalar_multiplication() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let result = 2.0 * v;
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 4.0);
        assert_eq!(result.z, 6.0);
    }

    #[test]
    fn test_vector_scalar_division() {
        let v = Vector::new(2.0, 4.0, 6.0);
        let result = v / 2.0;
        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, 2.0);
        assert_eq!(result.z, 3.0);
    }

    #[test]
    fn test_vector_negation() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let result = -v;
        assert_eq!(result.x, -1.0);
        assert_eq!(result.y, -2.0);
        assert_eq!(result.z, -3.0);
    }

    #[test]
    fn test_vector_negate_method() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let result = v.negate();
        assert_eq!(result.x, -1.0);
        assert_eq!(result.y, -2.0);
        assert_eq!(result.z, -3.0);
    }

    #[test]
    fn test_vector_component_wise_multiplication() {
        let v1 = Vector::new(2.0, 3.0, 4.0);
        let v2 = Vector::new(5.0, 6.0, 7.0);
        let result = v1 * v2;
        assert_eq!(result.x, 10.0);
        assert_eq!(result.y, 18.0);
        assert_eq!(result.z, 28.0);
    }

    #[test]
    fn test_vector_reflection() {
        let incident = Vector::new(1.0, -1.0, 0.0).normalise();
        let normal = Vector::new(0.0, 1.0, 0.0);
        let reflected = incident.reflection(normal);
        assert_relative_eq!(reflected.x, incident.x, epsilon = 1e-6);
        assert_relative_eq!(reflected.y, -incident.y, epsilon = 1e-6);
        assert_relative_eq!(reflected.z, incident.z, epsilon = 1e-6);
    }

    #[test]
    fn test_vector_refraction_air_to_glass() {
        let incident = Vector::new(0.0, -1.0, 0.0);
        let normal = Vector::new(0.0, 1.0, 0.0);
        let ior = 1.52; // Glass
        let refracted = incident.refraction(normal, ior);
        // At normal incidence, direction should not change much
        assert!(refracted.y < 0.0); // Should still be going down
    }

    #[test]
    fn test_vector_refraction_total_internal_reflection() {
        // High angle of incidence from denser to less dense medium
        let incident = Vector::new(0.8, -0.6, 0.0).normalise();
        let normal = Vector::new(0.0, 1.0, 0.0);
        let ior = 0.67; // From glass to air
        let result = incident.refraction(normal, ior);
        // Should reflect instead of refract
        assert!(result.y > 0.0); // Should be going up (reflected)
    }

    #[test]
    fn test_vector_equality() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(1.0, 2.0, 3.0);
        let v3 = Vector::new(1.0, 2.0, 4.0);
        assert_eq!(v1, v2);
        assert_ne!(v1, v3);
    }

    #[test]
    fn test_vector_large_values() {
        let v = Vector::new(1e10, 1e10, 1e10);
        assert!(v.length().is_finite());
    }

    #[test]
    fn test_vector_small_values() {
        let v = Vector::new(1e-10, 1e-10, 1e-10);
        assert!(v.length() >= 0.0);
    }

    #[test]
    fn test_vector_mixed_signs() {
        let v = Vector::new(-1.0, 2.0, -3.0);
        assert_relative_eq!(v.len_sqr(), 14.0, epsilon = 1e-6);
    }

    #[test]
    fn test_vector_cross_parallel_vectors() {
        let v1 = Vector::new(1.0, 0.0, 0.0);
        let v2 = Vector::new(2.0, 0.0, 0.0);
        let cross = v1.cross(v2);
        assert_relative_eq!(cross.len_sqr(), 0.0, epsilon = 1e-6);
    }

    #[test]
    fn test_vector_refraction_perpendicular() {
        let incident = Vector::new(0.0, -1.0, 0.0);
        let normal = Vector::new(0.0, 1.0, 0.0);
        let ior = 1.5;
        let refracted = incident.refraction(normal, ior);
        // Should refract straight through at normal incidence
        assert!(refracted.y < 0.0);
    }

    #[test]
    fn test_vector_division_by_small_scalar() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let result = v / 0.1;
        assert_eq!(result.x, 10.0);
        assert_eq!(result.y, 20.0);
        assert_eq!(result.z, 30.0);
    }

    #[test]
    fn test_vector_normalize_already_normalized() {
        let v = Vector::new(1.0, 0.0, 0.0);
        let normalized = v.normalise();
        assert_relative_eq!(normalized.x, 1.0, epsilon = 1e-6);
        assert_relative_eq!(normalized.length(), 1.0, epsilon = 1e-6);
    }
}
