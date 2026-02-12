use alloc::vec;
use core::ops::Neg;

use ndarray::array;
use num_complex::Complex;

use super::unitary_gate::UnitaryGate;
use crate::complex::{c32, c64};
use crate::state_vector::{StateVector, StateVectorOperation};

#[derive(Clone, Copy)]
pub struct CYGate {
    control: usize,
    target: usize,
}

impl CYGate {
    pub fn new(control: usize, target: usize) -> Self {
        Self { control, target }
    }
}

impl<T> StateVectorOperation<T> for CYGate
where
    T: Copy + Neg<Output = T>,
{
    fn apply<R>(&self, state: &mut StateVector<T>, _rng: &mut R)
    where
        R: rand::Rng + ?Sized,
    {
        let control_mask = 1usize << self.control;
        let target_mask = 1usize << self.target;

        for i0 in 0..state.qstate.len() {
            if (i0 & control_mask) == 0 || (i0 & target_mask) != 0 {
                continue;
            }

            let i1 = i0 | target_mask;
            let a0 = state.qstate[i0];
            let a1 = state.qstate[i1];

            state.qstate[i0] = Complex::new(a1.im, -a1.re);
            state.qstate[i1] = Complex::new(-a0.im, a0.re);
        }
    }
}

impl From<CYGate> for UnitaryGate<f32, 2> {
    fn from(gate: CYGate) -> Self {
        UnitaryGate::new(
            array![
                [c32(1., 0.), c32(0., 0.), c32(0., 0.), c32(0., 0.)],
                [c32(0., 0.), c32(1., 0.), c32(0., 0.), c32(0., 0.)],
                [c32(0., 0.), c32(0., 0.), c32(0., 0.), c32(0., -1.)],
                [c32(0., 0.), c32(0., 0.), c32(0., 1.), c32(0., 0.)],
            ],
            [gate.control, gate.target],
        )
    }
}

impl From<CYGate> for UnitaryGate<f64, 2> {
    fn from(gate: CYGate) -> Self {
        UnitaryGate::new(
            array![
                [c64(1., 0.), c64(0., 0.), c64(0., 0.), c64(0., 0.)],
                [c64(0., 0.), c64(1., 0.), c64(0., 0.), c64(0., 0.)],
                [c64(0., 0.), c64(0., 0.), c64(0., 0.), c64(0., -1.)],
                [c64(0., 0.), c64(0., 0.), c64(0., 1.), c64(0., 0.)],
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
    fn cy_applies_y_when_control_is_one() {
        let mut state = basis_state(2, 2);
        let gate = CYGate::new(1, 0);
        let mut rng = crate::rand::rng();

        gate.apply(&mut state, &mut rng);

        assert_complex_close(state.qstate[2], c64(0.0, 0.0));
        assert_complex_close(state.qstate[3], c64(0.0, 1.0));
    }

    #[test]
    fn cy_does_not_apply_when_control_is_zero() {
        let mut state = basis_state(2, 1);
        let gate = CYGate::new(1, 0);
        let mut rng = crate::rand::rng();

        gate.apply(&mut state, &mut rng);

        assert_complex_close(state.qstate[1], c64(1.0, 0.0));
        assert_complex_close(state.qstate[0], c64(0.0, 0.0));
    }
}
