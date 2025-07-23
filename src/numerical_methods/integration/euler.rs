use num_traits::Float;
use crate::math::Vector;
use crate::coordinate::Cartesian;

pub trait EulerIntegrate<T: Float> {
    fn integrate_euler(&self, dt: T) -> Self;
}

impl<T: Float + Copy, const N: usize> EulerIntegrate<T> for Vector<T, N> {
    fn integrate_euler(&self, dt: T) -> Self {
        *self * dt
    }
}

impl<T: Float + Copy, RF> EulerIntegrate<T> for Cartesian<T, RF> {
    fn integrate_euler(&self, dt: T) -> Self {
        Cartesian {
            data: self.data * dt,
            ..*self
        }
    }
}