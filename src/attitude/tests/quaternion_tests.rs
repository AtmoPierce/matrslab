mod tests{
    use super::*;
    use crate::Matrix;
    use crate::matrix;
    use crate::attitude::{DirectionCosineMatrix, Euler, Quaternion};
    use crate::attitude::tests::test_utils::*;
    use approx::assert_relative_eq;
    use num_traits::Float;

    const EPSILON: f64 = 1e-6;

    #[test]
    fn test_quaternion_euler_roundtrip() {
        let q = Quaternion::new(0.9, 0.1, 0.2, 0.3).normalized();
        let euler = Euler::from(&q);
        let q_back = Quaternion::from(&euler).normalized();

        for i in 0..4 {
            let orig = q.data[i];
            let back = q_back.data[i];
            let diff = (orig - back).abs();

            assert!(
                diff <= EPSILON,
                "Mismatch at index {}: original = {}, recovered = {}, diff = {}",
                i, orig, back, diff
            );
        }
    }

    #[test]
    fn test_quaternion_dcm_roundtrip() {
        let q = Quaternion::new(0.9, 0.1, 0.2, 0.3).normalized();
        let dcm = DirectionCosineMatrix::from(q);
        let q_back = Quaternion::try_from(&dcm).unwrap().normalized();

        for i in 0..4 {
            let orig = q.data[i];
            let back = q_back.data[i];
            let diff = (orig - back).abs();

            assert!(
                diff <= EPSILON,
                "Mismatch at index {}: original = {}, recovered = {}, diff = {}",
                i, orig, back, diff
            );
        }
    }
}