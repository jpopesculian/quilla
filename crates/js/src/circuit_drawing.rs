use quilla::CircuitDrawing as QuillaCircuitDrawing;
use wasm_bindgen::prelude::*;

use crate::operation::Operation;

#[wasm_bindgen]
pub struct CircuitDrawing {
    inner: QuillaCircuitDrawing,
}

#[wasm_bindgen]
impl CircuitDrawing {
    #[wasm_bindgen(constructor)]
    pub fn new(qbits: usize, cbits: usize) -> Self {
        Self {
            inner: QuillaCircuitDrawing::new(qbits, cbits),
        }
    }

    pub fn wires(&self) -> usize {
        self.inner.wires()
    }

    pub fn operations(&self) -> usize {
        self.inner.operations()
    }

    pub fn draw(
        &mut self,
        #[wasm_bindgen(unchecked_param_type = "Operation")] operation: JsValue,
    ) -> Result<(), serde_wasm_bindgen::Error> {
        self.inner.draw(Operation::from_value(operation)?);
        Ok(())
    }

    #[wasm_bindgen(js_name = "toString")]
    pub fn js_to_string(&self) -> String {
        self.inner.to_string()
    }
}
