use std::ops::Mul;

use super::{Vector, Vertex};

/// A 4x4 transformation matrix.
#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub matrix: [[f32; 4]; 4],
}

impl Transform {
    pub fn new(matrix: [[f32; 4]; 4]) -> Self {
        Self { matrix }
    }

    pub fn identity() -> Self {
        Self::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn apply_to_vertex(&self, input: &mut Vertex) {
        let x = self.matrix[0][0] * input.vector.x
            + self.matrix[0][1] * input.vector.y
            + self.matrix[0][2] * input.vector.z
            + self.matrix[0][3] * input.w;

        let y = self.matrix[1][0] * input.vector.x
            + self.matrix[1][1] * input.vector.y
            + self.matrix[1][2] * input.vector.z
            + self.matrix[1][3] * input.w;

        let z = self.matrix[2][0] * input.vector.x
            + self.matrix[2][1] * input.vector.y
            + self.matrix[2][2] * input.vector.z
            + self.matrix[2][3] * input.w;

        let w = self.matrix[3][0] * input.vector.x
            + self.matrix[3][1] * input.vector.y
            + self.matrix[3][2] * input.vector.z
            + self.matrix[3][3] * input.w;

        input.vector.x = x;
        input.vector.y = y;
        input.vector.z = z;
        input.w = w;
    }

    pub fn apply_to_vector(&self, vector: &mut Vector) {
        let x = self.matrix[0][0] * vector.x
            + self.matrix[0][1] * vector.y
            + self.matrix[0][2] * vector.z;
        let y = self.matrix[1][0] * vector.x
            + self.matrix[1][1] * vector.y
            + self.matrix[1][2] * vector.z;
        let z = self.matrix[2][0] * vector.x
            + self.matrix[2][1] * vector.y
            + self.matrix[2][2] * vector.z;

        vector.x = x;
        vector.y = y;
        vector.z = z;
    }

    /// MESA (https://www.mesa3d.org/) implementation of the inverse of a 4x4 matrix.
    /// Based on Minors, Cofactors and the Adjugate Matrix.
    pub fn inverse(&self) -> Transform {
        let mut r = Transform::identity();

        r.matrix[0][0] = self.matrix[1][1] * self.matrix[2][2] * self.matrix[3][3]
            - self.matrix[1][1] * self.matrix[2][3] * self.matrix[3][2]
            - self.matrix[2][1] * self.matrix[1][2] * self.matrix[3][3]
            + self.matrix[2][1] * self.matrix[1][3] * self.matrix[3][2]
            + self.matrix[3][1] * self.matrix[1][2] * self.matrix[2][3]
            - self.matrix[3][1] * self.matrix[1][3] * self.matrix[2][2];

        r.matrix[1][0] = -self.matrix[1][0] * self.matrix[2][2] * self.matrix[3][3]
            + self.matrix[1][0] * self.matrix[2][3] * self.matrix[3][2]
            + self.matrix[2][0] * self.matrix[1][2] * self.matrix[3][3]
            - self.matrix[2][0] * self.matrix[1][3] * self.matrix[3][2]
            - self.matrix[3][0] * self.matrix[1][2] * self.matrix[2][3]
            + self.matrix[3][0] * self.matrix[1][3] * self.matrix[2][2];

        r.matrix[2][0] = self.matrix[1][0] * self.matrix[2][1] * self.matrix[3][3]
            - self.matrix[1][0] * self.matrix[2][3] * self.matrix[3][1]
            - self.matrix[2][0] * self.matrix[1][1] * self.matrix[3][3]
            + self.matrix[2][0] * self.matrix[1][3] * self.matrix[3][1]
            + self.matrix[3][0] * self.matrix[1][1] * self.matrix[2][3]
            - self.matrix[3][0] * self.matrix[1][3] * self.matrix[2][1];

        r.matrix[3][0] = -self.matrix[1][0] * self.matrix[2][1] * self.matrix[3][2]
            + self.matrix[1][0] * self.matrix[2][2] * self.matrix[3][1]
            + self.matrix[2][0] * self.matrix[1][1] * self.matrix[3][2]
            - self.matrix[2][0] * self.matrix[1][2] * self.matrix[3][1]
            - self.matrix[3][0] * self.matrix[1][1] * self.matrix[2][2]
            + self.matrix[3][0] * self.matrix[1][2] * self.matrix[2][1];

        r.matrix[0][1] = -self.matrix[0][1] * self.matrix[2][2] * self.matrix[3][3]
            + self.matrix[0][1] * self.matrix[2][3] * self.matrix[3][2]
            + self.matrix[2][1] * self.matrix[0][2] * self.matrix[3][3]
            - self.matrix[2][1] * self.matrix[0][3] * self.matrix[3][2]
            - self.matrix[3][1] * self.matrix[0][2] * self.matrix[2][3]
            + self.matrix[3][1] * self.matrix[0][3] * self.matrix[2][2];

        r.matrix[1][1] = self.matrix[0][0] * self.matrix[2][2] * self.matrix[3][3]
            - self.matrix[0][0] * self.matrix[2][3] * self.matrix[3][2]
            - self.matrix[2][0] * self.matrix[0][2] * self.matrix[3][3]
            + self.matrix[2][0] * self.matrix[0][3] * self.matrix[3][2]
            + self.matrix[3][0] * self.matrix[0][2] * self.matrix[2][3]
            - self.matrix[3][0] * self.matrix[0][3] * self.matrix[2][2];

        r.matrix[2][1] = -self.matrix[0][0] * self.matrix[2][1] * self.matrix[3][3]
            + self.matrix[0][0] * self.matrix[2][3] * self.matrix[3][1]
            + self.matrix[2][0] * self.matrix[0][1] * self.matrix[3][3]
            - self.matrix[2][0] * self.matrix[0][3] * self.matrix[3][1]
            - self.matrix[3][0] * self.matrix[0][1] * self.matrix[2][3]
            + self.matrix[3][0] * self.matrix[0][3] * self.matrix[2][1];

        r.matrix[3][1] = self.matrix[0][0] * self.matrix[2][1] * self.matrix[3][2]
            - self.matrix[0][0] * self.matrix[2][2] * self.matrix[3][1]
            - self.matrix[2][0] * self.matrix[0][1] * self.matrix[3][2]
            + self.matrix[2][0] * self.matrix[0][2] * self.matrix[3][1]
            + self.matrix[3][0] * self.matrix[0][1] * self.matrix[2][2]
            - self.matrix[3][0] * self.matrix[0][2] * self.matrix[2][1];

        r.matrix[0][2] = self.matrix[0][1] * self.matrix[1][2] * self.matrix[3][3]
            - self.matrix[0][1] * self.matrix[1][3] * self.matrix[3][2]
            - self.matrix[1][1] * self.matrix[0][2] * self.matrix[3][3]
            + self.matrix[1][1] * self.matrix[0][3] * self.matrix[3][2]
            + self.matrix[3][1] * self.matrix[0][2] * self.matrix[1][3]
            - self.matrix[3][1] * self.matrix[0][3] * self.matrix[1][2];

        r.matrix[1][2] = -self.matrix[0][0] * self.matrix[1][2] * self.matrix[3][3]
            + self.matrix[0][0] * self.matrix[1][3] * self.matrix[3][2]
            + self.matrix[1][0] * self.matrix[0][2] * self.matrix[3][3]
            - self.matrix[1][0] * self.matrix[0][3] * self.matrix[3][2]
            - self.matrix[3][0] * self.matrix[0][2] * self.matrix[1][3]
            + self.matrix[3][0] * self.matrix[0][3] * self.matrix[1][2];

        r.matrix[2][2] = self.matrix[0][0] * self.matrix[1][1] * self.matrix[3][3]
            - self.matrix[0][0] * self.matrix[1][3] * self.matrix[3][1]
            - self.matrix[1][0] * self.matrix[0][1] * self.matrix[3][3]
            + self.matrix[1][0] * self.matrix[0][3] * self.matrix[3][1]
            + self.matrix[3][0] * self.matrix[0][1] * self.matrix[1][3]
            - self.matrix[3][0] * self.matrix[0][3] * self.matrix[1][1];

        r.matrix[3][2] = -self.matrix[0][0] * self.matrix[1][1] * self.matrix[3][2]
            + self.matrix[0][0] * self.matrix[1][2] * self.matrix[3][1]
            + self.matrix[1][0] * self.matrix[0][1] * self.matrix[3][2]
            - self.matrix[1][0] * self.matrix[0][2] * self.matrix[3][1]
            - self.matrix[3][0] * self.matrix[0][1] * self.matrix[1][2]
            + self.matrix[3][0] * self.matrix[0][2] * self.matrix[1][1];

        r.matrix[0][3] = -self.matrix[0][1] * self.matrix[1][2] * self.matrix[2][3]
            + self.matrix[0][1] * self.matrix[1][3] * self.matrix[2][2]
            + self.matrix[1][1] * self.matrix[0][2] * self.matrix[2][3]
            - self.matrix[1][1] * self.matrix[0][3] * self.matrix[2][2]
            - self.matrix[2][1] * self.matrix[0][2] * self.matrix[1][3]
            + self.matrix[2][1] * self.matrix[0][3] * self.matrix[1][2];

        r.matrix[1][3] = self.matrix[0][0] * self.matrix[1][2] * self.matrix[2][3]
            - self.matrix[0][0] * self.matrix[1][3] * self.matrix[2][2]
            - self.matrix[1][0] * self.matrix[0][2] * self.matrix[2][3]
            + self.matrix[1][0] * self.matrix[0][3] * self.matrix[2][2]
            + self.matrix[2][0] * self.matrix[0][2] * self.matrix[1][3]
            - self.matrix[2][0] * self.matrix[0][3] * self.matrix[1][2];

        r.matrix[2][3] = -self.matrix[0][0] * self.matrix[1][1] * self.matrix[2][3]
            + self.matrix[0][0] * self.matrix[1][3] * self.matrix[2][1]
            + self.matrix[1][0] * self.matrix[0][1] * self.matrix[2][3]
            - self.matrix[1][0] * self.matrix[0][3] * self.matrix[2][1]
            - self.matrix[2][0] * self.matrix[0][1] * self.matrix[1][3]
            + self.matrix[2][0] * self.matrix[0][3] * self.matrix[1][1];

        r.matrix[3][3] = self.matrix[0][0] * self.matrix[1][1] * self.matrix[2][2]
            - self.matrix[0][0] * self.matrix[1][2] * self.matrix[2][1]
            - self.matrix[1][0] * self.matrix[0][1] * self.matrix[2][2]
            + self.matrix[1][0] * self.matrix[0][2] * self.matrix[2][1]
            + self.matrix[2][0] * self.matrix[0][1] * self.matrix[1][2]
            - self.matrix[2][0] * self.matrix[0][2] * self.matrix[1][1];

        let det = self.matrix[0][0] * r.matrix[0][0]
            + self.matrix[0][1] * r.matrix[1][0]
            + self.matrix[0][2] * r.matrix[2][0]
            + self.matrix[0][3] * r.matrix[3][0];

        if det != 0.0 {
            let inverse_det = 1.0 / det;
            for i in 0..4 {
                for j in 0..4 {
                    r.matrix[i][j] *= inverse_det;
                }
            }
        }

        r
    }

    pub fn transpose(&self) -> Transform {
        let mut result = Transform::identity();

        for x in 0..4 {
            for y in 0..4 {
                result.matrix[x][y] = self.matrix[y][x];
            }
        }

        result
    }
}

impl Mul<Self> for Transform {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Transform::identity();

