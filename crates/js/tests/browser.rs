use js_sys::JsString;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

use quilla_js::circuit::Circuit;
use quilla_js::rand::Rng;
use quilla_js::state_vector::StateVector;

wasm_bindgen_test_configure!(run_in_browser);

// --- Circuit ---

#[wasm_bindgen_test]
fn circuit_x_then_measure_gives_one() {
    let mut c = Circuit::new(1, 1);
    c.x(0);
    c.meas(0, 0);
    assert_eq!(c.sample_once(Some(Rng::seeded(0.0))), "1");
}

#[wasm_bindgen_test]
fn circuit_sample_returns_correct_counts() {
    let mut c = Circuit::new(1, 1);
    c.x(0);
    c.meas(0, 0);
    let map = c.sample(50, Some(Rng::seeded(0.0)));
    let count = map.get(&JsValue::from_str("1")).as_f64().unwrap() as u32;
    assert_eq!(count, 50);
    assert!(map.get(&JsValue::from_str("0")).is_undefined());
}

#[wasm_bindgen_test]
fn bell_state_only_00_and_11() {
    let mut c = Circuit::new(2, 2);
    c.h(0);
    c.cx(0, 1);
    c.meas(0, 0);
    c.meas(1, 1);
    let map = c.sample(200, Some(Rng::seeded(42.0)));
    assert!(map.get(&JsValue::from_str("01")).is_undefined());
    assert!(map.get(&JsValue::from_str("10")).is_undefined());
    let count_00 = map.get(&JsValue::from_str("00")).as_f64().unwrap_or(0.0) as u32;
    let count_11 = map.get(&JsValue::from_str("11")).as_f64().unwrap_or(0.0) as u32;
    assert_eq!(count_00 + count_11, 200);
    assert!(count_00 > 0);
    assert!(count_11 > 0);
}

#[wasm_bindgen_test]
fn circuit_to_string_contains_gate_names() {
    let mut c = Circuit::new(2, 1);
    c.h(0);
    c.cx(0, 1);
    let s = c.js_to_string();
    assert!(s.contains('H') || s.contains('h'));
}

// --- StateVector ---

fn re_im(amp: JsValue) -> (f64, f64) {
    let re = js_sys::Reflect::get(&amp, &JsString::from("re"))
        .unwrap()
        .as_f64()
        .unwrap();
    let im = js_sys::Reflect::get(&amp, &JsString::from("im"))
        .unwrap()
        .as_f64()
        .unwrap();
    (re, im)
}

#[wasm_bindgen_test]
fn state_vector_initial_amplitude_is_one() {
    let sv = StateVector::new(1, 0);
    let (re, im) = re_im(sv.amplitude("0").unwrap());
    assert!((re - 1.0).abs() < 1e-10);
    assert!(im.abs() < 1e-10);
}

#[wasm_bindgen_test]
fn state_vector_after_x_amplitude_flips() {
    let mut sv = StateVector::new(1, 0);
    sv.apply(
        quilla_js::operation::Operation::X { target: 0 }.to_value().unwrap(),
        None,
    )
    .unwrap();
    let (re0, _) = re_im(sv.amplitude("0").unwrap());
    let (re1, _) = re_im(sv.amplitude("1").unwrap());
    assert!(re0.abs() < 1e-10);
    assert!((re1 - 1.0).abs() < 1e-10);
}
