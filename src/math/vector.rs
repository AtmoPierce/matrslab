use core::ops::{Add, Sub, Mul, Div, Neg};
use num_traits::Float;
use crate::utils::{ToRadians, ToDegrees};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<T, const N: usize> {
    pub data: [T; N],
}

// Add: Vector + Vector
impl<T, const N: usize> Add for Vector<T, N>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut result = self;
        for i in 0..N {
            result.data[i] = self.data[i] + rhs.data[i];
        }
        result
    }
}

// Sub: Vector - Vector
impl<T, const N: usize> Sub for Vector<T, N>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut result = self;
        for i in 0..N {
            result.data[i] = self.data[i] - rhs.data[i];
        }
        result
    }
}

// Neg: -Vector
impl<T, const N: usize> Neg for Vector<T, N>
where
    T: Neg<Output = T> + Copy,
{
    type Output = Self;
    fn neg(self) -> Self {
        let mut result = self;
        for i in 0..N {
            result.data[i] = -self.data[i];
        }
        result
    }
}

// Scalar multiplication: Vector * scalar
impl<T, const N: usize> Mul<T> for Vector<T, N>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        let mut result = self;
        for i in 0..N {
            result.data[i] = self.data[i] * rhs;
        }
        result
    }
}

// Scalar division: Vector / scalar
impl<T, const N: usize> Div<T> for Vector<T, N>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        let mut result = self;
        for i in 0..N {
            result.data[i] = self.data[i] / rhs;
        }
        result
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Mul<Output = T> + Copy + Float,
{
    pub fn dot(&self, rhs: &Self) -> T {
        let mut acc = T::zero();
        for i in 0..N {
            acc = acc + self.data[i] * rhs.data[i];
        }
        acc
    }
    pub fn norm(&self) -> T {
        let mut acc = T::zero();
        for &x in &self.data {
            acc = acc + x * x;
        }
        acc.sqrt()
    }
    pub fn angle(&self, rhs: &Self) -> T {
        let dot = self.dot(rhs);
        let denom = self.norm() * rhs.norm();
        if denom == T::zero() {
            T::zero()
        } else {
            let cos_theta = (dot / denom).max(T::from(-1.0).unwrap()).min(T::one());
            cos_theta.acos()
        }
    }
}

// Cross-product: only for N=3
impl<T> Vector<T, 3>
where
    T: Copy + Sub<Output = T> + Mul<Output = T>,
{
    pub fn cross(self, rhs: Self) -> Self {
        let [a1, a2, a3] = self.data;
        let [b1, b2, b3] = rhs.data;
        Self {
            data: [
                a2 * b3 - a3 * b2,
                a3 * b1 - a1 * b3,
                a1 * b2 - a2 * b1,
            ],
        }
    }
}


// units
impl<T, const N: usize> ToRadians for Vector<T, N>
where
    T: ToRadians + Copy,
{
    type Output = Vector<T::Output, N>;
    fn to_radians(self) -> Self::Output {
        let data = self.data.map(|x| x.to_radians());
        Vector { data }
    }
}

impl<T, const N: usize> ToDegrees for Vector<T, N>
where
    T: ToDegrees + Copy,
{
    type Output = Vector<T::Output, N>;
    fn to_degrees(self) -> Self::Output {
        let data = self.data.map(|x| x.to_degrees());
        Vector { data }
    }
}

// std 
#[cfg(feature = "std")]
use std::fmt;

#[cfg(feature = "std")]
impl<T: Float + fmt::Display, const N: usize> fmt::Display for Vector<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, val) in self.data.iter().enumerate() {
            write!(f, "{}", val)?;
            if i < N - 1 {
                write!(f, " ")?; // space between elements
            }
        }
        write!(f, "]")
    }
}
