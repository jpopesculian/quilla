use core::f32::consts::{
    FRAC_PI_2 as F32_FRAC_PI_2, FRAC_PI_3 as F32_FRAC_PI_3, FRAC_PI_4 as F32_FRAC_PI_4,
    FRAC_PI_6 as F32_FRAC_PI_6, FRAC_PI_8 as F32_FRAC_PI_8, PI as F32_PI, TAU as F32_TAU,
};
use core::f64::consts::{
    FRAC_PI_2 as F64_FRAC_PI_2, FRAC_PI_3 as F64_FRAC_PI_3, FRAC_PI_4 as F64_FRAC_PI_4,
    FRAC_PI_6 as F64_FRAC_PI_6, FRAC_PI_8 as F64_FRAC_PI_8, PI as F64_PI, TAU as F64_TAU,
};
use num_complex::Complex;
use num_traits::Float;

pub const fn c32(re: f32, im: f32) -> Complex<f32> {
    Complex::new(re, im)
}

pub const fn c64(re: f64, im: f64) -> Complex<f64> {
    Complex::new(re, im)
}

#[inline]
fn is_close<T: Float>(a: T, b: T) -> bool {
    (a - b).abs() < T::epsilon()
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WellKnownAngle {
    FracPi8,
    FracPi6,
    FracPi4,
    FracPi3,
    Frac3Pi8,
    FracPi2,
    Frac5Pi8,
    Frac2Pi3,
    Frac3Pi4,
    Frac5Pi6,
    Frac7Pi8,
    Pi,
    Frac9Pi8,
    Frac7Pi6,
    Frac5Pi4,
    Frac4Pi3,
    Frac11Pi8,
    Frac3Pi2,
    Frac13Pi8,
    Frac5Pi3,
    Frac7Pi4,
    Frac11Pi6,
    Frac15Pi8,
    Tau,
}

impl WellKnownAngle {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FracPi8 => "π/8",
            Self::FracPi6 => "π/6",
            Self::FracPi4 => "π/4",
            Self::FracPi3 => "π/3",
            Self::Frac3Pi8 => "3π/8",
            Self::FracPi2 => "π/2",
            Self::Frac5Pi8 => "5π/8",
            Self::Frac2Pi3 => "2π/3",
            Self::Frac3Pi4 => "3π/4",
            Self::Frac5Pi6 => "5π/6",
            Self::Frac7Pi8 => "7π/8",
            Self::Pi => "π",
            Self::Frac9Pi8 => "9π/8",
            Self::Frac7Pi6 => "7π/6",
            Self::Frac5Pi4 => "5π/4",
            Self::Frac4Pi3 => "4π/3",
            Self::Frac11Pi8 => "11π/8",
            Self::Frac3Pi2 => "3π/2",
            Self::Frac13Pi8 => "13π/8",
            Self::Frac5Pi3 => "5π/3",
            Self::Frac7Pi4 => "7π/4",
            Self::Frac11Pi6 => "11π/6",
            Self::Frac15Pi8 => "15π/8",
            Self::Tau => "2π",
        }
    }
}

const WELL_KNOWN_ANGLES: [WellKnownAngle; 24] = [
    WellKnownAngle::FracPi8,
    WellKnownAngle::FracPi6,
    WellKnownAngle::FracPi4,
    WellKnownAngle::FracPi3,
    WellKnownAngle::Frac3Pi8,
    WellKnownAngle::FracPi2,
    WellKnownAngle::Frac5Pi8,
    WellKnownAngle::Frac2Pi3,
    WellKnownAngle::Frac3Pi4,
    WellKnownAngle::Frac5Pi6,
    WellKnownAngle::Frac7Pi8,
    WellKnownAngle::Pi,
    WellKnownAngle::Frac9Pi8,
    WellKnownAngle::Frac7Pi6,
    WellKnownAngle::Frac5Pi4,
    WellKnownAngle::Frac4Pi3,
    WellKnownAngle::Frac11Pi8,
    WellKnownAngle::Frac3Pi2,
    WellKnownAngle::Frac13Pi8,
    WellKnownAngle::Frac5Pi3,
    WellKnownAngle::Frac7Pi4,
    WellKnownAngle::Frac11Pi6,
    WellKnownAngle::Frac15Pi8,
    WellKnownAngle::Tau,
];

