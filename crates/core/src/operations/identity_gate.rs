use alloc::vec;

use super::unitary_gate::UnitaryGate;
use crate::draw::{CircuitDrawing, DrawOperation, DrawPosition};
use crate::num::{c32, c64};
use crate::state_vector::{StateVector, StateVectorOperation};

#[derive(Clone, Copy, Debug)]
pub struct IdentityGate {
    target: usize,
}

impl IdentityGate {
    pub fn new(target: usize) -> Self {
        Self { target }
    }
}

impl DrawOperation for IdentityGate {
    fn draw_to(&self, d: &mut CircuitDrawing) {
        d.push_box(DrawPosition::Qbit(self.target), "I");
    }
}

impl<T> StateVectorOperation<T> for IdentityGate {
    fn apply_to(&self, _state: &mut StateVector<T>, _rng: &mut crate::rand::DynRng) {}
}

impl From<IdentityGate> for UnitaryGate<f32, 1> {
    fn from(gate: IdentityGate) -> Self {
        UnitaryGate::new(
            vec![c32(1., 0.), c32(0., 0.), c32(0., 0.), c32(1., 0.)],
            [gate.target],
        )
    }
}

impl From<IdentityGate> for UnitaryGate<f64, 1> {
    fn from(gate: IdentityGate) -> Self {
        UnitaryGate::new(
            vec![c64(1., 0.), c64(0., 0.), c64(0., 0.), c64(1., 0.)],
            [gate.target],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::num::{assert_complex_close, c64};
    use crate::state_vector::StateVector;

    #[test]
    fn identity_gate_leaves_state_unchanged() {
        let mut state = StateVector::<f64>::new(1, 0);
        state.qstate[0] = c64(0.3, -0.1);
        state.qstate[1] = c64(-0.4, 0.2);
        let original = state.qstate.clone();

        let gate = IdentityGate::new(0);
        let mut rng = crate::rand::rng();
        gate.apply_to(&mut state, &mut rng);

        assert_complex_close(state.qstate[0], original[0]);
        assert_complex_close(state.qstate[1], original[1]);
    }

    #[test]
    fn identity_gate_matches_unitary_apply() {
        let gate = IdentityGate::new(0);

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
