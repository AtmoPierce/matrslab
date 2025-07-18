use std::f64::consts::{PI, FRAC_PI_2, FRAC_PI_4};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coordinate::*;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-10
    }
    fn approx_eq_angle(a: f64, b: f64) -> bool {
        ((a - b + PI).rem_euclid(2.0 * PI) - PI).abs() < 1e-10
    }

    #[test]
    fn test_cartesian_cylindrical_roundtrip() {
        let originals = [
            CartesianPosition::new(3.0_f64, 4.0, 1.0),
            CartesianPosition::new(0.0, 0.0, 1.0),
            CartesianPosition::new(-3.0,-4.0, 1.0),
            CartesianPosition::new(2.0, -2.0, 1.0),
        ];
        for cart in &originals {
            let cyl: CylindricalPosition<f64> = cart.into();
            let cart2: CartesianPosition<f64> = (&cyl).into();
            assert!(approx_eq(cart.x(), cart2.x()));
            assert!(approx_eq(cart.y(), cart2.y()));
            assert!(approx_eq(cart.z(), cart2.z()));
        }
    }
    #[test]
    fn test_cartesian_spherical_roundtrip() {
        let originals = [
            CartesianPosition::new(1.0, 0.0, 0.0),
            CartesianPosition::new(0.0, 1.0, 0.0),
            CartesianPosition::new(0.0, 0.0, 1.0),
            CartesianPosition::new(1.0, 1.0, 1.0),
            CartesianPosition::new(-1.0, -1.0, -1.0),
        ];

        for cart in &originals {
            let sph: SphericalPosition<f64> = cart.into();
            let cart2: CartesianPosition<f64> = (&sph).into();

            assert!(approx_eq(cart.x(), cart2.x()), "x: {} vs {}", cart.x(), cart2.x());
            assert!(approx_eq(cart.y(), cart2.y()), "y: {} vs {}", cart.y(), cart2.y());
            assert!(approx_eq(cart.z(), cart2.z()), "z: {} vs {}", cart.z(), cart2.z());
        }
    }

    #[test]
    fn test_cylindrical_cartesian_roundtrip() {
        let originals = [
            CylindricalPosition::new(5.0, FRAC_PI_4 , 1.0),
            CylindricalPosition::new(0.0, PI, 1.0),
            CylindricalPosition::new(2.0, -FRAC_PI_2, 1.0 ),
            CylindricalPosition::new(3.0, 2.0, 1.0)
        ];
        for cyl in &originals {
            let cart: CartesianPosition<f64> = cyl.into();
            let cyl2: CylindricalPosition<f64> = (&cart).into();
            assert!(approx_eq(cyl.r(), cyl2.r()));
            assert!(approx_eq(cyl.theta(), cyl2.theta()));
            assert!(approx_eq(cyl.z(), cyl2.z()));
        }
    }

    #[test]
    fn test_cylindrical_spherical_roundtrip() {
        let originals = [
            CylindricalPosition::new(1.0, 0.0, 0.0),
            CylindricalPosition::new(2.0, PI, 1.0),
            CylindricalPosition::new(3.0, FRAC_PI_2, -1.0),
            CylindricalPosition::new(4.0, -FRAC_PI_2, 2.0),
        ];
        for cyl in &originals {
            let sph: SphericalPosition<f64> = cyl.into();
            let cyl2: CylindricalPosition<f64> = (&sph).into();
            assert!(approx_eq(cyl.r(), cyl2.r()));
            assert!(approx_eq(cyl.theta(), cyl2.theta()));
            assert!(approx_eq(cyl.z(), cyl2.z()));
        }
    }

    #[test]
    fn test_spherical_cylindrical_roundtrip() {
        let originals = [
            SphericalPosition::new(1.0, 0.0, FRAC_PI_2),
            SphericalPosition::new(2.0, PI, FRAC_PI_2),
            SphericalPosition::new(3.0, FRAC_PI_2, FRAC_PI_4),
            SphericalPosition::new(5.0, -FRAC_PI_2, FRAC_PI_4),
        ];
        for sph in &originals {
            let cyl: CylindricalPosition<f64> = sph.into();
            let sph2: SphericalPosition<f64> = (&cyl).into();
            assert!(approx_eq(sph.r(), sph2.r()));
            assert!(approx_eq(sph.theta(), sph2.theta()));
            assert!(approx_eq(sph.phi(), sph2.phi()));
        }
    }
    #[test]
    fn test_spherical_cartesian_roundtrip() {
        let originals = [
            SphericalPosition::new(1.0, 0.0, FRAC_PI_2),     // +x
            SphericalPosition::new(1.0, PI, FRAC_PI_2),      // -x
            SphericalPosition::new(1.0, FRAC_PI_2, FRAC_PI_2), // +y
            SphericalPosition::new(1.0, 0.0, 0.0),           // +z pole
            SphericalPosition::new(1.0, 0.0, PI),            // -z pole
        ];
        for sph in &originals {
            let cart: CartesianPosition<f64> = sph.into();
            let sph2: SphericalPosition<f64> = (&cart).into();

            assert!(approx_eq(sph.r(), sph2.r()));
            if sph.phi().abs() < 1e-6 || (sph.phi() - PI).abs() < 1e-6 {
                // At poles, azimuth is undefined, skip phi/theta check
                println!("Hi");
            } else {
                assert!(approx_eq_angle(sph.theta(), sph2.theta()));
                assert!(approx_eq(sph.phi(), sph2.phi()));
            }
        }
    }
}