const WELL_KNOWN_ANGLES_F32: [f32; 24] = [
    F32_FRAC_PI_8,
    F32_FRAC_PI_6,
    F32_FRAC_PI_4,
    F32_FRAC_PI_3,
    F32_FRAC_PI_8 * 3.0,
    F32_FRAC_PI_2,
    F32_FRAC_PI_8 * 5.0,
    F32_FRAC_PI_3 * 2.0,
    F32_FRAC_PI_4 * 3.0,
    F32_FRAC_PI_6 * 5.0,
    F32_FRAC_PI_8 * 7.0,
    F32_PI,
    F32_FRAC_PI_8 * 9.0,
    F32_FRAC_PI_6 * 7.0,
    F32_FRAC_PI_4 * 5.0,
    F32_FRAC_PI_3 * 4.0,
    F32_FRAC_PI_8 * 11.0,
    F32_FRAC_PI_2 * 3.0,
    F32_FRAC_PI_8 * 13.0,
    F32_FRAC_PI_3 * 5.0,
    F32_FRAC_PI_4 * 7.0,
    F32_FRAC_PI_6 * 11.0,
    F32_FRAC_PI_8 * 15.0,
    F32_TAU,
];

const WELL_KNOWN_ANGLES_F64: [f64; 24] = [
    F64_FRAC_PI_8,
    F64_FRAC_PI_6,
    F64_FRAC_PI_4,
    F64_FRAC_PI_3,
    F64_FRAC_PI_8 * 3.0,
    F64_FRAC_PI_2,
    F64_FRAC_PI_8 * 5.0,
    F64_FRAC_PI_3 * 2.0,
    F64_FRAC_PI_4 * 3.0,
    F64_FRAC_PI_6 * 5.0,
    F64_FRAC_PI_8 * 7.0,
    F64_PI,
    F64_FRAC_PI_8 * 9.0,
    F64_FRAC_PI_6 * 7.0,
    F64_FRAC_PI_4 * 5.0,
    F64_FRAC_PI_3 * 4.0,
    F64_FRAC_PI_8 * 11.0,
    F64_FRAC_PI_2 * 3.0,
    F64_FRAC_PI_8 * 13.0,
    F64_FRAC_PI_3 * 5.0,
    F64_FRAC_PI_4 * 7.0,
    F64_FRAC_PI_6 * 11.0,
    F64_FRAC_PI_8 * 15.0,
    F64_TAU,
];

pub trait FloatExt {
    fn well_known_angle(self) -> Option<WellKnownAngle>;
}

impl FloatExt for f64 {
    fn well_known_angle(self) -> Option<WellKnownAngle> {
        let i = WELL_KNOWN_ANGLES_F64
            .binary_search_by(|&v| v.partial_cmp(&self).unwrap_or(core::cmp::Ordering::Less))
            .unwrap_or_else(|i| i);
        [i.checked_sub(1), Some(i)]
            .into_iter()
            .flatten()
            .filter(|&j| j < WELL_KNOWN_ANGLES_F64.len())
            .find(|&j| is_close(WELL_KNOWN_ANGLES_F64[j], self))
            .map(|j| WELL_KNOWN_ANGLES[j])
    }
}

impl FloatExt for f32 {
    fn well_known_angle(self) -> Option<WellKnownAngle> {
        let i = WELL_KNOWN_ANGLES_F32
            .binary_search_by(|&v| v.partial_cmp(&self).unwrap_or(core::cmp::Ordering::Less))
            .unwrap_or_else(|i| i);
        [i.checked_sub(1), Some(i)]
            .into_iter()
            .flatten()
            .filter(|&j| j < WELL_KNOWN_ANGLES_F32.len())
            .find(|&j| is_close(WELL_KNOWN_ANGLES_F32[j], self))
            .map(|j| WELL_KNOWN_ANGLES[j])
    }
}

#[cfg(test)]
pub(crate) fn assert_complex_close(actual: Complex<f64>, expected: Complex<f64>) {
    assert!(
        is_close(actual.re, expected.re),
        "real mismatch: expected {}, got {}",
        expected.re,
        actual.re
    );
    assert!(
        is_close(actual.im, expected.im),
        "imag mismatch: expected {}, got {}",
        expected.im,
        actual.im
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f32_well_known_angles_roundtrip() {
        for (i, &value) in WELL_KNOWN_ANGLES_F32.iter().enumerate() {
            assert_eq!(
                value.well_known_angle(),
                Some(WELL_KNOWN_ANGLES[i]),
                "f32 index {i} failed"
            );
        }
    }

    #[test]
    fn f32_unknown_angle_returns_none() {
        assert_eq!(1.0_f32.well_known_angle(), None);
    }

    #[test]
    fn f64_well_known_angles_roundtrip() {
        for (i, &value) in WELL_KNOWN_ANGLES_F64.iter().enumerate() {
            assert_eq!(
                value.well_known_angle(),
                Some(WELL_KNOWN_ANGLES[i]),
                "f64 index {i} failed"
            );
        }
    }

    #[test]
    fn f64_unknown_angle_returns_none() {
        assert_eq!(1.0_f64.well_known_angle(), None);
    }
}
