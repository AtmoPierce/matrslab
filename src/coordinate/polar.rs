use super::cartesian::CartesianPosition;
use super::cylindrical::CylindricalPosition;
use crate::{Vector, Matrix};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PolarPosition {
    pub data: Vector<f64, 3>, // [r, theta, phi]
}

impl PolarPosition {
    pub fn new(r: f64, theta: f64, phi: f64) -> Self {
        Self { data: Vector { data: [r, theta, phi] } }
    }
    pub fn r(&self) -> f64 { self.data.data[0] }
    pub fn theta(&self) -> f64 { self.data.data[1] }
    pub fn phi(&self) -> f64 { self.data.data[2] }
}

impl From<&CartesianPosition> for PolarPosition {
    fn from(cart: &CartesianPosition) -> Self {
        let x = cart.x();
        let y = cart.y();
        let z = cart.z();
        let r = (x * x + y * y + z * z).sqrt();
        let theta = if r != 0.0 { (z / r).acos() } else { 0.0 };
        let phi = y.atan2(x);
        PolarPosition::new(r, theta, phi)
    }
}