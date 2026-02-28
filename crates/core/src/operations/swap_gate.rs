use alloc::vec;

use super::unitary_gate::UnitaryGate;
use crate::draw::{CircuitDrawing, ControlEnd, DrawOperation, DrawPosition};
use crate::num::{c32, c64};
use crate::state_vector::{StateVector, StateVectorOperation};

#[derive(Clone, Copy, Debug)]
pub struct SwapGate {
    first: usize,
    second: usize,
}

impl SwapGate {
    pub fn new(first: usize, second: usize) -> Self {
        Self { first, second }
    }
}

impl DrawOperation for SwapGate {
    fn draw_to(&self, d: &mut CircuitDrawing) {
        d.push_double_control(
            DrawPosition::Qbit(self.first),
            ControlEnd::Cross,
            DrawPosition::Qbit(self.second),
            ControlEnd::Cross,
        );
    }
}

impl<T> StateVectorOperation<T> for SwapGate {
    fn apply_to(&self, state: &mut StateVector<T>, _rng: &mut crate::rand::DynRng) {
        let bit0 = 1usize << self.first;
        let bit1 = 1usize << self.second;
        let mask = bit0 | bit1;

        for i in 0..state.qstate.len() {
            if (i & bit0) == 0 && (i & bit1) != 0 {
                state.qstate.swap(i, i ^ mask);
            }
        }
    }
}

impl From<SwapGate> for UnitaryGate<f32, 2> {
    fn from(gate: SwapGate) -> Self {
        UnitaryGate::new(
            vec![
                c32(1., 0.),
                c32(0., 0.),
                c32(0., 0.),
                c32(0., 0.),
                c32(0., 0.),
                c32(0., 0.),
                c32(1., 0.),
                c32(0., 0.),
                c32(0., 0.),
                c32(1., 0.),
                c32(0., 0.),
                c32(0., 0.),
                c32(0., 0.),
                c32(0., 0.),
                c32(0., 0.),
                c32(1., 0.),
            ],
            [gate.first, gate.second],
        )
    }
}

impl From<SwapGate> for UnitaryGate<f64, 2> {
    fn from(gate: SwapGate) -> Self {
        UnitaryGate::new(
            vec![
                c64(1., 0.),
                c64(0., 0.),
                c64(0., 0.),
                c64(0., 0.),
                c64(0., 0.),
                c64(0., 0.),
                c64(1., 0.),
                c64(0., 0.),
                c64(0., 0.),
                c64(1., 0.),
                c64(0., 0.),
                c64(0., 0.),
                c64(0., 0.),
                c64(0., 0.),
                c64(0., 0.),
                c64(1., 0.),
            ],
            [gate.first, gate.second],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::num::{assert_complex_close, c64};
    use crate::state_vector::StateVector;

    fn basis_state(qubits: usize, index: usize) -> StateVector<f64> {
        let mut state = StateVector::<f64>::new(qubits, 0);
        state.qstate[0] = c64(0.0, 0.0);
        state.qstate[index] = c64(1.0, 0.0);
        state
    }

    #[test]
    fn swap_exchanges_01_and_10() {
        let mut state = basis_state(2, 1);
        let gate = SwapGate::new(1, 0);
        let mut rng = crate::rand::rng();

        gate.apply_to(&mut state, &mut rng);

        assert_complex_close(state.qstate[1], c64(0.0, 0.0));
        assert_complex_close(state.qstate[2], c64(1.0, 0.0));
    }

    #[test]
    fn swap_keeps_11_unchanged() {
        let mut state = basis_state(2, 3);
        let gate = SwapGate::new(1, 0);
        let mut rng = crate::rand::rng();

        gate.apply_to(&mut state, &mut rng);

        assert_complex_close(state.qstate[3], c64(1.0, 0.0));
    }

    #[test]
    fn swap_gate_matches_unitary_apply() {
        let gate = SwapGate::new(1, 0);

        let mut direct_state = StateVector::<f64>::new(2, 0);
        direct_state.qstate[0] = c64(0.1, 0.2);
        direct_state.qstate[1] = c64(-0.3, 0.4);
        direct_state.qstate[2] = c64(0.5, -0.6);
        direct_state.qstate[3] = c64(-0.7, -0.8);

        let mut unitary_state = StateVector::<f64>::new(2, 0);
        unitary_state.qstate = direct_state.qstate.clone();

        let mut rng = crate::rand::rng();
        gate.apply_to(&mut direct_state, &mut rng);

        let mut rng = crate::rand::rng();
        UnitaryGate::<f64, 2>::from(gate).apply_to(&mut unitary_state, &mut rng);

        for i in 0..direct_state.qstate.len() {
            assert_complex_close(direct_state.qstate[i], unitary_state.qstate[i]);
        }
    }
}
