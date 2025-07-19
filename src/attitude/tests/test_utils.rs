use super::*;
use approx::assert_relative_eq;
use crate::Matrix;
use crate::matrix;
use crate::attitude::{DirectionCosineMatrix, Euler, Quaternion};
use num_traits::Float;

pub fn matrices_approx_eq<T: Float + std::fmt::Debug>(
    a: &Matrix<T, 3, 3>,
    b: &Matrix<T, 3, 3>,
    epsilon: T,
) -> bool {
    let mut ok = true;
    for r in 0..3 {
        for c in 0..3 {
            let diff = (a.data[r][c] - b.data[r][c]).abs();
            if diff > epsilon {
                println!(
                    "Mismatch at ({}, {}): a = {:?}, b = {:?}, diff = {:?}",
                    r, c, a.data[r][c], b.data[r][c], diff
                );
                ok = false;
            }
        }
    }
    ok
}

pub fn test_round_trip<T>(dcm: DirectionCosineMatrix<T>, epsilon: T)
where
    T: Float + std::fmt::Debug,
{
    let q: Quaternion<T> = (&dcm).try_into().unwrap();        
    let q_n = q.normalized();
    let dcm_rt = DirectionCosineMatrix::from(q_n);
    assert!(matrices_approx_eq(
        dcm.as_matrix(),
        dcm_rt.as_matrix(),
        epsilon,
    ));
}