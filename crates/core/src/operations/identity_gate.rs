use alloc::vec;
use ndarray::array;

use super::unitary_gate::UnitaryGate;
use crate::complex::{c32, c64};
use crate::state_vector::{StateVector, StateVectorOperation};

#[derive(Clone, Copy)]
pub struct IdentityGate {
    target: usize,
}

impl IdentityGate {
    pub fn new(target: usize) -> Self {
        Self { target }
    }
}

impl<T> StateVectorOperation<T> for IdentityGate {
    fn apply<R>(&self, _state: &mut StateVector<T>, _rng: &mut R)
    where
        R: rand::Rng + ?Sized,
    {
    }
}

impl From<IdentityGate> for UnitaryGate<f32, 1> {
    fn from(gate: IdentityGate) -> Self {
        UnitaryGate::new(
            array![[c32(1., 0.), c32(0., 0.)], [c32(0., 0.), c32(1., 0.)]],
            [gate.target],
        )
    }
}

impl From<IdentityGate> for UnitaryGate<f64, 1> {
    fn from(gate: IdentityGate) -> Self {
        UnitaryGate::new(
            array![[c64(1., 0.), c64(0., 0.)], [c64(0., 0.), c64(1., 0.)]],
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
    fn identity_gate_leaves_state_unchanged() {
        let mut state = StateVector::<f64>::new(1, 0);
        state.qstate[0] = c64(0.3, -0.1);
        state.qstate[1] = c64(-0.4, 0.2);
        let original = state.qstate.clone();

        let gate = IdentityGate::new(0);
        let mut rng = crate::rand::rng();
        gate.apply(&mut state, &mut rng);

        assert_complex_close(state.qstate[0], original[0]);
        assert_complex_close(state.qstate[1], original[1]);
    }
}
