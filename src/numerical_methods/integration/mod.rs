pub mod euler;
pub mod trapezoidal;
pub mod rk4;
pub use euler::EulerIntegrate;
pub use rk4::Rk4Integrate;
pub use trapezoidal::TrapezoidalIntegrate;
