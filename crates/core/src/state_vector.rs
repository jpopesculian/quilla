use alloc::boxed::Box;
use alloc::{vec, vec::Vec};

use bitvec::{bitvec, vec::BitVec};
use num_complex::Complex;
use num_traits::Num;

use crate::rand::DynRng;

pub trait StateVectorOperation<T> {
    fn apply_to(&self, state: &mut StateVector<T>, rng: &mut DynRng);
}

impl<T, U> StateVectorOperation<U> for &T
where
    T: StateVectorOperation<U>,
{
    fn apply_to(&self, state: &mut StateVector<U>, rng: &mut DynRng) {
        T::apply_to(self, state, rng);
    }
}

impl<T, U> StateVectorOperation<U> for Box<T>
where
    T: StateVectorOperation<U> + ?Sized,
{
    fn apply_to(&self, state: &mut StateVector<U>, rng: &mut DynRng) {
        T::apply_to(self, state, rng);
    }
}

#[derive(Debug, Clone)]
pub struct StateVector<T> {
    pub(crate) qbits: usize,
    pub(crate) qstate: Vec<Complex<T>>,
    pub(crate) cbits: usize,
    pub(crate) cstate: BitVec,
}

impl<T> StateVector<T> {
    pub fn new(qbits: usize, cbits: usize) -> Self
    where
        T: Clone + Num,
    {
        let mut qstate = vec![Complex::new(T::zero(), T::zero()); 1 << qbits];
        if qbits > 0 {
            qstate[0] = Complex::new(T::one(), T::zero());
        }
        Self {
            qbits,
            qstate,
            cbits,
            cstate: bitvec![0; cbits],
        }
    }

    pub fn apply<O>(&mut self, op: O, rng: &mut DynRng)
    where
        O: StateVectorOperation<T>,
    {
        op.apply_to(self, rng);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitvec::bitvec;
    use bitvec::prelude::Lsb0;

    use crate::circuit::Circuit;

    #[derive(Clone, Copy)]
    struct SetCBitOp {
        cbit: usize,
        value: bool,
    }

    impl SetCBitOp {
        fn new(cbit: usize, value: bool) -> Self {
            Self { cbit, value }
        }
    }

    impl<T> StateVectorOperation<T> for SetCBitOp {
        fn apply_to(&self, state: &mut StateVector<T>, _rng: &mut DynRng) {
            state.cstate.set(self.cbit, self.value);
        }
    }

    #[test]
    fn sample_once_with_rng_applies_operations_in_order() {
        let mut circuit = Circuit::new(1, 1);
        circuit.op(SetCBitOp::new(0, false));
        circuit.op(SetCBitOp::new(0, true));

        let mut rng = crate::rand::rng();
        let result = circuit.sample_once_with_rng::<f64>(&mut rng);

        assert_eq!(result, bitvec![1]);
    }

    #[test]
    fn sample_once_returns_classical_state() {
        let mut circuit = Circuit::new(1, 1);
        circuit.op(SetCBitOp::new(0, true));

        let result = circuit.sample_once::<f64>();

        assert_eq!(result, bitvec![1]);
    }

    #[test]
    fn sample_with_rng_accumulates_shot_counts() {
        let mut circuit = Circuit::new(1, 1);
        circuit.op(SetCBitOp::new(0, true));

        let mut rng = crate::rand::rng();
        let results = circuit.sample_with_rng::<f64>(5, &mut rng);

        let expected = bitvec![1];
        assert_eq!(results.len(), 1);
        assert_eq!(results.get(&expected), Some(&5));
    }

    #[test]
    fn sample_returns_empty_map_for_zero_shots() {
        let mut circuit = Circuit::new(1, 1);
        circuit.op(SetCBitOp::new(0, true));

        let results = circuit.sample::<f64>(0);

        assert!(results.is_empty());
    }

    #[test]
    fn bell_state_has_only_01_and_10_outcomes() {
        let mut circuit = crate::circuit::DynCircuit::new(2, 2);
        circuit.x(1).h(0).cx(0, 1).meas(0, 0).meas(1, 1);

        let results = circuit.sample::<f64>(1000);

        let b00 = bitvec![0, 0];
        let b01 = bitvec![1, 0];
        let b10 = bitvec![0, 1];
        let b11 = bitvec![1, 1];

        assert_eq!(results.values().sum::<usize>(), 1000);
        assert_eq!(results.get(&b00).copied().unwrap_or(0), 0);
        assert_eq!(results.get(&b11).copied().unwrap_or(0), 0);
        assert!(results.get(&b01).copied().unwrap_or(0) > 0);
        assert!(results.get(&b10).copied().unwrap_or(0) > 0);
    }
}
