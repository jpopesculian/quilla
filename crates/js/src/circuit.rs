use js_sys::Map;
use quilla::DynCircuit;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Circuit {
    inner: DynCircuit,
}

#[wasm_bindgen]
impl Circuit {
    #[wasm_bindgen(constructor)]
    pub fn new(qbits: usize, cbits: usize) -> Self {
        Self {
            inner: DynCircuit::new(qbits, cbits),
        }
    }

    pub fn qbits(&self) -> usize {
        self.inner.qbits()
    }

    pub fn cbits(&self) -> usize {
        self.inner.cbits()
    }

    // Single-qubit gates

    pub fn i(&mut self, target: usize) {
        self.inner.i(target);
    }

    pub fn h(&mut self, target: usize) {
        self.inner.h(target);
    }

    pub fn x(&mut self, target: usize) {
        self.inner.x(target);
    }

    pub fn y(&mut self, target: usize) {
        self.inner.y(target);
    }

    pub fn z(&mut self, target: usize) {
        self.inner.z(target);
    }

    pub fn s(&mut self, target: usize) {
        self.inner.s(target);
    }

    pub fn sdg(&mut self, target: usize) {
        self.inner.sdg(target);
    }

    pub fn t(&mut self, target: usize) {
        self.inner.t(target);
    }

    pub fn tdg(&mut self, target: usize) {
        self.inner.tdg(target);
    }

    // Two-qubit gates

    pub fn cx(&mut self, control: usize, target: usize) {
        self.inner.cx(control, target);
    }

    pub fn cy(&mut self, control: usize, target: usize) {
        self.inner.cy(control, target);
    }

    pub fn cz(&mut self, control: usize, target: usize) {
        self.inner.cz(control, target);
    }

    pub fn swap(&mut self, first: usize, second: usize) {
        self.inner.swap(first, second);
    }

    // Parametric gates

    pub fn rx(&mut self, theta: f64, target: usize) {
        self.inner.rx(theta, target);
    }

    pub fn ry(&mut self, theta: f64, target: usize) {
        self.inner.ry(theta, target);
    }

    pub fn rz(&mut self, theta: f64, target: usize) {
        self.inner.rz(theta, target);
    }

    // Measurement

    pub fn meas(&mut self, qbit: usize, cbit: usize) {
        self.inner.meas(qbit, cbit);
    }

    // Sampling

    /// Run the circuit once and return the classical bit register as an array of 0/1 values.
    pub fn sample_once(&self) -> Vec<u8> {
        self.inner
            .sample_once::<f64>()
            .iter()
            .map(|b| *b as u8)
            .collect()
    }

    /// Run the circuit `shots` times and return a `Map<string, number>` from
    /// bit-string (e.g. `"01"`) to occurrence count.
    pub fn sample(&self, shots: usize) -> Map {
        let results = self.inner.sample::<f64>(shots);
        let map = Map::new();
        for (bits, count) in results {
            let key: String = bits.iter().map(|b| if *b { '1' } else { '0' }).collect();
            map.set(&JsValue::from_str(&key), &JsValue::from_f64(count as f64));
        }
        map
    }

    #[wasm_bindgen(js_name = "toString")]
    pub fn js_to_string(&self) -> String {
        self.inner.to_string()
    }
}
