mod cartesian;
mod spherical;
mod cylindrical;
pub use cartesian::Cartesian;
pub use cylindrical::Cylindrical;
pub use spherical::Spherical;

pub mod coordinate{
    use num_traits::Float;
    use crate::coordinate::{Cartesian, Cylindrical, Spherical};
    pub enum Coordinate<T: Float>{
        CartesianValue(Cartesian<T>),
        CylindricalValue(Cylindrical<T>),
        SphericalValue(Spherical<T>)
    }
}

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;