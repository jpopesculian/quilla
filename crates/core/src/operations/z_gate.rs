use alloc::vec;
use core::ops::Neg;

use num_traits::Num;

use super::unitary_gate::UnitaryGate;
use crate::draw::{CircuitDrawing, DrawOperation, DrawPosition};
use crate::num::{c32, c64};
use crate::state_vector::{StateVector, StateVectorOperation};

#[derive(Clone, Copy, Debug)]
pub struct ZGate {
    target: usize,
}

impl ZGate {
    pub fn new(target: usize) -> Self {
        Self { target }
    }
}

impl DrawOperation for ZGate {
    fn draw_to(&self, d: &mut CircuitDrawing) {
        d.push_box(DrawPosition::Qbit(self.target), "Z");
    }
}

impl<T> StateVectorOperation<T> for ZGate
where
    T: Num + Copy + Neg<Output = T>,
{
    fn apply_to(&self, state: &mut StateVector<T>, _rng: &mut crate::rand::DynRng) {
        let bit = 1usize << self.target;

        for i in 0..state.qstate.len() {
            if (i & bit) != 0 {
                state.qstate[i] = -state.qstate[i];
            }
        }
    }
}

impl From<ZGate> for UnitaryGate<f32, 1> {
    fn from(gate: ZGate) -> Self {
        UnitaryGate::new(
            vec![c32(1., 0.), c32(0., 0.), c32(0., 0.), c32(-1., 0.)],
            [gate.target],
        )
    }
}

impl From<ZGate> for UnitaryGate<f64, 1> {
    fn from(gate: ZGate) -> Self {
        UnitaryGate::new(
            vec![c64(1., 0.), c64(0., 0.), c64(0., 0.), c64(-1., 0.)],
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
    fn z_gate_flips_phase_of_one_state() {
        let mut state = StateVector::<f64>::new(1, 0);
        state.qstate[0] = c64(0.0, 0.0);
        state.qstate[1] = c64(1.0, 0.0);
        let gate = ZGate::new(0);
        let mut rng = crate::rand::rng();

        gate.apply_to(&mut state, &mut rng);

        assert_complex_close(state.qstate[0], c64(0.0, 0.0));
        assert_complex_close(state.qstate[1], c64(-1.0, 0.0));
    }

    #[test]
    fn z_gate_matches_unitary_apply() {
        let gate = ZGate::new(0);

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
