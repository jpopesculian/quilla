use alloc::string::ToString;
use alloc::vec;

use ndarray::array;
use num_traits::{Float, Num};

use super::unitary_gate::UnitaryGate;
use crate::draw::{CircuitDrawing, DrawOperation, DrawPosition};
use crate::num::{FloatExt, c32, c64};
use crate::state_vector::{StateVector, StateVectorOperation};

#[derive(Clone, Copy, Debug)]
pub struct RXGate<T> {
    theta: T,
    target: usize,
}

impl<T> RXGate<T> {
    pub fn new(theta: T, target: usize) -> Self {
        Self { theta, target }
    }
}

impl<T> DrawOperation for RXGate<T>
where
    T: FloatExt + Copy,
{
    fn draw_to(&self, d: &mut CircuitDrawing) {
        let theta_str = match self.theta.well_known_angle() {
            Some(angle) => angle.as_str(),
            None => "θ",
        };
        let mut label = "Rx".to_string();
        label.push_str(theta_str);
        d.push_box(DrawPosition::Qbit(self.target), &label);
    }
}

impl From<RXGate<f32>> for RXGate<f64> {
    fn from(gate: RXGate<f32>) -> Self {
        Self {
            theta: gate.theta as f64,
            target: gate.target,
        }
    }
}

impl From<RXGate<f64>> for RXGate<f32> {
    fn from(gate: RXGate<f64>) -> Self {
        Self {
            theta: gate.theta as f32,
            target: gate.target,
        }
    }
}

impl<T, U> StateVectorOperation<T> for RXGate<U>
where
    T: Num + Copy,
    U: Copy,
    UnitaryGate<T, 1>: From<RXGate<U>>,
{
    fn apply_to(&self, state: &mut StateVector<T>, rng: &mut crate::rand::DynRng) {
        UnitaryGate::<T, 1>::from(*self).apply_to(state, rng)
    }
}

impl From<RXGate<f32>> for UnitaryGate<f32, 1> {
    fn from(gate: RXGate<f32>) -> Self {
        let half_theta = gate.theta / 2.0;
        let c = <f32 as Float>::cos(half_theta);
        let s = <f32 as Float>::sin(half_theta);

        UnitaryGate::new(
            array![[c32(c, 0.), c32(0., -s)], [c32(0., -s), c32(c, 0.)]],
            [gate.target],
        )
    }
}

impl From<RXGate<f64>> for UnitaryGate<f32, 1> {
    fn from(gate: RXGate<f64>) -> Self {
        RXGate::<f32>::from(gate).into()
    }
}

impl From<RXGate<f64>> for UnitaryGate<f64, 1> {
    fn from(gate: RXGate<f64>) -> Self {
        let half_theta = gate.theta / 2.0;
        let c = <f64 as Float>::cos(half_theta);
        let s = <f64 as Float>::sin(half_theta);

        UnitaryGate::new(
            array![[c64(c, 0.), c64(0., -s)], [c64(0., -s), c64(c, 0.)]],
            [gate.target],
        )
    }
}

impl From<RXGate<f32>> for UnitaryGate<f64, 1> {
    fn from(gate: RXGate<f32>) -> Self {
        RXGate::<f64>::from(gate).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::num::{assert_complex_close, c64};
    use crate::state_vector::StateVector;

    #[test]
    fn rx_pi_maps_zero_to_minus_i_one() {
        let mut state = StateVector::<f64>::new(1, 0);
        state.qstate[0] = c64(1.0, 0.0);
        let gate = RXGate::new(core::f64::consts::PI, 0);
        let mut rng = crate::rand::rng();

        gate.apply_to(&mut state, &mut rng);

        assert_complex_close(state.qstate[0], c64(0.0, 0.0));
        assert_complex_close(state.qstate[1], c64(0.0, -1.0));
    }
}