        for x in 0..4 {
            for y in 0..4 {
                result.matrix[x][y] = (self.matrix[x][0] * rhs.matrix[0][y])
                    + (self.matrix[x][1] * rhs.matrix[1][y])
                    + (self.matrix[x][2] * rhs.matrix[2][y])
                    + (self.matrix[x][3] * rhs.matrix[3][y]);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_transform_identity() {
        let t = Transform::identity();
        for i in 0..4 {
            for j in 0..4 {
                if i == j {
                    assert_eq!(t.matrix[i][j], 1.0);
                } else {
                    assert_eq!(t.matrix[i][j], 0.0);
                }
            }
        }
    }

    #[test]
    fn test_transform_apply_to_vertex_identity() {
        let t = Transform::identity();
        let mut v = Vertex::new(1.0, 2.0, 3.0, 1.0);
        t.apply_to_vertex(&mut v);
        assert_eq!(v.vector.x, 1.0);
        assert_eq!(v.vector.y, 2.0);
        assert_eq!(v.vector.z, 3.0);
        assert_eq!(v.w, 1.0);
    }

    #[test]
    fn test_transform_apply_to_vector_identity() {
        let t = Transform::identity();
        let mut v = Vector::new(1.0, 2.0, 3.0);
        t.apply_to_vector(&mut v);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_transform_translation() {
        let t = Transform::new([
            [1.0, 0.0, 0.0, 5.0],
            [0.0, 1.0, 0.0, 3.0],
            [0.0, 0.0, 1.0, 2.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let mut v = Vertex::new(1.0, 2.0, 3.0, 1.0);
        t.apply_to_vertex(&mut v);
        assert_eq!(v.vector.x, 6.0);
        assert_eq!(v.vector.y, 5.0);
        assert_eq!(v.vector.z, 5.0);
    }

    #[test]
    fn test_transform_scaling() {
        let t = Transform::new([
            [2.0, 0.0, 0.0, 0.0],
            [0.0, 3.0, 0.0, 0.0],
            [0.0, 0.0, 4.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let mut v = Vertex::new(1.0, 2.0, 3.0, 1.0);
        t.apply_to_vertex(&mut v);
        assert_eq!(v.vector.x, 2.0);
        assert_eq!(v.vector.y, 6.0);
        assert_eq!(v.vector.z, 12.0);
    }

    #[test]
    fn test_transform_multiplication() {
        let t1 = Transform::new([
            [2.0, 0.0, 0.0, 0.0],
            [0.0, 2.0, 0.0, 0.0],
            [0.0, 0.0, 2.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let t2 = Transform::new([
            [1.0, 0.0, 0.0, 5.0],
            [0.0, 1.0, 0.0, 3.0],
            [0.0, 0.0, 1.0, 2.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let combined = t1 * t2;

        let mut v = Vertex::new(1.0, 2.0, 3.0, 1.0);
        combined.apply_to_vertex(&mut v);
        assert_eq!(v.vector.x, 12.0); // (1 + 5) * 2
        assert_eq!(v.vector.y, 10.0); // (2 + 3) * 2
        assert_eq!(v.vector.z, 10.0); // (3 + 2) * 2
    }

    #[test]
    fn test_transform_inverse_identity() {
        let t = Transform::identity();
        let inv = t.inverse();

        for i in 0..4 {
            for j in 0..4 {
                assert_relative_eq!(inv.matrix[i][j], t.matrix[i][j], epsilon = 1e-6);
            }
        }
    }

    #[test]
    fn test_transform_inverse_scaling() {
        let t = Transform::new([
            [2.0, 0.0, 0.0, 0.0],
            [0.0, 3.0, 0.0, 0.0],
            [0.0, 0.0, 4.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let inv = t.inverse();

        assert_relative_eq!(inv.matrix[0][0], 0.5, epsilon = 1e-6);
        assert_relative_eq!(inv.matrix[1][1], 1.0 / 3.0, epsilon = 1e-6);
        assert_relative_eq!(inv.matrix[2][2], 0.25, epsilon = 1e-6);
    }

    #[test]
    fn test_transform_inverse_composition() {
        let t = Transform::new([
            [2.0, 0.0, 0.0, 5.0],
            [0.0, 2.0, 0.0, 3.0],
            [0.0, 0.0, 2.0, 2.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let inv = t.inverse();
        let identity = t * inv;

        // Check if result is close to identity
        for i in 0..4 {
            for j in 0..4 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert_relative_eq!(identity.matrix[i][j], expected, epsilon = 1e-5);
            }
        }
    }

    #[test]
    fn test_transform_transpose() {
        let t = Transform::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        let transposed = t.transpose();

        for i in 0..4 {
            for j in 0..4 {
                assert_eq!(transposed.matrix[i][j], t.matrix[j][i]);
            }
        }
    }

    #[test]
    fn test_transform_transpose_involution() {
        let t = Transform::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        let double_transposed = t.transpose().transpose();

        for i in 0..4 {
            for j in 0..4 {
                assert_eq!(double_transposed.matrix[i][j], t.matrix[i][j]);
            }
        }
    }

    #[test]
    fn test_transform_zero_determinant() {
        // Singular matrix (determinant = 0)
        let t = Transform::new([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 6.0, 8.0],
            [3.0, 6.0, 9.0, 12.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let inv = t.inverse();
        // Inverse should still be computable (but may not be accurate)
        assert!(inv.matrix[0][0].is_finite());
    }

    #[test]
    fn test_transform_large_scale() {
        let t = Transform::new([
            [1000.0, 0.0, 0.0, 0.0],
            [0.0, 1000.0, 0.0, 0.0],
            [0.0, 0.0, 1000.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let mut v = Vertex::new(1.0, 1.0, 1.0, 1.0);
        t.apply_to_vertex(&mut v);
        assert_eq!(v.vector.x, 1000.0);
    }

    #[test]
    fn test_transform_rotation_preserves_length() {
        // 90 degree rotation around z-axis
        let t = Transform::new([
            [0.0, -1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let mut v = Vector::new(1.0, 0.0, 0.0);
        let original_length = v.length();
        t.apply_to_vector(&mut v);
        let new_length = v.length();

        assert_relative_eq!(original_length, new_length, epsilon = 1e-5);
    }
}
