use super::euler::Euler;
use super::quaternion::Quaternion;
use crate::mathematics::mathematics::Matrix3;

pub struct DirectionCosineMatrix {
    data: Matrix3,
}
impl DirectionCosineMatrix {
    pub fn new(
        m11: f64, m12: f64, m13: f64,
        m21: f64, m22: f64, m23: f64,
        m31: f64, m32: f64, m33: f64,
    ) -> Self {
        DirectionCosineMatrix {
            data: Matrix3::from([
                [m11, m12, m13],
                [m21, m22, m23],
                [m31, m32, m33],
            ]),
        }
    }
    pub fn new(mat3: Matrix3)->Self{
        DirectionCosineMatrix{
            data: mat3
        }
    }

    pub fn to_euler(&self) -> Euler {
        Euler::from(self)        
    }

    pub fn to_quaternion(&self) -> Quaternion {
        Quaternion::from(self)
    }
}

impl From<Euler> for DirectionCosineMatrix {
    fn from(euler: Euler) -> Self {
        DirectionCosineMatrix::new()
    }
}

impl From<Quaternion> for DirectionCosineMatrix {
    fn from(q: Quaternion) -> Self {
    }
}