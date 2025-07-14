use core::ops::{Add, Sub, Mul, Div, Neg};
use num_traits::Float;

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

// Dot product: a method, not an operator
impl<T, const N: usize> Vector<T, N>
where
    T: Mul<Output = T> + core::iter::Sum + Copy,
{
    pub fn dot(self, rhs: Self) -> T {
        (0..N).map(|i| self.data[i] * rhs.data[i]).sum()
    }
}

// Magnitude (Euclidean norm)
impl<T, const N: usize> Vector<T, N>
where
    T: Mul<Output = T> + Copy + core::iter::Sum + Float,
{
    pub fn norm(self) -> T {
        let sum: T = (0..N)
            .map(|i| (self.data[i] * self.data[i]).into())
            .sum();
        sum.sqrt()
    }
}

// Magnitude (returns T, works for floats)
impl<T, const N: usize> Vector<T, N>
where
    T: Float + core::iter::Sum,
{
    pub fn norm_t(self) -> T {
        let sum: T = (0..N).map(|i| self.data[i] * self.data[i]).sum();
        sum.sqrt()
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
