use super::cylindrical::CylindricalPosition;
use super::spherical::SphericalPosition;
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

impl From<&SphericalPosition> for CartesianPosition {
    fn from(p: &SphericalPosition) -> Self {
        let r = p.r();
        let theta = p.theta();
        let phi = p.phi();
        let x = r * phi.sin() * theta.cos();
        let y = r * phi.sin() * theta.sin();
        let z = r * phi.cos();

        CartesianPosition::new(x, y, z)
    }
}

impl From<&CylindricalPosition> for CartesianPosition {
    fn from(c: &CylindricalPosition) -> Self {
        let x = c.r()*c.theta().cos();
        let y = c.r()*c.theta().sin();
        let z = c.z(); 
        CartesianPosition::new(x, y, z)
    }
}