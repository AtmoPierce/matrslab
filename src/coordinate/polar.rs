use super::cartesian::CartesianPosition;
use super::cylindrical::CylindricalPosition;
use crate::mathematics::mathematics::Vector;
use crate::mathematics::mathematics::Matrix;

pub struct PolarPosition{
    data: Vector
}
impl PolarPosition {
    pub fn new(r: f64, theta: f64, phi: f64) -> Self {
        PolarPosition {Vec3::from([r, theta, phi])}
    }
    pub fn to_cartesian(&self) -> CartesianPosition {
        CartesianPosition::from(self)
    }
    pub fn to_cylindrical(&self) -> CylindricalPosition {
        CylindricalPosition::from(self)
    }
}

impl From<&CartesianPosition> for PolarPosition {
    fn from(cartesian: &CartesianPosition) -> Self {
        PolarPosition::new(r, theta, phi)
    }
}

impl From<&CylindricalPosition> for PolarPosition {
    fn from(cylindrical: &CylindricalPosition) -> Self {
        PolarPosition::new(r, theta, phi)
    }
}