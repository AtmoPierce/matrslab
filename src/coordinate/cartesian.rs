use super::polar::PolarPosition;
use super::cylindrical::CylindricalPosition;
use crate::Vector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CartesianPosition {
    pub data: Vector<f64, 3>,
}

impl CartesianPosition {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { data: Vector { data: [x, y, z] } }
    }
    pub fn x(&self) -> f64 { self.data.data[0] }
    pub fn y(&self) -> f64 { self.data.data[1] }
    pub fn z(&self) -> f64 { self.data.data[2] }
}

impl From<&PolarPosition> for CartesianPosition {
    fn from(p: &PolarPosition) -> Self {
        let r = p.r();
        let theta = p.theta();
        let phi = p.phi();
        let x = r * theta.sin() * phi.cos();
        let y = r * theta.sin() * phi.sin();
        let z = r * theta.cos();
        CartesianPosition::new(x, y, z)
    }
}

impl From<&CylindricalPosition> for CartesianPosition {
    fn from(c: &CylindricalPosition) -> Self {
        let rho = c.rho();
        let phi = c.phi();
        let z = c.z();
        let x = rho * phi.cos();
        let y = rho * phi.sin();
        CartesianPosition::new(x, y, z)
    }
}