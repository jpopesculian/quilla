use alloc::vec;
use ndarray::array;

use super::unitary_gate::UnitaryGate;
use crate::complex::{c32, c64};
use crate::state_vector::{StateVector, StateVectorOperation};

#[derive(Clone, Copy)]
pub struct XGate {
    target: usize,
}

impl XGate {
    pub fn new(target: usize) -> Self {
        Self { target }
    }
}

impl<T> StateVectorOperation<T> for XGate {
    fn apply<R>(&self, state: &mut StateVector<T>, _rng: &mut R)
    where
        R: rand::Rng + ?Sized,
    {
        let bit = 1usize << self.target;

        for i0 in 0..state.qstate.len() {
            if (i0 & bit) != 0 {
                continue;
            }

            state.qstate.swap(i0, i0 | bit);
        }
    }
}

impl From<XGate> for UnitaryGate<f32, 1> {
    fn from(gate: XGate) -> Self {
        UnitaryGate::new(
            array![[c32(0., 0.), c32(1., 0.)], [c32(1., 0.), c32(0., 0.)]],
            [gate.target],
        )
    }
}

impl From<XGate> for UnitaryGate<f64, 1> {
    fn from(gate: XGate) -> Self {
        UnitaryGate::new(
            array![[c64(0., 0.), c64(1., 0.)], [c64(1., 0.), c64(0., 0.)]],
            [gate.target],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::complex::{assert_complex_close, c64};
    use crate::state_vector::StateVector;

    fn basis_state(qubits: usize, index: usize) -> StateVector<f64> {
        let mut state = StateVector::<f64>::new(qubits, 0);
        state.qstate[index] = c64(1.0, 0.0);
        state
    }

    #[test]
    fn x_gate_flips_zero_to_one() {
        let mut state = basis_state(1, 0);
        let gate = XGate::new(0);
        let mut rng = crate::rand::rng();

        gate.apply(&mut state, &mut rng);

        assert_complex_close(state.qstate[0], c64(0.0, 0.0));
        assert_complex_close(state.qstate[1], c64(1.0, 0.0));
    }
}
