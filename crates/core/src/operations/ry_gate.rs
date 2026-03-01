use alloc::string::ToString;
use alloc::vec;
use num_traits::{Float, Num};

use super::unitary_gate::UnitaryGate;
use crate::draw::{CircuitDrawing, DrawOperation, DrawPosition};
use crate::num::{FloatExt, c32, c64};
use crate::state_vector::{StateVector, StateVectorOperation};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct RYGate<T> {
    theta: T,
    target: usize,
}

impl<T> RYGate<T> {
    pub fn new(theta: T, target: usize) -> Self {
        Self { theta, target }
    }
}

impl<T> DrawOperation for RYGate<T>
where
    T: FloatExt + Copy,
{
    fn draw_to(&self, d: &mut CircuitDrawing) {
        let theta_str = match self.theta.well_known_angle() {
            Some(angle) => angle.as_str(),
            None => "θ",
        };
        let mut label = "Ry".to_string();
        label.push_str(theta_str);
        d.push_box(DrawPosition::Qbit(self.target), &label);
    }
}

impl From<RYGate<f32>> for RYGate<f64> {
    fn from(gate: RYGate<f32>) -> Self {
        Self {
            theta: gate.theta as f64,
            target: gate.target,
        }
    }
}

impl From<RYGate<f64>> for RYGate<f32> {
    fn from(gate: RYGate<f64>) -> Self {
        Self {
            theta: gate.theta as f32,
            target: gate.target,
        }
    }
}

impl<T, U> StateVectorOperation<T> for RYGate<U>
where
    T: Num + Copy,
    U: Copy,
    UnitaryGate<T, 1>: From<RYGate<U>>,
{
    fn apply_to(&self, state: &mut StateVector<T>, rng: &mut crate::rand::DynRng) {
        UnitaryGate::<T, 1>::from(*self).apply_to(state, rng)
    }
}

impl From<RYGate<f32>> for UnitaryGate<f32, 1> {
    fn from(gate: RYGate<f32>) -> Self {
        let half_theta = gate.theta / 2.0;
        let c = <f32 as Float>::cos(half_theta);
        let s = <f32 as Float>::sin(half_theta);

        UnitaryGate::new(
            vec![c32(c, 0.), c32(-s, 0.), c32(s, 0.), c32(c, 0.)],
            [gate.target],
        )
    }
}

impl From<RYGate<f64>> for UnitaryGate<f32, 1> {
    fn from(gate: RYGate<f64>) -> Self {
        RYGate::<f32>::from(gate).into()
    }
}

impl From<RYGate<f64>> for UnitaryGate<f64, 1> {
    fn from(gate: RYGate<f64>) -> Self {
        let half_theta = gate.theta / 2.0;
        let c = <f64 as Float>::cos(half_theta);
        let s = <f64 as Float>::sin(half_theta);

        UnitaryGate::new(
            vec![c64(c, 0.), c64(-s, 0.), c64(s, 0.), c64(c, 0.)],
            [gate.target],
        )
    }
}

impl From<RYGate<f32>> for UnitaryGate<f64, 1> {
    fn from(gate: RYGate<f32>) -> Self {
        RYGate::<f64>::from(gate).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::num::{assert_complex_close, c64};
    use crate::state_vector::StateVector;

    #[test]
    fn ry_pi_maps_zero_to_one() {
        let mut state = StateVector::<f64>::new(1, 0);
        state.qstate[0] = c64(1.0, 0.0);
        let gate = RYGate::new(core::f64::consts::PI, 0);
        let mut rng = crate::rand::default_rng();

        gate.apply_to(&mut state, &mut rng);

        assert_complex_close(state.qstate[0], c64(0.0, 0.0));
        assert_complex_close(state.qstate[1], c64(1.0, 0.0));
    }
}
