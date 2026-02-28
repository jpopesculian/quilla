use alloc::vec;

use super::unitary_gate::UnitaryGate;
use crate::draw::{CircuitDrawing, ControlEnd, DrawOperation, DrawPosition};
use crate::num::{c32, c64};
use crate::state_vector::{StateVector, StateVectorOperation};

#[derive(Clone, Copy, Debug)]
pub struct CXGate {
    control: usize,
    target: usize,
}

impl DrawOperation for CXGate {
    fn draw_to(&self, d: &mut CircuitDrawing) {
        d.push_box_with_control(
            DrawPosition::Qbit(self.target),
            "X",
            DrawPosition::Qbit(self.control),
            ControlEnd::Circle,
        );
    }
}

impl CXGate {
    pub fn new(control: usize, target: usize) -> Self {
        Self { control, target }
    }
}

impl<T> StateVectorOperation<T> for CXGate {
    fn apply_to(&self, state: &mut StateVector<T>, _rng: &mut crate::rand::DynRng) {
        let control_mask = 1usize << self.control;
        let target_mask = 1usize << self.target;

        for i in 0..state.qstate.len() {
            if (i & control_mask) == 0 || (i & target_mask) != 0 {
                continue;
            }

            state.qstate.swap(i, i | target_mask);
        }
    }
}

impl From<CXGate> for UnitaryGate<f32, 2> {
    fn from(gate: CXGate) -> Self {
        UnitaryGate::new(
            vec![
                c32(1., 0.),
                c32(0., 0.),
                c32(0., 0.),
                c32(0., 0.),
                c32(0., 0.),
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
            ],
            [gate.control, gate.target],
        )
    }
}

impl From<CXGate> for UnitaryGate<f64, 2> {
    fn from(gate: CXGate) -> Self {
        UnitaryGate::new(
            vec![
                c64(1., 0.),
                c64(0., 0.),
                c64(0., 0.),
                c64(0., 0.),
                c64(0., 0.),
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
            ],
            [gate.control, gate.target],
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
    fn cx_flips_target_when_control_is_one() {
        let mut state = basis_state(2, 2);
        let gate = CXGate::new(1, 0);
        let mut rng = crate::rand::default_rng();

        gate.apply_to(&mut state, &mut rng);

        assert_complex_close(state.qstate[2], c64(0.0, 0.0));
        assert_complex_close(state.qstate[3], c64(1.0, 0.0));
    }

    #[test]
    fn cx_does_not_flip_target_when_control_is_zero() {
        let mut state = basis_state(2, 1);
        let gate = CXGate::new(1, 0);
        let mut rng = crate::rand::default_rng();

        gate.apply_to(&mut state, &mut rng);

        assert_complex_close(state.qstate[1], c64(1.0, 0.0));
        assert_complex_close(state.qstate[0], c64(0.0, 0.0));
    }

    #[test]
    fn cx_gate_matches_unitary_apply() {
        let gate = CXGate::new(1, 0);

        let mut direct_state = StateVector::<f64>::new(2, 0);
        direct_state.qstate[0] = c64(0.1, 0.2);
        direct_state.qstate[1] = c64(-0.3, 0.4);
        direct_state.qstate[2] = c64(0.5, -0.6);
        direct_state.qstate[3] = c64(-0.7, -0.8);

        let mut unitary_state = StateVector::<f64>::new(2, 0);
        unitary_state.qstate = direct_state.qstate.clone();

        let mut rng = crate::rand::default_rng();
        gate.apply_to(&mut direct_state, &mut rng);

        let mut rng = crate::rand::default_rng();
        UnitaryGate::<f64, 2>::from(gate).apply_to(&mut unitary_state, &mut rng);

        for i in 0..direct_state.qstate.len() {
            assert_complex_close(direct_state.qstate[i], unitary_state.qstate[i]);
        }
    }
}
