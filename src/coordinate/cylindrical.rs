use super::polar::PolarPosition;
use super::cartesian::CartesianPosition;
use crate::Vector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CylindricalPosition {
    pub data: Vector<f64, 3>, // [rho, phi, z]
}
impl CylindricalPosition {
    pub fn new(rho: f64, phi: f64, z: f64) -> Self {
        Self { data: Vector { data: [rho, phi, z] } }
    }
    pub fn rho(&self) -> f64 { self.data.data[0] }
    pub fn phi(&self) -> f64 { self.data.data[1] }
    pub fn z(&self) -> f64 { self.data.data[2] }
}

impl From<&CartesianPosition> for CylindricalPosition {
    fn from(cart: &CartesianPosition) -> Self {
        let x = cart.x();
        let y = cart.y();
        let z = cart.z();
        let rho = (x * x + y * y).sqrt();
        let phi = y.atan2(x);
        CylindricalPosition::new(rho, phi, z)
    }
}