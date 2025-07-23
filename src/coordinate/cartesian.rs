use super::cylindrical::Cylindrical;
use super::spherical::Spherical;
use crate::math::Vector;
use num_traits::Float;
use core::marker::PhantomData; // Reference frame tracking.

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Cartesian<T: Float, ReferenceFrame> {
    pub data: Vector<T, 3>,
    pub _reference_frame: PhantomData<ReferenceFrame>
}

impl<T: Float, ReferenceFrame> Cartesian<T, ReferenceFrame>{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { data: Vector { data: [x, y, z] }, _reference_frame: PhantomData }
    }
    pub fn x(&self) -> T{ self.data.data[0] }
    pub fn y(&self) -> T{ self.data.data[1] }
    pub fn z(&self) -> T{ self.data.data[2] }
}


use core::ops::{Add, AddAssign, Sub, SubAssign, Neg, Mul, Div};
// ----- Add -----
impl<T: Float, RF> Add for Cartesian<T, RF> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            data: self.data + rhs.data,
            _reference_frame: PhantomData,
        }
    }
}
impl<'a, T: Float, RF> Add for &'a Cartesian<T, RF>
where
    Cartesian<T, RF>: Clone,
{
    type Output = Cartesian<T, RF>;
    fn add(self, rhs: Self) -> Self::Output {
        self.clone() + rhs.clone()
    }
}
impl<'a, T: Float, RF> Add<Cartesian<T, RF>> for &'a Cartesian<T, RF>
where
    Cartesian<T, RF>: Clone,
{
    type Output = Cartesian<T, RF>;
    fn add(self, rhs: Cartesian<T, RF>) -> Self::Output {
        self.clone() + rhs
    }
}
impl<'a, T: Float, RF> Add<&'a Cartesian<T, RF>> for Cartesian<T, RF>
where
    Cartesian<T, RF>: Clone,
{
    type Output = Cartesian<T, RF>;
    fn add(self, rhs: &'a Cartesian<T, RF>) -> Self::Output {
        self + rhs.clone()
    }
}

// ----- AddAssign -----
impl<T: Float + AddAssign, RF> AddAssign for Cartesian<T, RF> {
    fn add_assign(&mut self, rhs: Self) {
        self.data += rhs.data;
    }
}

// ----- Sub -----
impl<T: Float, RF> Sub for Cartesian<T, RF> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            data: self.data - rhs.data,
            _reference_frame: PhantomData,
        }
    }
}
impl<'a, T: Float, RF> Sub for &'a Cartesian<T, RF>
where
    Cartesian<T, RF>: Clone,
{
    type Output = Cartesian<T, RF>;
    fn sub(self, rhs: Self) -> Self::Output {
        self.clone() - rhs.clone()
    }
}
impl<'a, T: Float, RF> Sub<Cartesian<T, RF>> for &'a Cartesian<T, RF>
where
    Cartesian<T, RF>: Clone,
{
    type Output = Cartesian<T, RF>;
    fn sub(self, rhs: Cartesian<T, RF>) -> Self::Output {
        self.clone() - rhs
    }
}
impl<'a, T: Float, RF> Sub<&'a Cartesian<T, RF>> for Cartesian<T, RF>
where
    Cartesian<T, RF>: Clone,
{
    type Output = Cartesian<T, RF>;
    fn sub(self, rhs: &'a Cartesian<T, RF>) -> Self::Output {
        self - rhs.clone()
    }
}

// ----- SubAssign -----
impl<T: Float + SubAssign, RF> SubAssign for Cartesian<T, RF> {
    fn sub_assign(&mut self, rhs: Self) {
        self.data -= rhs.data;
    }
}

// ----- Neg -----
impl<T: Float, RF> Neg for Cartesian<T, RF> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            data: -self.data,
            _reference_frame: PhantomData,
        }
    }
}
impl<'a, T: Float, RF> Neg for &'a Cartesian<T, RF>
where
    Cartesian<T, RF>: Clone,
{
    type Output = Cartesian<T, RF>;
    fn neg(self) -> Self::Output {
        -self.clone()
    }
}

// ----- Scalar Mul -----
impl<T: Float + Copy, RF> Mul<T> for Cartesian<T, RF> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        Self {
            data: self.data * rhs,
            ..self
        }
    }
}
impl<'a, T: Float + Copy, RF> Mul<T> for &'a Cartesian<T, RF>
where
    Cartesian<T, RF>: Clone,
{
    type Output = Cartesian<T, RF>;
    fn mul(self, rhs: T) -> Self::Output {
        self.clone() * rhs
    }
}

// ----- Scalar Div -----
impl<T: Float + Copy, RF> Div<T> for Cartesian<T, RF> {
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        Self {
            data: self.data / rhs,
            ..self
        }
    }
}
impl<'a, T: Float + Copy, RF> Div<T> for &'a Cartesian<T, RF>
where
    Cartesian<T, RF>: Clone,
{
    type Output = Cartesian<T, RF>;
    fn div(self, rhs: T) -> Self::Output {
        self.clone() / rhs
    }
}

// Conversions
impl<T: Float, ReferenceFrame> From<&Spherical<T>> for Cartesian<T, ReferenceFrame> {
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

impl<T: Float, ReferenceFrame> From<&Cylindrical<T>> for Cartesian<T, ReferenceFrame>{
    fn from(c: &Cylindrical<T>) -> Self {
        let x = c.r()*c.theta().cos();
        let y = c.r()*c.theta().sin();
        let z = c.z(); 
        Cartesian::new(x, y, z)
    }
}

