use super::quaternion::Quaternion;
use super::dcm::DirectionCosineMatrix;
use crate::mathematics::mathematics::Vec3;

pub struct Euler{
    pub data: Vec3,
}
impl Euler{
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Euler { Vec3::from([x, y, z]) }
    }
    pub fn to_quaternion(&self) -> Quaternion {
        Quaternion::from(self)
    }
    pub fn to_direction_cosine_matrix(&self) -> DirectionCosineMatrix {
        DirectionCosineMatrix::from(self)
    }
}

impl From<&Quaternion> for Euler {
    fn from(q: &Quaternion) -> Self {
    }
}

impl From<&DirectionCosineMatrix> for Euler {
    fn from(dcm: &DirectionCosineMatrix) -> Self {
    }
}