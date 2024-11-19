use std::ops::Mul;

use super::{vector::Vector, vertex::Vertex};

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
