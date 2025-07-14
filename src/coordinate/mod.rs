mod cartesian;
mod polar;
mod cylindrical;

pub mod coordinate{
    use super::{cartesian::CartesianPosition, polar::PolarPosition, cylindrical::CylindricalPosition};
    pub enum Position{
        Cartesian(CartesianPosition),
        Polar(PolarPosition),
        Cylindrical(CylindricalPosition),
    }
}