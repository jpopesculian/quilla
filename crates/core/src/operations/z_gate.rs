use alloc::vec;
use core::ops::Neg;

use ndarray::array;
use num_traits::Num;

use super::unitary_gate::UnitaryGate;
use crate::complex::{c32, c64};
use crate::state_vector::{StateVector, StateVectorOperation};

#[derive(Clone, Copy)]
pub struct ZGate {
    target: usize,
}

impl ZGate {
    pub fn new(target: usize) -> Self {
        Self { target }
    }
}

impl<T> StateVectorOperation<T> for ZGate
where
    T: Num + Copy + Neg<Output = T>,
{
    fn apply<R>(&self, state: &mut StateVector<T>, _rng: &mut R)
    where
        R: rand::Rng + ?Sized,
    {
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
            array![[c32(1., 0.), c32(0., 0.)], [c32(0., 0.), c32(-1., 0.)],],
            [gate.target],
        )
    }
}

impl From<ZGate> for UnitaryGate<f64, 1> {
    fn from(gate: ZGate) -> Self {
        UnitaryGate::new(
            array![[c64(1., 0.), c64(0., 0.)], [c64(0., 0.), c64(-1., 0.)],],
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
    fn z_gate_flips_phase_of_one_state() {
        let mut state = StateVector::<f64>::new(1, 0);
        state.qstate[1] = c64(1.0, 0.0);
        let gate = ZGate::new(0);
        let mut rng = crate::rand::rng();

        gate.apply(&mut state, &mut rng);

        assert_complex_close(state.qstate[0], c64(0.0, 0.0));
        assert_complex_close(state.qstate[1], c64(-1.0, 0.0));
    }
}
