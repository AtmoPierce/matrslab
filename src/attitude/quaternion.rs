use crate::{Vector, Matrix};
use crate::attitude::{Euler, DirectionCosineMatrix};
use num_traits::Float;
use core::ops::{Mul, Add, Sub, Neg, Div};


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion<T: Float> {
    pub data: Vector<T, 4>, // [w, x, y, z]
}

impl<T: Float> Quaternion<T> {
    pub fn new(w: T, x: T, y: T, z: T) -> Self {
        Self { data: Vector { data: [w, x, y, z] } }
    }
    pub fn norm(&self) -> T {
        self.data.dot(&self.data).sqrt()
    }

    pub fn normalized(self) -> Self {
        let norm = self.norm();
        Self {
            data: self.data / norm,
        }
    }
    pub fn identity() -> Self {
        Self::new(
            T::one(), // w = 1
            T::zero(),
            T::zero(),
            T::zero(),
        )
    }
}

// Hamilton product for quaternion * quaternion
impl<T: Float> Mul for Quaternion<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let [w1, x1, y1, z1] = self.data.data;
        let [w2, x2, y2, z2] = rhs.data.data;

        let w = w1 * w2 - x1 * x2 - y1 * y2 - z1 * z2; 
        let x = w1 * x2 + x1 * w2 + y1 * z2 - z1 * y2;
        let y = w1 * y2 - x1 * z2 + y1 * w2 + z1 * x2;
        let z = w1 * z2 + x1 * y2 - y1 * x2 + z1 * w2;

        Self::new(w, x, y, z)
    }
}


impl<T: Float> TryFrom<&DirectionCosineMatrix<T>> for Quaternion<T> {
    type Error = ();
    fn try_from(dcm: &DirectionCosineMatrix<T>) -> Result<Self, Self::Error> {
        // Norm Constraint
        // https://motoq.github.io/doc/tnotes/dcmq.pdf
        let m = &dcm.as_matrix().data;
        let one = T::one();
        let two = one + one;
        let four = two + two;
        let k = one / four;

        let c1 = one + m[0][0] + m[1][1] + m[2][2];
        let c2 = one + m[0][0] - m[1][1] - m[2][2];
        let c3 = one + m[0][0] + m[1][1] - m[2][2];
        let c4 = one + m[0][0] - m[1][1] + m[2][2];
        if c1 > k{
            let qs = (c1 / four).sqrt();
            let qs4 =  qs * four;
            let qi = (m[1][2] - m[2][1]) / qs4;
            let qj = (m[2][0] - m[0][2]) / qs4;
            let qk = (m[0][1] - m[1][0]) / qs4;
            return Ok(Quaternion::new(qs, qi, qj, qk));
        }
        else if c2 > k{
            let qi = (c2 / four).sqrt();
            let qi4 = qi * four;
            let qs = (m[1][2] - m[2][1]) / qi4;
            let qj = (m[0][1] + m[1][0]) / qi4;
            let qk = (m[2][0] + m[0][2]) / qi4;
            return Ok(Quaternion::new(qs, qi, qj, qk));
        }
        else if c3 > k{
            let qj = (c3 / four).sqrt();
            let qj4 = qj * four;
            let qs = (m[2][0] - m[0][2]) / qj4;
            let qi = (m[0][1] + m[1][0]) / qj4;
            let qk = (m[1][2] + m[2][1]) / qj4;
            return Ok(Quaternion::new(qs, qi, qj, qk));
        }
        else if c4 > k{
            let qk = (c4 / four).sqrt();
            let qk4 = qk * four;
            let qs = (m[0][1] - m[1][0]) / qk4;
            let qi = (m[2][0] + m[0][2]) / qk4;
            let qj = (m[1][2] + m[2][1]) / qk4;
            return Ok(Quaternion::new(qs, qi, qj, qk));
        }
        else{
            Err(())
        }
    }
}

impl<T: Float> From<&Euler<T>> for Quaternion<T> {
    fn from(e: &Euler<T>) -> Self {
        // e.0, e.1, e.2 are roll, pitch, yaw
        let [roll, pitch, yaw] = e.data.data;
        let half = T::one() / (T::one() + T::one());
        let (cr, sr) = ((roll * half).cos(), (roll * half).sin());
        let (cp, sp) = ((pitch * half).cos(), (pitch * half).sin());
        let (cy, sy) = ((yaw * half).cos(), (yaw * half).sin());

        let w = cr * cp * cy + sr * sp * sy;
        let x = sr * cp * cy - cr * sp * sy;
        let y = cr * sp * cy + sr * cp * sy;
        let z = cr * cp * sy - sr * sp * cy;

        Quaternion::new(w, x, y, z)
    }
}

impl<T: Float> Mul<T> for Quaternion<T> {
    type Output = Quaternion<T>;
    fn mul(self, rhs: T) -> Quaternion<T> {
        Quaternion { data: self.data * rhs }
    }
}

// Elementwise addition
impl<T: Float> Add for Quaternion<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self { data: self.data + rhs.data }
    }
}

// Elementwise subtraction
impl<T: Float> Sub for Quaternion<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self { data: self.data - rhs.data }
    }
}

// Elementwise negation
impl<T: Float> Neg for Quaternion<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Self { data: -self.data }
    }
}

// Scalar division (q / scalar)
impl<T: Float> Div<T> for Quaternion<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        Self { data: self.data / rhs }
    }
}


// std
#[cfg(feature = "std")]
impl<T: Float + std::fmt::Display> std::fmt::Display for Quaternion<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.data)
    }
}

// Tests
#[cfg(test)]
#[path = "tests/quaternion_tests.rs"]
mod quaternion_tests;