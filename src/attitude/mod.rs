pub mod euler;
pub mod dcm;
pub mod quaternion;
pub mod rotation;
pub use euler::Euler;
pub use dcm::DirectionCosineMatrix;
pub use quaternion::Quaternion;
pub use rotation::Rotation;
#[cfg(test)]
#[path = "tests/mod.rs"]
pub mod tests;

pub mod attitude{
    use num_traits::Float;
    use super::dcm::DirectionCosineMatrix;
    use super::euler::Euler;
    use super::quaternion::Quaternion;
    pub enum Rotation<T> 
        where T: Float
            {
            DCMRotation(DirectionCosineMatrix<T>),
            EulerRotation(Euler<T>),
            QuaternionRotation(Quaternion<T>),
    }
}