mod euler;
mod dcm;
mod quaternion;

pub mod rotation{
    use super::{euler::Euler, dcm::DirectionCosineMatrix, quaternion::Quaternion};
    enum Rotation {
        Euler(Euler),
        DirectionCosineMatrix(DirectionCosineMatrix),
        Quaternion(Quaternion),
    }
}