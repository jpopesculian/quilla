use alloc::vec;
use num_traits::Num;

use super::unitary_gate::UnitaryGate;
use crate::draw::{CircuitDrawing, DrawOperation, DrawPosition};
use crate::num::{c32, c64};
use crate::state_vector::{StateVector, StateVectorOperation};

#[derive(Clone, Copy, Debug)]
pub struct HadamardGate {
    target: usize,
}

impl HadamardGate {
    pub fn new(target: usize) -> Self {
        Self { target }
    }
}

impl DrawOperation for HadamardGate {
    fn draw_to(&self, d: &mut CircuitDrawing) {
        d.push_box(DrawPosition::Qbit(self.target), "H");
    }
}

impl<T> StateVectorOperation<T> for HadamardGate
where
    T: Num + Copy,
    UnitaryGate<T, 1>: From<HadamardGate>,
{
    fn apply_to(&self, state: &mut StateVector<T>, rng: &mut crate::rand::DynRng) {
        UnitaryGate::<T, 1>::from(*self).apply_to(state, rng)
    }
}

impl From<HadamardGate> for UnitaryGate<f32, 1> {
    fn from(gate: HadamardGate) -> Self {
        let frac_1_sqrt_2 = core::f32::consts::FRAC_1_SQRT_2;
        UnitaryGate::new(
            vec![
                c32(frac_1_sqrt_2, 0.),
                c32(frac_1_sqrt_2, 0.),
                c32(frac_1_sqrt_2, 0.),
                c32(-frac_1_sqrt_2, 0.),
            ],
            [gate.target],
        )
    }
}

impl From<HadamardGate> for UnitaryGate<f64, 1> {
    fn from(gate: HadamardGate) -> Self {
        let frac_1_sqrt_2 = core::f64::consts::FRAC_1_SQRT_2;
        UnitaryGate::new(
            vec![
                c64(frac_1_sqrt_2, 0.),
                c64(frac_1_sqrt_2, 0.),
                c64(frac_1_sqrt_2, 0.),
                c64(-frac_1_sqrt_2, 0.),
            ],
            [gate.target],
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
    fn hadamard_on_zero_creates_equal_superposition() {
        let mut state = basis_state(1, 0);
        let gate = HadamardGate::new(0);
        let mut rng = crate::rand::rng();

        gate.apply_to(&mut state, &mut rng);

        let s = core::f64::consts::FRAC_1_SQRT_2;
        assert_complex_close(state.qstate[0], c64(s, 0.0));
        assert_complex_close(state.qstate[1], c64(s, 0.0));
    }
}
