use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::fmt;

use bitvec::vec::BitVec;
use num_traits::Num;

use crate::draw::{CircuitDrawing, DrawOperation};
use crate::operations::Operation;
use crate::rand::DynRng;
use crate::state_vector::{StateVector, StateVectorOperation};

#[derive(Debug, Clone)]
pub struct Circuit<O> {
    qbits: usize,
    cbits: usize,
    operations: Vec<O>,
}

impl<O> Circuit<O> {
    pub fn new(qbits: usize, cbits: usize) -> Self {
        Self {
            qbits,
            cbits,
            operations: Vec::new(),
        }
    }

    pub fn op(&mut self, op: O) {
        self.operations.push(op);
    }

    pub fn qbits(&self) -> usize {
        self.qbits
    }

    pub fn cbits(&self) -> usize {
        self.cbits
    }

    pub fn operations(&self) -> &[O] {
        &self.operations
    }
}

pub type DynCircuit = Circuit<Box<dyn Operation>>;

impl DynCircuit {
    pub fn i(&mut self, target: usize) -> &mut Self {
        self.op(Box::new(crate::operations::IdentityGate::new(target)));
        self
    }

    pub fn h(&mut self, target: usize) -> &mut Self {
        self.op(Box::new(crate::operations::HadamardGate::new(target)));
        self
    }

    pub fn x(&mut self, target: usize) -> &mut Self {
        self.op(Box::new(crate::operations::XGate::new(target)));
        self
    }

    pub fn y(&mut self, target: usize) -> &mut Self {
        self.op(Box::new(crate::operations::YGate::new(target)));
        self
    }

    pub fn z(&mut self, target: usize) -> &mut Self {
        self.op(Box::new(crate::operations::ZGate::new(target)));
        self
    }

    pub fn s(&mut self, target: usize) -> &mut Self {
        self.op(Box::new(crate::operations::SGate::new(target)));
        self
    }

    pub fn sdg(&mut self, target: usize) -> &mut Self {
        self.op(Box::new(crate::operations::SDaggerGate::new(target)));
        self
    }

    pub fn t(&mut self, target: usize) -> &mut Self {
        self.op(Box::new(crate::operations::TGate::new(target)));
        self
    }

    pub fn tdg(&mut self, target: usize) -> &mut Self {
        self.op(Box::new(crate::operations::TDaggerGate::new(target)));
        self
    }

    pub fn cx(&mut self, control: usize, target: usize) -> &mut Self {
        self.op(Box::new(crate::operations::CXGate::new(control, target)));
        self
    }

    pub fn cy(&mut self, control: usize, target: usize) -> &mut Self {
        self.op(Box::new(crate::operations::CYGate::new(control, target)));
        self
    }

    pub fn cz(&mut self, control: usize, target: usize) -> &mut Self {
        self.op(Box::new(crate::operations::CZGate::new(control, target)));
        self
    }

    pub fn rx(&mut self, theta: f64, target: usize) -> &mut Self {
        self.op(Box::new(crate::operations::RXGate::new(theta, target)));
        self
    }

    pub fn ry(&mut self, theta: f64, target: usize) -> &mut Self {
        self.op(Box::new(crate::operations::RYGate::new(theta, target)));
        self
    }

    pub fn rz(&mut self, theta: f64, target: usize) -> &mut Self {
        self.op(Box::new(crate::operations::RZGate::new(theta, target)));
        self
    }

    pub fn swap(&mut self, first: usize, second: usize) -> &mut Self {
        self.op(Box::new(crate::operations::SwapGate::new(first, second)));
        self
    }

    pub fn meas(&mut self, qbit: usize, cbit: usize) -> &mut Self {
        self.op(Box::new(crate::operations::Measure::new(qbit, cbit)));
        self
    }
}

impl<O, T> StateVectorOperation<T> for Circuit<O>
where
    O: StateVectorOperation<T>,
{
    fn apply_to(&self, state: &mut StateVector<T>, rng: &mut DynRng) {
        for op in self.operations() {
            op.apply_to(state, rng);
        }
    }
}

impl<O> Circuit<O> {
    pub fn sample_once_with_rng<T>(&self, rng: &mut DynRng) -> BitVec
    where
        O: StateVectorOperation<T>,
        T: Clone + Num,
    {
        let mut state = StateVector::<T>::new(self.qbits(), self.cbits());
        state.apply(self, rng);
        state.cstate
    }

    pub fn sample_once<T>(&self) -> BitVec
    where
        O: StateVectorOperation<T>,
        T: Clone + Num,
    {
        let mut rng = crate::rand::default_rng();
        self.sample_once_with_rng(&mut rng)
    }

    pub fn sample_with_rng<T>(&self, shots: usize, rng: &mut DynRng) -> BTreeMap<BitVec, usize>
    where
        O: StateVectorOperation<T>,
        T: Clone + Num,
    {
        let mut results = BTreeMap::new();
        for _ in 0..shots {
            let result = self.sample_once_with_rng(rng);
            let count = results.entry(result).or_insert(0);
            *count += 1
        }
        results
    }

    pub fn sample<T>(&self, shots: usize) -> BTreeMap<BitVec, usize>
    where
        O: StateVectorOperation<T>,
        T: Clone + Num,
    {
        let mut rng = crate::rand::default_rng();
        self.sample_with_rng(shots, &mut rng)
    }
}

impl<O> DrawOperation for Circuit<O>
where
    O: DrawOperation,
{
    fn draw_to(&self, d: &mut CircuitDrawing) {
        for op in self.operations() {
            op.draw_to(d);
        }
    }
}

impl<O> fmt::Display for Circuit<O>
where
    O: DrawOperation,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = CircuitDrawing::new(self.qbits(), self.cbits());
        d.draw(self);
        d.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dyn_circuit_gate_helpers_append_operations() {
        let mut circuit = DynCircuit::new(2, 1);

        circuit
            .i(0)
            .h(0)
            .x(0)
            .y(0)
            .z(0)
            .s(0)
            .sdg(0)
            .t(0)
            .tdg(0)
            .cx(1, 0)
            .cy(1, 0)
            .cz(1, 0)
            .rx(0.0, 0)
            .ry(0.0, 0)
            .rz(0.0, 0)
            .swap(0, 1)
            .meas(0, 0);

        assert_eq!(circuit.operations.len(), 17);
    }
}
