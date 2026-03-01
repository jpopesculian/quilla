use num_complex::Complex64;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use wasm_bindgen::JsValue;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Complex {
    re: f64,
    im: f64,
}

impl From<Complex64> for Complex {
    fn from(value: Complex64) -> Self {
        Self {
            re: value.re,
            im: value.im,
        }
    }
}

impl Complex {
    pub fn from_value(value: JsValue) -> Result<Self, serde_wasm_bindgen::Error> {
        serde_wasm_bindgen::from_value(value)
    }
    pub fn to_value(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        self.serialize(&serde_wasm_bindgen::Serializer::json_compatible())
    }
}
