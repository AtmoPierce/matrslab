mod matrix;
mod vector;
pub mod macros;
pub use vector::Vector;
pub use matrix::Matrix;

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;