use quilla::{
    CXGate, CYGate, CZGate, DrawOperation, HadamardGate, IdentityGate, Measure,
    Operation as OperationTrait, RXGate, RYGate, RZGate, SDaggerGate, SGate, StateVector,
    StateVectorOperation, SwapGate, TDaggerGate, TGate, XGate, YGate, ZGate, rand::DynRng,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use wasm_bindgen::JsValue;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, TS)]
#[serde(tag = "kind", rename_all = "lowercase")]
#[ts(export)]
pub enum Operation {
    H { target: usize },
    I { target: usize },
    X { target: usize },
    Y { target: usize },
    Z { target: usize },
    S { target: usize },
    Sdg { target: usize },
    T { target: usize },
    Tdg { target: usize },
    CX { control: usize, target: usize },
    CY { control: usize, target: usize },
    CZ { control: usize, target: usize },
    Swap { first: usize, second: usize },
    RX { theta: f64, target: usize },
    RY { theta: f64, target: usize },
    RZ { theta: f64, target: usize },
    Meas { qbit: usize, cbit: usize },
}

impl Operation {
    pub fn from_value(value: JsValue) -> Result<Self, serde_wasm_bindgen::Error> {
        serde_wasm_bindgen::from_value(value)
    }
    pub fn to_value(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        self.serialize(&serde_wasm_bindgen::Serializer::json_compatible())
    }
    pub fn as_dyn(self) -> Box<dyn OperationTrait> {
        match self {
            Self::H { target } => Box::new(HadamardGate::new(target)),
            Self::I { target } => Box::new(IdentityGate::new(target)),
            Self::X { target } => Box::new(XGate::new(target)),
            Self::Y { target } => Box::new(YGate::new(target)),
            Self::Z { target } => Box::new(ZGate::new(target)),
            Self::S { target } => Box::new(SGate::new(target)),
            Self::Sdg { target } => Box::new(SDaggerGate::new(target)),
            Self::T { target } => Box::new(TGate::new(target)),
            Self::Tdg { target } => Box::new(TDaggerGate::new(target)),
            Self::CX { control, target } => Box::new(CXGate::new(control, target)),
            Self::CY { control, target } => Box::new(CYGate::new(control, target)),
            Self::CZ { control, target } => Box::new(CZGate::new(control, target)),
            Self::Swap { first, second } => Box::new(SwapGate::new(first, second)),
            Self::RX { theta, target } => Box::new(RXGate::new(theta, target)),
            Self::RY { theta, target } => Box::new(RYGate::new(theta, target)),
            Self::RZ { theta, target } => Box::new(RZGate::new(theta, target)),
            Self::Meas { qbit, cbit } => Box::new(Measure::new(qbit, cbit)),
        }
    }
}

impl StateVectorOperation<f64> for Operation {
    fn apply_to(&self, state: &mut StateVector<f64>, rng: &mut DynRng) {
        self.as_dyn().apply_to(state, rng);
    }
}

impl DrawOperation for Operation {
    fn draw_to(&self, d: &mut quilla::CircuitDrawing) {
        self.as_dyn().draw_to(d);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitvec::prelude::*;
    use quilla::{StateVector, rand::default_rng};

    fn re(state: &StateVector<f64>, bits: &bitvec::vec::BitVec) -> f64 {
        state.get(bits).re
    }

    #[test]
    fn x_flips_zero_to_one() {
        let mut state = StateVector::<f64>::new(1, 0);
        let mut rng = default_rng();
        Operation::X { target: 0 }.apply_to(&mut state, &mut rng);
        assert!((re(&state, &bitvec![0])).abs() < 1e-10);
        assert!((re(&state, &bitvec![1]) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn cx_flips_target_when_control_set() {
        // Start in |10⟩: bit 0 = 0, bit 1 = 1 → index 2
        let mut state = StateVector::<f64>::new(2, 0);
        let mut rng = default_rng();
        Operation::X { target: 1 }.apply_to(&mut state, &mut rng);
        Operation::CX { control: 1, target: 0 }.apply_to(&mut state, &mut rng);
        // Result should be |11⟩: bit 0 = 1, bit 1 = 1 → index 3
        assert!((re(&state, &bitvec![1, 1]) - 1.0).abs() < 1e-10);
        assert!((re(&state, &bitvec![0, 1])).abs() < 1e-10);
    }

    #[test]
    fn h_creates_superposition() {
        let mut state = StateVector::<f64>::new(1, 0);
        let mut rng = default_rng();
        Operation::H { target: 0 }.apply_to(&mut state, &mut rng);
        let expected = 1.0 / std::f64::consts::SQRT_2;
        assert!((re(&state, &bitvec![0]) - expected).abs() < 1e-10);
        assert!((re(&state, &bitvec![1]) - expected).abs() < 1e-10);
    }
}
