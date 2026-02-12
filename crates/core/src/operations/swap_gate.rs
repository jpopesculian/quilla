use alloc::vec;
use ndarray::array;

use super::unitary_gate::UnitaryGate;
use crate::complex::{c32, c64};
use crate::state_vector::{StateVector, StateVectorOperation};

#[derive(Clone, Copy)]
pub struct SwapGate {
    first: usize,
    second: usize,
}

impl SwapGate {
    pub fn new(first: usize, second: usize) -> Self {
        Self { first, second }
    }
}

impl<T> StateVectorOperation<T> for SwapGate {
    fn apply<R>(&self, state: &mut StateVector<T>, _rng: &mut R)
    where
        R: rand::Rng + ?Sized,
    {
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
            array![
                [c32(1., 0.), c32(0., 0.), c32(0., 0.), c32(0., 0.)],
                [c32(0., 0.), c32(0., 0.), c32(1., 0.), c32(0., 0.)],
                [c32(0., 0.), c32(1., 0.), c32(0., 0.), c32(0., 0.)],
                [c32(0., 0.), c32(0., 0.), c32(0., 0.), c32(1., 0.)],
            ],
            [gate.first, gate.second],
        )
    }
}

impl From<SwapGate> for UnitaryGate<f64, 2> {
    fn from(gate: SwapGate) -> Self {
        UnitaryGate::new(
            array![
                [c64(1., 0.), c64(0., 0.), c64(0., 0.), c64(0., 0.)],
                [c64(0., 0.), c64(0., 0.), c64(1., 0.), c64(0., 0.)],
                [c64(0., 0.), c64(1., 0.), c64(0., 0.), c64(0., 0.)],
                [c64(0., 0.), c64(0., 0.), c64(0., 0.), c64(1., 0.)],
            ],
            [gate.first, gate.second],
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
    fn swap_exchanges_01_and_10() {
        let mut state = basis_state(2, 1);
        let gate = SwapGate::new(1, 0);
        let mut rng = crate::rand::rng();

        gate.apply(&mut state, &mut rng);

        assert_complex_close(state.qstate[1], c64(0.0, 0.0));
        assert_complex_close(state.qstate[2], c64(1.0, 0.0));
    }

    #[test]
    fn swap_keeps_11_unchanged() {
        let mut state = basis_state(2, 3);
        let gate = SwapGate::new(1, 0);
        let mut rng = crate::rand::rng();

        gate.apply(&mut state, &mut rng);

        assert_complex_close(state.qstate[3], c64(1.0, 0.0));
    }
}
