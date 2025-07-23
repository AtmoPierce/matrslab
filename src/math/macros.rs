// Matrix
/// Construct a statically typed matrix from literals
/// Example: `matrix![1.0, 2.0; 3.0, 4.0]` produces a Matrix<_, 2, 2>
#[macro_export]
macro_rules! matrix {
    // Accept input like: matrix![1.0, 2.0; 3.0, 4.0]
    ( $( [ $( $x:expr ),+ $(,)? ] ),+ $(,)? ) => {{
        use $crate::math::Matrix;
        Matrix {
            data: [
                $(
                    [ $( $x ),+ ],
                )+
            ]
        }
    }};
    // Alternative: Accept without inner brackets
    ( $( $( $x:expr ),+ );+ $(;)? ) => {{
        matrix![
            $(
                [ $( $x ),+ ]
            ),+
        ]
    }};
}
#[macro_export]
macro_rules! eye {
    ($n:expr) => {
        Matrix::identity($n)
    };
}

#[macro_export]
macro_rules! zeros {
    ($m:expr, $n:expr) => {
        Matrix::zeros($m, $n)
    };
}

#[macro_export]
macro_rules! ones {
    ($m:expr, $n:expr) => {
        Matrix::ones($m, $n)
    };
}

// Vectors
use num_traits::Float;
use crate::math::Vector;
impl<T, const N: usize> Vector<T, N>
where
    T: Float + Default + Copy,
{
    pub fn zeros() -> Self {
        Self { data: [T::zero(); N] }
    }
    pub fn ones() -> Self {
        Self { data: [T::one(); N] }
    }
}