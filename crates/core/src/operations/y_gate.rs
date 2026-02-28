use alloc::vec;
use core::ops::Neg;

use num_complex::Complex;

use super::unitary_gate::UnitaryGate;
use crate::draw::{CircuitDrawing, DrawOperation, DrawPosition};
use crate::num::{c32, c64};
use crate::state_vector::{StateVector, StateVectorOperation};

#[derive(Clone, Copy, Debug)]
pub struct YGate {
    target: usize,
}

impl YGate {
    pub fn new(target: usize) -> Self {
        Self { target }
    }
}

impl DrawOperation for YGate {
    fn draw_to(&self, d: &mut CircuitDrawing) {
        d.push_box(DrawPosition::Qbit(self.target), "Y");
    }
}

impl<T> StateVectorOperation<T> for YGate
where
    T: Copy + Neg<Output = T>,
{
    fn apply_to(&self, state: &mut StateVector<T>, _rng: &mut crate::rand::DynRng) {
        let bit = 1usize << self.target;

        for i0 in 0..state.qstate.len() {
            if (i0 & bit) != 0 {
                continue;
            }

            let i1 = i0 | bit;
            let a0 = state.qstate[i0];
            let a1 = state.qstate[i1];

            state.qstate[i0] = Complex::new(a1.im, -a1.re);
            state.qstate[i1] = Complex::new(-a0.im, a0.re);
        }
    }
}

impl From<YGate> for UnitaryGate<f32, 1> {
    fn from(gate: YGate) -> Self {
        UnitaryGate::new(
            vec![c32(0., 0.), c32(0., -1.), c32(0., 1.), c32(0., 0.)],
            [gate.target],
        )
    }
}

impl From<YGate> for UnitaryGate<f64, 1> {
    fn from(gate: YGate) -> Self {
        UnitaryGate::new(
            vec![c64(0., 0.), c64(0., -1.), c64(0., 1.), c64(0., 0.)],
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
    fn y_gate_maps_zero_to_i_one() {
        let mut state = StateVector::<f64>::new(1, 0);
        state.qstate[0] = c64(1.0, 0.0);
        let gate = YGate::new(0);
        let mut rng = crate::rand::default_rng();

        gate.apply_to(&mut state, &mut rng);

        assert_complex_close(state.qstate[0], c64(0.0, 0.0));
        assert_complex_close(state.qstate[1], c64(0.0, 1.0));
    }

    #[test]
    fn y_gate_matches_unitary_apply() {
        let gate = YGate::new(0);

        let mut direct_state = StateVector::<f64>::new(1, 0);
        direct_state.qstate[0] = c64(0.3, -0.1);
        direct_state.qstate[1] = c64(-0.4, 0.2);

        let mut unitary_state = StateVector::<f64>::new(1, 0);
        unitary_state.qstate = direct_state.qstate.clone();

        let mut rng = crate::rand::default_rng();
        gate.apply_to(&mut direct_state, &mut rng);

        let mut rng = crate::rand::default_rng();
        UnitaryGate::<f64, 1>::from(gate).apply_to(&mut unitary_state, &mut rng);

        for i in 0..direct_state.qstate.len() {
            assert_complex_close(direct_state.qstate[i], unitary_state.qstate[i]);
        }
    }
}
