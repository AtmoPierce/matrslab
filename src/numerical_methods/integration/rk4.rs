use num_traits::Float;
use crate::math::Vector;
use crate::coordinate::Cartesian;

pub trait Rk4Integrate<T: Float> {
    /// RK4 step given `self` as the current state,
    /// `f` as the derivative function, and `dt` as the time step.
    fn integrate_rk4<F>(&self, f: F, dt: T) -> Self
    where
        F: Fn(&Self) -> Self;
}

impl<T: Float + Copy, const N: usize> Rk4Integrate<T> for Vector<T, N> {
    fn integrate_rk4<F>(&self, f: F, dt: T) -> Self
    where
        F: Fn(&Self) -> Self,
    {
        let k1 = f(self);
        let k2 = f(&(*self + k1.clone() * (dt / T::from(2.0).unwrap())));
        let k3 = f(&(*self + k2.clone() * (dt / T::from(2.0).unwrap())));
        let k4 = f(&(*self + k3.clone() * dt));
        *self
            + (k1 + k2 + k2 + k3 + k3 + k4)
                * (dt / T::from(6.0).unwrap())
    }
}

impl<T: Float + Copy, RF: Clone> Rk4Integrate<T> for Cartesian<T, RF> {
    fn integrate_rk4<F>(&self, f: F, dt: T) -> Self
    where
        F: Fn(&Self) -> Self,
    {
        let half_dt = dt / T::from(2.0).unwrap();
        let sixth_dt = dt / T::from(6.0).unwrap();

        let k1 = f(self);
        let k2 = f(&(self.clone() + k1.clone() * half_dt));
        let k3 = f(&(self.clone() + k2.clone() * half_dt));
        let k4 = f(&(self.clone() + k3.clone() * dt));

        Cartesian {
            data: self.data
                + (k1.data
                    + k2.data * T::from(2.0).unwrap()
                    + k3.data * T::from(2.0).unwrap()
                    + k4.data)
                    * sixth_dt,
            ..self.clone()
        }
    }
}