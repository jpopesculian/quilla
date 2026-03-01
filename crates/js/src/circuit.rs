use js_sys::Map;
use quilla::Circuit as QuillaCircuit;
use wasm_bindgen::prelude::*;

use crate::bitvec;
use crate::operation::Operation;
use crate::rand::Rng;

#[wasm_bindgen]
pub struct Circuit {
    inner: QuillaCircuit<Operation>,
}

#[wasm_bindgen]
impl Circuit {
    #[wasm_bindgen(constructor)]
    pub fn new(qbits: usize, cbits: usize) -> Self {
        Self {
            inner: QuillaCircuit::new(qbits, cbits),
        }
    }

    pub fn qbits(&self) -> usize {
        self.inner.qbits()
    }

    pub fn cbits(&self) -> usize {
        self.inner.cbits()
    }

    #[wasm_bindgen(unchecked_return_type = "Operation[]")]
    pub fn operations(&self) -> Result<Vec<JsValue>, serde_wasm_bindgen::Error> {
        self.inner
            .operations()
            .iter()
            .map(|op| op.to_value())
            .collect()
    }

    pub fn push(
        &mut self,
        #[wasm_bindgen(unchecked_param_type = "Operation")] operation: JsValue,
    ) -> Result<(), serde_wasm_bindgen::Error> {
        let operation = Operation::from_value(operation)?;
        self.inner.push(operation);
        Ok(())
    }

    // Single-qubit gates

    pub fn i(&mut self, target: usize) {
        self.inner.push(Operation::I { target });
    }

    pub fn h(&mut self, target: usize) {
        self.inner.push(Operation::H { target });
    }

    pub fn x(&mut self, target: usize) {
        self.inner.push(Operation::X { target });
    }

    pub fn y(&mut self, target: usize) {
        self.inner.push(Operation::Y { target });
    }

    pub fn z(&mut self, target: usize) {
        self.inner.push(Operation::Z { target });
    }

    pub fn s(&mut self, target: usize) {
        self.inner.push(Operation::S { target });
    }

    pub fn sdg(&mut self, target: usize) {
        self.inner.push(Operation::Sdg { target });
    }

    pub fn t(&mut self, target: usize) {
        self.inner.push(Operation::T { target });
    }

    pub fn tdg(&mut self, target: usize) {
        self.inner.push(Operation::Tdg { target });
    }

    // Two-qubit gates

    pub fn cx(&mut self, control: usize, target: usize) {
        self.inner.push(Operation::CX { control, target });
    }

    pub fn cy(&mut self, control: usize, target: usize) {
        self.inner.push(Operation::CY { control, target });
    }

    pub fn cz(&mut self, control: usize, target: usize) {
        self.inner.push(Operation::CZ { control, target });
    }

    pub fn swap(&mut self, first: usize, second: usize) {
        self.inner.push(Operation::Swap { first, second });
    }

    // Parametric gates

    pub fn rx(&mut self, theta: f64, target: usize) {
        self.inner.push(Operation::RX { theta, target });
    }

    pub fn ry(&mut self, theta: f64, target: usize) {
        self.inner.push(Operation::RY { theta, target });
    }

    pub fn rz(&mut self, theta: f64, target: usize) {
        self.inner.push(Operation::RZ { theta, target });
    }

    // Measurement

    pub fn meas(&mut self, qbit: usize, cbit: usize) {
        self.inner.push(Operation::Meas { qbit, cbit });
    }

    // Sampling

    #[wasm_bindgen(js_name = "sampleOnce", unchecked_return_type = "BitString")]
    pub fn sample_once(&self, rng: Option<Rng>) -> String {
        bitvec::bits_to_string(
            &self
                .inner
                .sample_once_with_rng::<f64>(&mut rng.unwrap_or_default().as_dyn()),
        )
    }

    #[wasm_bindgen(unchecked_return_type = "Map<BitString, number>")]
    pub fn sample(&self, shots: u32, rng: Option<Rng>) -> Map {
        let results = self
            .inner
            .sample_with_rng::<f64>(shots as usize, &mut rng.unwrap_or_default().as_dyn());
        let map = Map::new();
        for (bits, count) in results {
            map.set(
                &JsValue::from(bitvec::bits_to_string(&bits)),
                &JsValue::from(count),
            );
        }
        map
    }

    #[wasm_bindgen(js_name = "toString")]
    pub fn js_to_string(&self) -> String {
        self.inner.to_string()
    }
}
