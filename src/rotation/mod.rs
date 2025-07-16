pub mod euler;
pub mod dcm;
pub mod quaternion;
pub use euler::Euler;
pub use dcm::DirectionCosineMatrix;
pub use quaternion::Quaternion;
#[cfg(test)]
#[path = "tests/mod.rs"]
pub mod tests;