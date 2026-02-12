use alloc::vec;
use num_complex::Complex;
use num_traits::Float;

use ndarray::array;

use super::unitary_gate::UnitaryGate;
use crate::complex::{c32, c64};
use crate::state_vector::{StateVector, StateVectorOperation};

#[derive(Clone, Copy)]
pub struct TGate {
    target: usize,
}

impl TGate {
    pub fn new(target: usize) -> Self {
        Self { target }
    }
}

impl<T> StateVectorOperation<T> for TGate
where
    T: Float + Copy,
{
    fn apply<R>(&self, state: &mut StateVector<T>, _rng: &mut R)
    where
        R: rand::Rng + ?Sized,
    {
        let bit = 1usize << self.target;
        let one = T::one();
        let inv_sqrt_2 = one / (one + one).sqrt();
        let phase = Complex::new(inv_sqrt_2, inv_sqrt_2);

        for i in 0..state.qstate.len() {
            if (i & bit) != 0 {
                state.qstate[i] = state.qstate[i] * phase;
            }
        }
    }
}

impl From<TGate> for UnitaryGate<f32, 1> {
    fn from(gate: TGate) -> Self {
        let phase = core::f32::consts::FRAC_1_SQRT_2;
        UnitaryGate::new(
            array![[c32(1., 0.), c32(0., 0.)], [c32(0., 0.), c32(phase, phase)]],
            [gate.target],
        )
    }
}

impl From<TGate> for UnitaryGate<f64, 1> {
    fn from(gate: TGate) -> Self {
        let phase = core::f64::consts::FRAC_1_SQRT_2;
        UnitaryGate::new(
            array![[c64(1., 0.), c64(0., 0.)], [c64(0., 0.), c64(phase, phase)]],
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
    fn t_gate_maps_one_to_pi_over_4_phase() {
        let mut state = StateVector::<f64>::new(1, 0);
        state.qstate[1] = c64(1.0, 0.0);
        let gate = TGate::new(0);
        let mut rng = crate::rand::rng();

        gate.apply(&mut state, &mut rng);

        let phase = core::f64::consts::FRAC_1_SQRT_2;
        assert_complex_close(state.qstate[0], c64(0.0, 0.0));
        assert_complex_close(state.qstate[1], c64(phase, phase));
    }
}
