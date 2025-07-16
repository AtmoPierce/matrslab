#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use crate::Matrix;
    use crate::rotation::{DirectionCosineMatrix, Euler, Quaternion};
    use num_traits::Float;

    const EPSILON: f64 = 1e-6;

    fn matrices_approx_eq<T: Float>(a: &Matrix<T, 3, 3>, b: &Matrix<T, 3, 3>, epsilon: T) -> bool {
        for r in 0..3 {
            for c in 0..3 {
                if (a.data[r][c] - b.data[r][c]).abs() > epsilon {
                    return false;
                }
            }
        }
        true
    }

    #[test]
    fn test_dcm_quaternion_roundtrip() {
        let original = DirectionCosineMatrix::new(0.306185853, -0.250000803, 0.918557021, 0.8838825, 0.433011621, -0.176776249, -0.35355216, 0.866024084, 0.353553866);
        // Convert to Quaternion and back
        let q = Quaternion::from(&original);
        println!("Q: {}", q);
        let dcm_back = DirectionCosineMatrix::from(q);
        println!("{}", original.as_matrix());
        println!("{}", dcm_back.as_matrix());
        assert!(matrices_approx_eq(
            &original.as_matrix(),
            &dcm_back.as_matrix(),
            EPSILON,
        ));
    }

    #[test]
    fn test_dcm_euler_roundtrip() {
        let angles = [0.5, 0.0, 0.0]; // identity
        let euler = Euler::new(angles[0], angles[1], angles[2]);
        let dcm = DirectionCosineMatrix::from(euler);
        let back = Euler::from(&dcm);
        assert_relative_eq!(euler.data.data[0], back.data.data[0], epsilon = 1e-10);
        assert_relative_eq!(euler.data.data[1], back.data.data[1], epsilon = 1e-10);
        assert_relative_eq!(euler.data.data[2], back.data.data[2], epsilon = 1e-10);
    }
}
