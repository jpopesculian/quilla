use quilla::StateVector as QuillaStateVector;
use wasm_bindgen::prelude::*;

use crate::bitvec::string_to_bits;
use crate::complex::Complex;
use crate::operation::Operation;

#[wasm_bindgen]
pub struct StateVector {
    inner: QuillaStateVector<f64>,
}

#[wasm_bindgen]
impl StateVector {
    #[wasm_bindgen(constructor)]
    pub fn new(qbits: usize, cbits: usize) -> Self {
        Self {
            inner: QuillaStateVector::new(qbits, cbits),
        }
    }

    pub fn qbits(&self) -> usize {
        self.inner.qbits()
    }

    pub fn cbits(&self) -> usize {
        self.inner.cbits()
    }

    #[wasm_bindgen(unchecked_return_type = "Complex")]
    pub fn amplitude(&self, bits: &str) -> Result<JsValue, JsError> {
        Ok(Complex::from(*self.inner.get(&string_to_bits(bits)?)).to_value()?)
    }

    pub fn apply(
        &mut self,
        #[wasm_bindgen(unchecked_param_type = "Operation")] operation: JsValue,
    ) -> Result<(), serde_wasm_bindgen::Error> {
        let mut rng = quilla::rand::default_rng();
        self.inner
            .apply(Operation::from_value(operation)?, &mut rng);
        Ok(())
    }
}
