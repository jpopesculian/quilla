use alloc::vec;
use num_complex::Complex;
use num_traits::Float;

use ndarray::array;

use super::unitary_gate::UnitaryGate;
use crate::complex::{c32, c64};
use crate::state_vector::{StateVector, StateVectorOperation};

#[derive(Clone, Copy)]
pub struct TDaggerGate {
    target: usize,
}

impl TDaggerGate {
    pub fn new(target: usize) -> Self {
        Self { target }
    }
}

impl<T> StateVectorOperation<T> for TDaggerGate
where
    T: Float + Copy,
{
    fn apply_to(&self, state: &mut StateVector<T>, _rng: &mut crate::rand::DynRng) {
        let bit = 1usize << self.target;
        let one = T::one();
        let inv_sqrt_2 = one / (one + one).sqrt();
        let phase = Complex::new(inv_sqrt_2, -inv_sqrt_2);

        for i in 0..state.qstate.len() {
            if (i & bit) != 0 {
                state.qstate[i] = state.qstate[i] * phase;
            }
        }
    }
}

impl From<TDaggerGate> for UnitaryGate<f32, 1> {
    fn from(gate: TDaggerGate) -> Self {
        let phase = core::f32::consts::FRAC_1_SQRT_2;
        UnitaryGate::new(
            array![
                [c32(1., 0.), c32(0., 0.)],
                [c32(0., 0.), c32(phase, -phase)]
            ],
            [gate.target],
        )
    }
}

impl From<TDaggerGate> for UnitaryGate<f64, 1> {
    fn from(gate: TDaggerGate) -> Self {
        let phase = core::f64::consts::FRAC_1_SQRT_2;
        UnitaryGate::new(
            array![
                [c64(1., 0.), c64(0., 0.)],
                [c64(0., 0.), c64(phase, -phase)]
            ],
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
    fn t_dagger_gate_maps_one_to_minus_pi_over_4_phase() {
        let mut state = StateVector::<f64>::new(1, 0);
        state.qstate[0] = c64(0.0, 0.0);
        state.qstate[1] = c64(1.0, 0.0);
        let gate = TDaggerGate::new(0);
        let mut rng = crate::rand::rng();

        gate.apply_to(&mut state, &mut rng);

        let phase = core::f64::consts::FRAC_1_SQRT_2;
        assert_complex_close(state.qstate[0], c64(0.0, 0.0));
        assert_complex_close(state.qstate[1], c64(phase, -phase));
    }

    #[test]
    fn t_dagger_gate_matches_unitary_apply() {
        let gate = TDaggerGate::new(0);

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
