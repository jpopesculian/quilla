use bitvec::{bitvec, vec::BitVec};
use ndarray::Array1;
use num_complex::Complex;
use num_traits::Num;

pub trait StateVectorOperation<T> {
    fn apply_to<R>(&self, state: &mut StateVector<T>, rng: &mut R)
    where
        R: rand::Rng + ?Sized;
}

impl<T, U> StateVectorOperation<U> for &T
where
    T: StateVectorOperation<U>,
{
    fn apply_to<R>(&self, state: &mut StateVector<U>, rng: &mut R)
    where
        R: rand::Rng + ?Sized,
    {
        T::apply_to(self, state, rng);
    }
}

pub struct StateVector<T> {
    pub(crate) qbits: usize,
    pub(crate) qstate: Array1<Complex<T>>,
    pub(crate) cbits: usize,
    pub(crate) cstate: BitVec,
}

impl<T> StateVector<T> {
    pub fn new(qbits: usize, cbits: usize) -> Self
    where
        T: Clone + Num,
    {
        Self {
            qbits,
            qstate: Array1::zeros(1 << qbits),
            cbits,
            cstate: bitvec![0; cbits],
        }
    }

    pub fn apply<O, R>(&mut self, op: O, rng: &mut R)
    where
        O: StateVectorOperation<T>,
        R: rand::Rng + ?Sized,
    {
        op.apply_to(self, rng);
    }

    pub fn apply_all<O, R>(&mut self, ops: impl IntoIterator<Item = O>, rng: &mut R)
    where
        O: StateVectorOperation<T>,
        R: rand::Rng + ?Sized,
    {
        for op in ops {
            op.apply_to(self, rng);
        }
    }
}
