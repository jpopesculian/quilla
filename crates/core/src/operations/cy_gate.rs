use alloc::vec;
use core::ops::Neg;

use ndarray::array;
use num_complex::Complex;

use super::unitary_gate::UnitaryGate;
use crate::draw::{CircuitDrawing, ControlEnd, DrawOperation, DrawPosition};
use crate::num::{c32, c64};
use crate::state_vector::{StateVector, StateVectorOperation};

#[derive(Clone, Copy, Debug)]
pub struct CYGate {
    control: usize,
    target: usize,
}

impl CYGate {
    pub fn new(control: usize, target: usize) -> Self {
        Self { control, target }
    }
}

impl DrawOperation for CYGate {
    fn draw_to(&self, d: &mut CircuitDrawing) {
        d.push_box_with_control(
            DrawPosition::Qbit(self.target),
            "Y",
            DrawPosition::Qbit(self.control),
            ControlEnd::Circle,
        );
    }
}

impl<T> StateVectorOperation<T> for CYGate
where
    T: Copy + Neg<Output = T>,
{
    fn apply_to(&self, state: &mut StateVector<T>, _rng: &mut crate::rand::DynRng) {
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
    use crate::num::{assert_complex_close, c64};
    use crate::state_vector::StateVector;

    fn basis_state(qubits: usize, index: usize) -> StateVector<f64> {
        let mut state = StateVector::<f64>::new(qubits, 0);
        state.qstate[0] = c64(0.0, 0.0);
        state.qstate[index] = c64(1.0, 0.0);
        state
    }

    #[test]
    fn cy_applies_y_when_control_is_one() {
        let mut state = basis_state(2, 2);
        let gate = CYGate::new(1, 0);
        let mut rng = crate::rand::rng();

        gate.apply_to(&mut state, &mut rng);

        assert_complex_close(state.qstate[2], c64(0.0, 0.0));
        assert_complex_close(state.qstate[3], c64(0.0, 1.0));
    }

    #[test]
    fn cy_does_not_apply_when_control_is_zero() {
        let mut state = basis_state(2, 1);
        let gate = CYGate::new(1, 0);
        let mut rng = crate::rand::rng();

        gate.apply_to(&mut state, &mut rng);

        assert_complex_close(state.qstate[1], c64(1.0, 0.0));
        assert_complex_close(state.qstate[0], c64(0.0, 0.0));
    }

    #[test]
    fn cy_gate_matches_unitary_apply() {
        let gate = CYGate::new(1, 0);

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
