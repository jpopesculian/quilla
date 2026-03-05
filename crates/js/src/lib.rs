mod bitvec;
pub mod circuit;
pub mod circuit_drawing;
pub mod complex;
pub mod lang;
pub mod operation;
pub mod rand;
pub mod state_vector;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND: &str = include_str!("append.ts");
