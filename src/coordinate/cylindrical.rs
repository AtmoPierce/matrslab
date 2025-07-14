use super::polar::PolarPosition;
use super::cartesian::CartesianPosition;
use crate::mathematics::mathematics::Vec3;

pub struct CylindricalPosition {
    data: Vec3
}
impl CylindricalPosition {
    pub fn new(r: f64, phi: f64, z: f64) -> Self {
        CylindricalPosition {Vec3::from([r, phi, z])}
    }
}

impl From<&CartesianPosition> for CylindricalPosition{
    fn from(cartesian: &CartesianPosition) -> Self {
    }
}

impl From<&PolarPosition> for CylindricalPosition {
    fn from(polar: &PolarPosition) -> Self {
    }
}