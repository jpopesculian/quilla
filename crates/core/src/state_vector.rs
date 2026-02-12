use bitvec::{bitvec, vec::BitVec};
use ndarray::Array1;
use num_complex::Complex;
use num_traits::Num;

pub trait StateVectorOperation<T> {
    fn apply<R>(&self, state: &mut StateVector<T>, rng: &mut R)
    where
        R: rand::Rng + ?Sized;
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
}
