use alloc::vec;
use core::ops::Neg;

use ndarray::array;
use num_traits::Num;

use super::unitary_gate::UnitaryGate;
use crate::complex::{c32, c64};
use crate::state_vector::{StateVector, StateVectorOperation};

#[derive(Clone, Copy)]
pub struct CZGate {
    control: usize,
    target: usize,
}

impl CZGate {
    pub fn new(control: usize, target: usize) -> Self {
        Self { control, target }
    }
}

impl<T> StateVectorOperation<T> for CZGate
where
    T: Num + Copy + Neg<Output = T>,
{
    fn apply<R>(&self, state: &mut StateVector<T>, _rng: &mut R)
    where
        R: rand::Rng + ?Sized,
    {
        let control_mask = 1usize << self.control;
        let target_mask = 1usize << self.target;

        for i in 0..state.qstate.len() {
            if (i & control_mask) != 0 && (i & target_mask) != 0 {
                state.qstate[i] = -state.qstate[i];
            }
        }
    }
}

impl From<CZGate> for UnitaryGate<f32, 2> {
    fn from(gate: CZGate) -> Self {
        UnitaryGate::new(
            array![
                [c32(1., 0.), c32(0., 0.), c32(0., 0.), c32(0., 0.)],
                [c32(0., 0.), c32(1., 0.), c32(0., 0.), c32(0., 0.)],
                [c32(0., 0.), c32(0., 0.), c32(1., 0.), c32(0., 0.)],
                [c32(0., 0.), c32(0., 0.), c32(0., 0.), c32(-1., 0.)],
            ],
            [gate.control, gate.target],
        )
    }
}

impl From<CZGate> for UnitaryGate<f64, 2> {
    fn from(gate: CZGate) -> Self {
        UnitaryGate::new(
            array![
                [c64(1., 0.), c64(0., 0.), c64(0., 0.), c64(0., 0.)],
                [c64(0., 0.), c64(1., 0.), c64(0., 0.), c64(0., 0.)],
                [c64(0., 0.), c64(0., 0.), c64(1., 0.), c64(0., 0.)],
                [c64(0., 0.), c64(0., 0.), c64(0., 0.), c64(-1., 0.)],
            ],
            [gate.control, gate.target],
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
    fn cz_flips_phase_of_11() {
        let mut state = basis_state(2, 3);
        let gate = CZGate::new(1, 0);
        let mut rng = crate::rand::rng();

        gate.apply(&mut state, &mut rng);

        assert_complex_close(state.qstate[3], c64(-1.0, 0.0));
    }

    #[test]
    fn cz_keeps_10_unchanged() {
        let mut state = basis_state(2, 2);
        let gate = CZGate::new(1, 0);
        let mut rng = crate::rand::rng();

        gate.apply(&mut state, &mut rng);

        assert_complex_close(state.qstate[2], c64(1.0, 0.0));
    }
}
