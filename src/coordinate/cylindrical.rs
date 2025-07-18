use super::spherical::SphericalPosition;
use super::cartesian::CartesianPosition;
use crate::Vector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CylindricalPosition {
    pub data: Vector<f64, 3>, // [r, theta, z]
}
impl CylindricalPosition {
    pub fn new(r: f64, theta: f64, z: f64) -> Self {
        Self { data: Vector { data: [r, theta, z] } }
    }
    pub fn r(&self) -> f64 { self.data.data[0] }
    pub fn theta(&self) -> f64 { self.data.data[1] }
    pub fn z(&self) -> f64 { self.data.data[2] }
}

impl From<&CartesianPosition> for CylindricalPosition {
    fn from(cart: &CartesianPosition) -> Self {
        let x = cart.x();
        let y = cart.y();
        let z = cart.z();
        let r = (x * x + y * y).sqrt();
        let theta = y.atan2(x);
        CylindricalPosition::new(r, theta, z)
    }
}

impl From<&SphericalPosition> for CylindricalPosition{
    fn from(s: &SphericalPosition) -> Self{
        let r = s.r() * s.phi().sin();
        let z = s.r() * s.phi().cos();
        let theta = s.theta();
        return CylindricalPosition::new(r, theta, z)
    }
}