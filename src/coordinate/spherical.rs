use num_traits::{Float, Pow};

use super::cartesian::CartesianPosition;
use super::cylindrical::CylindricalPosition;
use crate::{Vector, Matrix};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SphericalPosition<T: Float> {
    /// r: radial distance
    /// phi: azimuthal angle (radians, from +x in xy-plane)
    /// theta: inclination angle (radians (z-axis))
    pub data: Vector<T, 3>, // [r, phi, theta]
}

impl<T: Float> SphericalPosition<T>{
    pub fn new(r: T, phi: T, theta: T) -> Self {
        Self { data: Vector { data: [r, phi, theta] } }
    }
    pub fn r(&self) -> T { self.data.data[0] }
    pub fn phi(&self) -> T { self.data.data[1] }
    pub fn theta(&self) -> T { self.data.data[2] }
}

impl<T: Float> From<&CartesianPosition<T>> for SphericalPosition<T> {
    fn from(cart: &CartesianPosition<T>) -> Self {
        let x = cart.x();
        let y = cart.y();
        let z = cart.z();
        let rho = (x * x + y * y + z * z).sqrt();
        let theta = y.atan2(x); // azimuth angle in xy-plane
        let phi = if rho == T::zero() { T::zero() } else { (z / rho).acos() }; // inclination from z-axis
        SphericalPosition::new(rho, theta, phi)
    }
}

impl<T: Float> From<&CylindricalPosition<T>> for SphericalPosition<T>{
    fn from(c: &CylindricalPosition<T>) -> Self{
        let rect: CartesianPosition<T> = CartesianPosition::from(c);
        SphericalPosition::from(&rect)
    }
}