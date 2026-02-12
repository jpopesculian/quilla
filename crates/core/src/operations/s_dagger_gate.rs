use alloc::vec;
use core::ops::Neg;

use ndarray::array;

use super::unitary_gate::UnitaryGate;
use crate::complex::{c32, c64};
use crate::state_vector::{StateVector, StateVectorOperation};

#[derive(Clone, Copy)]
pub struct SDaggerGate {
    target: usize,
}

impl SDaggerGate {
    pub fn new(target: usize) -> Self {
        Self { target }
    }
}

impl<T> StateVectorOperation<T> for SDaggerGate
where
    T: Copy + Neg<Output = T>,
{
    fn apply_to<R>(&self, state: &mut StateVector<T>, _rng: &mut R)
    where
        R: rand::Rng + ?Sized,
    {
        let bit = 1usize << self.target;

        for i in 0..state.qstate.len() {
            if (i & bit) != 0 {
                let a = state.qstate[i];
                state.qstate[i] = num_complex::Complex::new(a.im, -a.re);
            }
        }
    }
}

impl From<SDaggerGate> for UnitaryGate<f32, 1> {
    fn from(gate: SDaggerGate) -> Self {
        UnitaryGate::new(
            array![[c32(1., 0.), c32(0., 0.)], [c32(0., 0.), c32(0., -1.)]],
            [gate.target],
        )
    }
}

impl From<SDaggerGate> for UnitaryGate<f64, 1> {
    fn from(gate: SDaggerGate) -> Self {
        UnitaryGate::new(
            array![[c64(1., 0.), c64(0., 0.)], [c64(0., 0.), c64(0., -1.)]],
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
    fn s_dagger_gate_maps_one_to_minus_i_one() {
        let mut state = StateVector::<f64>::new(1, 0);
        state.qstate[1] = c64(1.0, 0.0);
        let gate = SDaggerGate::new(0);
        let mut rng = crate::rand::rng();

        gate.apply_to(&mut state, &mut rng);

        assert_complex_close(state.qstate[0], c64(0.0, 0.0));
        assert_complex_close(state.qstate[1], c64(0.0, -1.0));
    }

    #[test]
    fn s_dagger_gate_matches_unitary_apply() {
        let gate = SDaggerGate::new(0);

        let mut direct_state = StateVector::<f64>::new(1, 0);
        direct_state.qstate[0] = c64(0.3, -0.1);
        direct_state.qstate[1] = c64(-0.4, 0.2);

        let mut unitary_state = StateVector::<f64>::new(1, 0);
        unitary_state.qstate = direct_state.qstate.clone();

        let mut rng = crate::rand::rng();
        gate.apply_to(&mut direct_state, &mut rng);

        let mut rng = crate::rand::rng();
        UnitaryGate::<f64, 1>::from(gate).apply_to(&mut unitary_state, &mut rng);

        for i in 0..direct_state.qstate.len() {
            assert_complex_close(direct_state.qstate[i], unitary_state.qstate[i]);
        }
    }
}
