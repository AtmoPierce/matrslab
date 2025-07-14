use super::polar::PolarPosition;
use super::cylindrical::CylindricalPosition;
use crate::mathematics::mathematics::Vec3;

pub struct CartesianPosition{
    data: Vec3,
}

impl CartesianPosition {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        CartesianPosition { x, y, z, data: Vec3::from([x, y, z]) }
    }
}

impl From<&PolarPosition> for CartesianPosition {
    fn from(radial: &PolarPosition) -> Self {
    }
}

impl From<&CylindricalPosition> for CartesianPosition {
    fn from(cylindrical: &CylindricalPosition) -> Self {
    }
}