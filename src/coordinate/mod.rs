mod cartesian;
mod spherical;
mod cylindrical;
pub use cartesian::CartesianPosition;
pub use cylindrical::CylindricalPosition;
pub use spherical::SphericalPosition;

pub mod coordinate{
    use crate::coordinate::{CartesianPosition, CylindricalPosition, SphericalPosition};
    pub enum Position{
        Cartesian(CartesianPosition),
        Cylindrical(CylindricalPosition),
        Spherical(SphericalPosition)
    }
}

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;