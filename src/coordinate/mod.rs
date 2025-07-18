mod cartesian;
mod spherical;
mod cylindrical;
pub use cartesian::CartesianPosition;
pub use cylindrical::CylindricalPosition;
pub use spherical::SphericalPosition;

pub mod coordinate{
    use num_traits::Float;
    use crate::coordinate::{CartesianPosition, CylindricalPosition, SphericalPosition};
    pub enum Position<T: Float>{
        Cartesian(CartesianPosition<T>),
        Cylindrical(CylindricalPosition<T>),
        Spherical(SphericalPosition<T>)
    }
}

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;