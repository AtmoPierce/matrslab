use super::cylindrical::CylindricalPosition;
use super::spherical::SphericalPosition;
use crate::Vector;
use num_traits::Float;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CartesianPosition<T: Float> {
    pub data: Vector<T, 3>,
}

impl<T: Float> CartesianPosition<T>{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { data: Vector { data: [x, y, z] } }
    }
    pub fn x(&self) -> T{ self.data.data[0] }
    pub fn y(&self) -> T{ self.data.data[1] }
    pub fn z(&self) -> T{ self.data.data[2] }
}

impl<T: Float> From<&SphericalPosition<T>> for CartesianPosition<T> {
    fn from(p: &SphericalPosition<T>) -> Self {
        let r = p.r();
        let phi = p.phi();     // azimuth
        let theta = p.theta(); // inclination
        let sin_theta = theta.sin();
        let x = r * sin_theta * phi.cos();
        let y = r * sin_theta * phi.sin();
        let z = r * theta.cos();
        CartesianPosition::new(x, y, z)
    }
}

impl<T: Float> From<&CylindricalPosition<T>> for CartesianPosition<T>{
    fn from(c: &CylindricalPosition<T>) -> Self {
        let x = c.r()*c.theta().cos();
        let y = c.r()*c.theta().sin();
        let z = c.z(); 
        CartesianPosition::new(x, y, z)
    }
}