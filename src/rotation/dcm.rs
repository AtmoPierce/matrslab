use super::euler::Euler;
use super::quaternion::Quaternion;
use crate::Matrix;
use num_traits::Float;
use core::ops::{Mul, Add, Sub, Neg, Div};

#[derive(Debug, Clone, Copy)]
pub struct DirectionCosineMatrix<T: Float> {
    data: Matrix<T, 3, 3>, // still private
}

impl<T: Float> DirectionCosineMatrix<T> {
    pub fn new(
        m11: T, m12: T, m13: T,
        m21: T, m22: T, m23: T,
        m31: T, m32: T, m33: T,
    ) -> Self {
        Self {
            data: Matrix {
                data: [
                    [m11, m12, m13],
                    [m21, m22, m23],
                    [m31, m32, m33],
                ],
            }
        }
    }

    pub fn from_matrix(mat3: Matrix<T, 3, 3>) -> Self {
        Self { data: mat3 }
    }

    pub fn as_matrix(&self) -> &Matrix<T, 3, 3> {
        &self.data
    }
    
    pub fn rotate_x(angle: T) -> Self {
        let (c, s) = (angle.cos(), angle.sin());
        let data: [[T; 3]; 3] = [
            [T::one(), T::zero(), T::zero()],
            [T::zero(), c, s],
            [T::zero(), -s, c],
        ];
        Self { data: Matrix { data } }
    }

    pub fn rotate_y(angle: T) -> Self {
        let (c, s) = (angle.cos(), angle.sin());
        let data = [
            [c, T::zero(), -s],
            [T::zero(), T::one(), T::zero()],
            [s, T::zero(), c],
        ];
        Self { data: Matrix { data } }
    }

    pub fn rotate_z(angle: T) -> Self {
        let (c, s) = (angle.cos(), angle.sin());

        let data = [
            [c, s, T::zero()],
            [-s, c, T::zero()],
            [T::zero(), T::zero(), T::one()],
        ];
        Self { data: Matrix { data } }
    }
}

// Operators
// Scalar multiplication: DCM * scalar
impl<T: Float> Mul<T> for DirectionCosineMatrix<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        Self{
            data: self.data * rhs
        }
    }
}
impl<T: Float + Mul<Output = T> + Add<Output = T> + Copy + Default> Mul for DirectionCosineMatrix<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            data: self.data * rhs.data, // Now valid
        }
    }
}

// From
impl<T> From<Euler<T>> for DirectionCosineMatrix<T>
where
    T: Float + Mul<Output = T> + Add<Output = T> + Copy + Default,
{
    fn from(euler: Euler<T>) -> Self {
        // ZYX Convention
        let [phi, theta, psi] = euler.data.data;
        let rx = DirectionCosineMatrix::rotate_x(phi);
        let ry = DirectionCosineMatrix::rotate_y(theta);
        let rz = DirectionCosineMatrix::rotate_z(psi);
        let c = rz * ry * rx;

        return c;
    }
}

impl<T: Float> From<Quaternion<T>> for DirectionCosineMatrix<T> {
    fn from(q: Quaternion<T>) -> Self {
        let s = q.data.data[0];
        let i= q.data.data[1];
        let j = q.data.data[2];
        let k = q.data.data[3];

        let two = T::one() + T::one();
        let sp2 = s.powf(two);
        let ip2 = i.powf(two);
        let jp2 = j.powf(two);
        let kp2 = k.powf(two);

        let data = [
            [
                sp2 + ip2 - jp2 - kp2,
                two * (i * j + s * k),
                two * (i * k - s * j)
            ],
            [
                two * (i * j - s * k),
                sp2 - ip2 + jp2 - kp2,
                two * (j * k + s * i)
            ],
            [
                two * (i * k + s * j),
                two * (j * k - s * i),
                sp2 - ip2 - jp2 + kp2
            ],
        ];
        Self { data: Matrix { data } }
    }
}
impl<T: Float> From<&Matrix<T, 3, 3>> for DirectionCosineMatrix<T> {
    fn from(m: &Matrix<T, 3, 3>) -> Self {
        Self {
            data: *m
        }
    }
}

#[cfg(feature = "std")]
impl<T: Float + std::fmt::Display> std::fmt::Display for DirectionCosineMatrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

// Tests
#[cfg(test)]
#[path = "tests/dcm_tests.rs"]
mod dcm_tests;