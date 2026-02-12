use num_complex::Complex;
use num_traits::Float;
use rand::{
    RngExt,
    distr::{Distribution, StandardUniform},
};

use crate::state_vector::{StateVector, StateVectorOperation};

#[derive(Clone, Copy)]
pub struct Measure {
    qbit: usize,
    cbit: usize,
}

impl Measure {
    pub fn new(qbit: usize, cbit: usize) -> Self {
        Self { qbit, cbit }
    }
}

impl<T> StateVectorOperation<T> for Measure
where
    T: Float,
    StandardUniform: Distribution<T>,
{
    fn apply_to(&self, state: &mut StateVector<T>, rng: &mut crate::rand::DynRng) {
        assert!(self.qbit < state.qbits, "qbit {} out of range", self.qbit);
        assert!(self.cbit < state.cbits, "cbit {} out of range", self.cbit);

        let qmask = 1usize << self.qbit;

        let mut p0 = T::zero();
        for (index, amplitude) in state.qstate.iter().enumerate() {
            if (index & qmask) == 0 {
                p0 = p0 + amplitude.norm_sqr();
            }
        }

        let sample = rng.random::<T>();
        let measured_one = sample >= p0;
        let measured_prob = if measured_one { T::one() - p0 } else { p0 };
        let inv_norm = if measured_prob > T::zero() {
            T::one() / measured_prob.sqrt()
        } else {
            T::zero()
        };
        let scale = Complex::new(inv_norm, T::zero());

        for (index, amplitude) in state.qstate.iter_mut().enumerate() {
            let bit_is_one = (index & qmask) != 0;
            if bit_is_one == measured_one {
                *amplitude = *amplitude * scale;
            } else {
                *amplitude = Complex::new(T::zero(), T::zero());
            }
        }

        state.cstate.set(self.cbit, measured_one);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::complex::{assert_complex_close, c64};
    use crate::state_vector::StateVector;

    #[test]
    fn measure_zero_sets_classical_zero_and_keeps_zero_state() {
        let mut state = StateVector::<f64>::new(1, 1);
        state.qstate[0] = c64(1.0, 0.0);
        let gate = Measure::new(0, 0);
        let mut rng = crate::rand::rng();

        gate.apply_to(&mut state, &mut rng);

        assert!(!state.cstate[0]);
        assert_complex_close(state.qstate[0], c64(1.0, 0.0));
        assert_complex_close(state.qstate[1], c64(0.0, 0.0));
    }

    #[test]
    fn measure_one_sets_classical_one_and_keeps_one_state() {
        let mut state = StateVector::<f64>::new(1, 1);
        state.qstate[0] = c64(0.0, 0.0);
        state.qstate[1] = c64(1.0, 0.0);
        let gate = Measure::new(0, 0);
        let mut rng = crate::rand::rng();

        gate.apply_to(&mut state, &mut rng);

        assert!(state.cstate[0]);
        assert_complex_close(state.qstate[0], c64(0.0, 0.0));
        assert_complex_close(state.qstate[1], c64(1.0, 0.0));
    }
}
