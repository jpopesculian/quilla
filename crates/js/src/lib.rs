pub mod circuit;
pub mod operation;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND: &str = include_str!("append.ts");
