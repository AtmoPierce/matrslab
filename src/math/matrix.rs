use super::vector::Vector;
use num_traits::Float;
use core::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix<T, const M: usize, const N: usize> {
    pub data: [[T; N]; M],
}

// Matrix addition
impl<T, const M: usize, const N: usize> Add for Matrix<T, M, N>
where
    T: Float + Add<Output = T> + Copy,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut result = self;
        for r in 0..M {
            for c in 0..N {
                result.data[r][c] = self.data[r][c] + rhs.data[r][c];
            }
        }
        result
    }
}

// Matrix subtraction
impl<T, const M: usize, const N: usize> Sub for Matrix<T, M, N>
where
    T: Float + Sub<Output = T> + Copy,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut result = self;
        for r in 0..M {
            for c in 0..N {
                result.data[r][c] = self.data[r][c] - rhs.data[r][c];
            }
        }
        result
    }
}

// Scalar multiplication
impl<T, const M: usize, const N: usize> Mul<T> for Matrix<T, M, N>
where
    T: Float + Mul<Output = T> + Copy,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        let mut result = self;
        for r in 0..M {
            for c in 0..N {
                result.data[r][c] = self.data[r][c] * rhs;
            }
        }
        result
    }
}

// Scalar division
impl<T, const M: usize, const N: usize> Div<T> for Matrix<T, M, N>
where
    T: Float + Div<Output = T> + Copy,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        let mut result = self;
        for r in 0..M {
            for c in 0..N {
                result.data[r][c] = self.data[r][c] / rhs;
            }
        }
        result
    }
}

// Negation
impl<T, const M: usize, const N: usize> Neg for Matrix<T, M, N>
where
    T: Float + Neg<Output = T> + Copy,
{
    type Output = Self;
    fn neg(self) -> Self {
        let mut result = self;
        for r in 0..M {
            for c in 0..N {
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

// Generic Matrix Implementations
impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Float + Default + Copy,
{
    pub fn zeros() -> Self {
        Self { data: [[T::zero(); N]; M] }
    }
    pub fn ones() -> Self {
        Self { data: [[T::one(); N]; M] }
    }
}

impl<T, const N: usize> Matrix<T, N, N>
where
    T: Float + Default + Copy,
{
    // Returns an identity matrix
    pub fn identity() -> Self {
        let mut data = [[T::zero(); N]; N];
        for i in 0..N {
            data[i][i] = T::one();
        }
        Self { data }
    }
    // Returns a diagonal matrix with the elements of `diag` on the diagonal.
    pub fn diag(diag: &[T; N]) -> Self {
        let mut data = [[T::zero(); N]; N];
        for i in 0..N {
            data[i][i] = diag[i];
        }
        Self { data }
    }
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where T: Copy + Default,
{
    pub fn transpose(&self) -> Matrix<T, N, M> {
        let mut out = [[T::default(); M]; N];
        for i in 0..M {
            for j in 0..N {
                out[j][i] = self.data[i][j];
            }
        }
        Matrix { data: out }
    }
}

impl<T: Copy + core::ops::AddAssign + Default, const N: usize> Matrix<T, N, N> {
    pub fn trace(&self) -> T {
        let mut sum = T::default();
        for i in 0..N {
            sum += self.data[i][i];
        }
        sum
    }
}

// Determinant
impl<T: Float + Copy> Matrix<T, 2, 2> {
    pub fn determinant(&self) -> T {
        self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]
    }
}
impl<T: Float + Copy> Matrix<T, 3, 3> {
    pub fn determinant(&self) -> T {
        let m = &self.data;
        m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
      - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
      + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
    }
}
// Larger size matrices to come through LU Decomp or Laplace expansion, need to research.


// std
#[cfg(feature = "std")]
use std::fmt;
#[cfg(feature = "std")]
impl<T, const M: usize, const N: usize> fmt::Display for Matrix<T, M, N>
where
    T: Float + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, row) in self.data.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                write!(f, "{}", val)?;
                if j < N - 1 {
                    write!(f, " ")?; // space between columns
                }
            }
            if i < M - 1 {
                write!(f, "; ")?; // semicolon + space between rows
            }
        }
        write!(f, "]")
    }
}