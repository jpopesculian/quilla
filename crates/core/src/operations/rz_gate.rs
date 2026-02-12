use alloc::vec;
use ndarray::array;
use num_traits::{Float, Num};

use super::unitary_gate::UnitaryGate;
use crate::complex::{c32, c64};
use crate::state_vector::{StateVector, StateVectorOperation};

#[derive(Clone, Copy)]
pub struct RZGate<T> {
    theta: T,
    target: usize,
}

impl<T> RZGate<T> {
    pub fn new(theta: T, target: usize) -> Self {
        Self { theta, target }
    }
}

impl<T> StateVectorOperation<T> for RZGate<T>
where
    T: Num + Copy,
    UnitaryGate<T, 1>: From<RZGate<T>>,
{
    fn apply<R>(&self, state: &mut StateVector<T>, rng: &mut R)
    where
        R: rand::Rng + ?Sized,
    {
        UnitaryGate::<T, 1>::from(*self).apply(state, rng)
    }
}

impl From<RZGate<f32>> for UnitaryGate<f32, 1> {
    fn from(gate: RZGate<f32>) -> Self {
        let half_theta = gate.theta / 2.0;
        let c = <f32 as Float>::cos(half_theta);
        let s = <f32 as Float>::sin(half_theta);

        UnitaryGate::new(
            array![[c32(c, -s), c32(0., 0.)], [c32(0., 0.), c32(c, s)]],
            [gate.target],
        )
    }
}

impl From<RZGate<f64>> for UnitaryGate<f64, 1> {
    fn from(gate: RZGate<f64>) -> Self {
        let half_theta = gate.theta / 2.0;
        let c = <f64 as Float>::cos(half_theta);
        let s = <f64 as Float>::sin(half_theta);

        UnitaryGate::new(
            array![[c64(c, -s), c64(0., 0.)], [c64(0., 0.), c64(c, s)]],
            [gate.target],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::complex::{assert_complex_close, c64};
    use crate::state_vector::StateVector;

    #[test]
    fn rz_pi_maps_one_to_i_one() {
        let mut state = StateVector::<f64>::new(1, 0);
        state.qstate[1] = c64(1.0, 0.0);
        let gate = RZGate::new(core::f64::consts::PI, 0);
        let mut rng = crate::rand::rng();

        gate.apply(&mut state, &mut rng);

        assert_complex_close(state.qstate[0], c64(0.0, 0.0));
        assert_complex_close(state.qstate[1], c64(0.0, 1.0));
    }
}
