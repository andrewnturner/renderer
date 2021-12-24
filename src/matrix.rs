use std::ops::Mul;

use num_traits::Num;

use crate::point::Point4;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Matrix44<T: Num> {
    pub m: [[T; 4]; 4],
}

impl<T: Num> Matrix44<T> {
    pub fn new(
        m00: T, m01: T, m02: T, m03: T,
        m10: T, m11: T, m12: T, m13: T,
        m20: T, m21: T, m22: T, m23: T,
        m30: T, m31: T, m32: T, m33: T,
    ) -> Self {
        let elements = [
            [m00, m01, m02, m03],
            [m10, m11, m12, m13],
            [m20, m21, m22, m23],
            [m30, m31, m32, m33],
        ];

        Self { m: elements }
    }

    pub fn identity() -> Self {
        let elements = [
            [T::one(), T::zero(), T::zero(), T::zero()],
            [T::zero(), T::one(), T::zero(), T::zero()],
            [T::zero(), T::zero(), T::one(), T::zero()],
            [T::zero(), T::zero(), T::zero(), T::one()],
        ];

        Self { m: elements }
    }
}

impl<T: Num + Copy> Mul for Matrix44<T> {
    type Output = Matrix44<T>;

    fn mul(self, other: Matrix44<T>) -> Matrix44<T> {
        let mut elements = [[T::zero(); 4]; 4];

        for i in 0..4 {
            for j in 0..4 {
                elements[i][j] = (self.m[i][0] * other.m[0][j]) +
                                 (self.m[i][1] * other.m[1][j]) + 
                                 (self.m[i][2] * other.m[2][j]) + 
                                 (self.m[i][3] * other.m[3][j]);
            }
        }

        Matrix44 { m: elements }
    }
}

impl<T: Num + Copy> Mul<Point4<T>> for Matrix44<T> {
    type Output = Point4<T>;

    fn mul(self, other: Point4<T>) -> Point4<T> {
        let answer_x = (other.x * self.m[0][0]) + (other.y * self.m[0][1]) + (other.z * self.m[0][2]) + (other.w * self.m[0][3]);
        let answer_y = (other.x * self.m[1][0]) + (other.y * self.m[1][1]) + (other.z * self.m[1][2]) + (other.w * self.m[1][3]);
        let answer_z = (other.x * self.m[2][0]) + (other.y * self.m[2][1]) + (other.z * self.m[2][2]) + (other.w * self.m[2][3]);
        let answer_w = (other.x * self.m[3][0]) + (other.y * self.m[3][1]) + (other.z * self.m[3][2]) + (other.w * self.m[3][3]);
        
        Point4 { x: answer_x, y: answer_y, z: answer_z, w: answer_w }
    }
}
