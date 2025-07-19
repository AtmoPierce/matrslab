use super::cylindrical::Cylindrical;
use super::spherical::Spherical;
use crate::Vector;
use num_traits::Float;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cartesian<T: Float> {
    pub data: Vector<T, 3>,
}

impl<T: Float> Cartesian<T>{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { data: Vector { data: [x, y, z] } }
    }
    pub fn x(&self) -> T{ self.data.data[0] }
    pub fn y(&self) -> T{ self.data.data[1] }
    pub fn z(&self) -> T{ self.data.data[2] }
}

impl<T: Float> From<&Spherical<T>> for Cartesian<T> {
    fn from(p: &Spherical<T>) -> Self {
        let r = p.r();
        let phi = p.phi();     // azimuth
        let theta = p.theta(); // inclination
        let sin_theta = theta.sin();
        let x = r * sin_theta * phi.cos();
        let y = r * sin_theta * phi.sin();
        let z = r * theta.cos();
        Cartesian::new(x, y, z)
    }
}

impl<T: Float> From<&Cylindrical<T>> for Cartesian<T>{
    fn from(c: &Cylindrical<T>) -> Self {
        let x = c.r()*c.theta().cos();
        let y = c.r()*c.theta().sin();
        let z = c.z(); 
        Cartesian::new(x, y, z)
    }
}