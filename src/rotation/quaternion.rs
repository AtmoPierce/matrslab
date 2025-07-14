use super::dcm::DirectionCosineMatrix;
use super::euler::Euler;

pub struct Quaternion {
    data: Vec4
}
impl Quaternion{
    pub fn new(w: f64, x: f64, y: f64, z: f64) -> Self {
        Quaternion {
            data: Vec4::new(w, x, y, z)
        }
    }

    pub fn to_euler(&self) -> Euler {
        Euler::from_quaternion(self)
    }

    pub fn to_direction_cosine_matrix(&self) -> DirectionCosineMatrix{
        DirectionCosineMatrix::from_quaternion(self)
    }
}

impl From<Euler> for Quaternion {
    fn from(euler: Euler) -> Self {
    }
}

impl From<DirectionCosineMatrix> for Quaternion {
    fn from(dcm: DirectionCosineMatrix) -> Self {
    }
}