use super::vector::Vector;
use num_traits::Float;
use core::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix<T, const ROWS: usize, const COLS: usize> {
    pub data: [[T; COLS]; ROWS],
}

// Matrix addition
impl<T, const R: usize, const C: usize> Add for Matrix<T, R, C>
where
    T: Float + Add<Output = T> + Copy,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut result = self;
        for r in 0..R {
            for c in 0..C {
                result.data[r][c] = self.data[r][c] + rhs.data[r][c];
            }
        }
        result
    }
}

// Matrix subtraction
impl<T, const R: usize, const C: usize> Sub for Matrix<T, R, C>
where
    T: Float + Sub<Output = T> + Copy,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut result = self;
        for r in 0..R {
            for c in 0..C {
                result.data[r][c] = self.data[r][c] - rhs.data[r][c];
            }
        }
        result
    }
}

// Scalar multiplication
impl<T, const R: usize, const C: usize> Mul<T> for Matrix<T, R, C>
where
    T: Float + Mul<Output = T> + Copy,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        let mut result = self;
        for r in 0..R {
            for c in 0..C {
                result.data[r][c] = self.data[r][c] * rhs;
            }
        }
        result
    }
}

// Scalar division
impl<T, const R: usize, const C: usize> Div<T> for Matrix<T, R, C>
where
    T: Float + Div<Output = T> + Copy,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        let mut result = self;
        for r in 0..R {
            for c in 0..C {
                result.data[r][c] = self.data[r][c] / rhs;
            }
        }
        result
    }
}

// Negation
impl<T, const R: usize, const C: usize> Neg for Matrix<T, R, C>
where
    T: Float + Neg<Output = T> + Copy,
{
    type Output = Self;
    fn neg(self) -> Self {
        let mut result = self;
        for r in 0..R {
            for c in 0..C {
                result.data[r][c] = -self.data[r][c];
            }
        }
        result
    }
}

// Matrix multiplication
impl<T, const M: usize, const N: usize, const P: usize> Mul<Matrix<T, N, P>> for Matrix<T, M, N>
where
    T: Float + Mul<Output = T> + Add<Output = T> + Copy + Default,
{
    type Output = Matrix<T, M, P>;
    fn mul(self, rhs: Matrix<T, N, P>) -> Matrix<T, M, P> {
        let mut result = Matrix {
            data: [[T::default(); P]; M],
        };
        for i in 0..M {
            for j in 0..P {
                let mut sum = T::zero();
                for k in 0..N {
                    sum = sum + self.data[i][k] * rhs.data[k][j];
                }
                result.data[i][j] = sum;
            }
        }
        result
    }
}

// Matrix × Vector multiplication
impl<T, const M: usize, const N: usize> Mul<Vector<T, N>> for Matrix<T, M, N>
where
    T: Float + Mul<Output = T> + Add<Output = T> + Copy + Default,
{
    type Output = Vector<T, M>;
    fn mul(self, rhs: Vector<T, N>) -> Vector<T, M> {
        let mut result = Vector { data: [T::default(); M] };
        for i in 0..M {
            let mut sum = T::zero();
            for j in 0..N {
                sum = sum + self.data[i][j] * rhs.data[j];
            }
            result.data[i] = sum;
        }
        result
    }
}
