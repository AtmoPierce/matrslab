use super::spherical::SphericalPosition;
use super::cartesian::CartesianPosition;
use crate::Vector;
use num_traits::Float;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CylindricalPosition<T: Float> {
    pub data: Vector<T, 3>, // [r, theta, z]
}
impl<T: Float> CylindricalPosition<T> {
    pub fn new(r: T, theta: T, z: T) -> Self {
        Self { data: Vector { data: [r, theta, z] } }
    }
    pub fn r(&self) -> T { self.data.data[0] }
    pub fn theta(&self) -> T { self.data.data[1] }
    pub fn z(&self) -> T { self.data.data[2] }
}

impl<T:Float> From<&CartesianPosition<T>> for CylindricalPosition<T> {
    fn from(cart: &CartesianPosition<T>) -> Self {
        let x = cart.x();
        let y = cart.y();
        let z = cart.z();
        let r = (x * x + y * y).sqrt();
        let theta = y.atan2(x);
        CylindricalPosition::new(r, theta, z)
    }
}

impl<T:Float> From<&SphericalPosition<T>> for CylindricalPosition<T>{
    fn from(s: &SphericalPosition<T>) -> Self{
        let r = s.r() * s.theta().sin();
        let z = s.r() * s.theta().cos();
        let theta = s.phi();
        return CylindricalPosition::new(r, theta, z)
    }
}