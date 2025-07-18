use num_traits::{Float, Pow};

use super::cartesian::CartesianPosition;
use super::cylindrical::CylindricalPosition;
use crate::{Vector, Matrix};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SphericalPosition {
    /// r: radial distance
    /// phi: azimuthal angle (radians, from +x in xy-plane)
    /// theta: inclination angle (radians (z-axis))
    pub data: Vector<f64, 3>, // [r, phi, theta]
}

impl SphericalPosition {
    pub fn new(r: f64, phi: f64, theta: f64) -> Self {
        Self { data: Vector { data: [r, phi, theta] } }
    }
    pub fn r(&self) -> f64 { self.data.data[0] }
    pub fn phi(&self) -> f64 { self.data.data[1] }
    pub fn theta(&self) -> f64 { self.data.data[2] }
}

impl From<&CartesianPosition> for SphericalPosition {
    fn from(cart: &CartesianPosition) -> Self {
        let x = cart.x();
        let y = cart.y();
        let z = cart.z();
        let rho = (x * x + y * y + z * z).sqrt();
        let theta = y.atan2(x); // azimuth angle in xy-plane
        let phi = if rho == 0.0 { 0.0 } else { (z / rho).acos() }; // inclination from z-axis
        SphericalPosition::new(rho, theta, phi)
    }
}

impl From<&CylindricalPosition> for SphericalPosition{
    fn from(c: &CylindricalPosition) -> Self{
        let rect: CartesianPosition = CartesianPosition::from(c);
        SphericalPosition::from(&rect)
    }
}