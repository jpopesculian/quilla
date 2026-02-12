use num_complex::Complex;

pub const fn c32(re: f32, im: f32) -> Complex<f32> {
    Complex::new(re, im)
}

pub const fn c64(re: f64, im: f64) -> Complex<f64> {
    Complex::new(re, im)
}

#[cfg(test)]
pub(crate) fn assert_complex_close(actual: Complex<f64>, expected: Complex<f64>) {
    let eps = 1e-12;
    assert!(
        (actual.re - expected.re).abs() < eps,
        "real mismatch: expected {}, got {}",
        expected.re,
        actual.re
    );
    assert!(
        (actual.im - expected.im).abs() < eps,
        "imag mismatch: expected {}, got {}",
        expected.im,
        actual.im
    );
}
