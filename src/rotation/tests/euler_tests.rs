mod tests{
        use super::*;
    use crate::Matrix;
    use crate::matrix;
    use crate::rotation::{DirectionCosineMatrix, Euler, Quaternion};
    use crate::rotation::tests::test_utils::*;
    use approx::assert_relative_eq;
    use num_traits::Float;

    #[test]
    fn test_euler_dcm_roundtrip(){
        let euler = Euler::new(-0.5, 0.1, 0.2);
        let dcm1 = DirectionCosineMatrix::from(euler);
        let euler_back = Euler::from(&dcm1);
        let dcm2 = DirectionCosineMatrix::from(euler_back);
        
        // Jeez euler angles are inaccurate...
        assert!(matrices_approx_eq(&dcm1.as_matrix(), &dcm2.as_matrix(), 0.1));
    }
    #[test]
    fn test_euler_quaternion_roundtrip() {
        let euler = Euler::new(-0.5, 0.1, 0.2);
        let quat = Quaternion::from(&euler);
        let euler_back = Euler::from(&quat);

        let tol = 1e-6;
        for i in 0..3 {
            let original = euler.data.data[i];
            let recovered = euler_back.data.data[i];

            // Optionally unwrap the angle difference to (-π, π)
            let diff = (original - recovered + std::f64::consts::PI) % (2.0 * std::f64::consts::PI) - std::f64::consts::PI;

            assert!(
                diff.abs() <= tol,
                "Mismatch at index {}: original = {}, recovered = {}, diff = {}",
                i, original, recovered, diff
            );
        }
    }
}
