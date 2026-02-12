use ndarray::Array2;
use num_complex::Complex;
use num_traits::Num;

use crate::state_vector::{StateVector, StateVectorOperation};

pub struct UnitaryGate<T, const N: usize> {
    matrix: Array2<Complex<T>>,
    targets: [usize; N],
}

impl<T, const N: usize> UnitaryGate<T, N> {
    pub(crate) fn new(matrix: Array2<Complex<T>>, targets: [usize; N]) -> Self {
        Self { matrix, targets }
    }
}

impl<T> StateVectorOperation<T> for UnitaryGate<T, 1>
where
    T: Num + Copy,
{
    fn apply_to(&self, state: &mut StateVector<T>, _rng: &mut crate::rand::DynRng) {
        let target = self.targets[0];
        let bit = 1 << target;

        let row0 = self.matrix.row(0);
        let row1 = self.matrix.row(1);

        let m00 = row0[0];
        let m01 = row0[1];
        let m10 = row1[0];
        let m11 = row1[1];

        for i0 in 0..state.qstate.len() {
            if (i0 & bit) != 0 {
                continue;
            }

            let i1 = i0 | bit;
            let a0 = state.qstate[i0];
            let a1 = state.qstate[i1];

            state.qstate[i0] = m00 * a0 + m01 * a1;
            state.qstate[i1] = m10 * a0 + m11 * a1;
        }
    }
}

impl<T> StateVectorOperation<T> for UnitaryGate<T, 2>
where
    T: Num + Copy,
{
    fn apply_to(&self, state: &mut StateVector<T>, _rng: &mut crate::rand::DynRng) {
        let target0 = self.targets[0];
        let target1 = self.targets[1];
        let bit0 = 1 << target0;
        let bit1 = 1 << target1;

        let row0 = self.matrix.row(0);
        let row1 = self.matrix.row(1);
        let row2 = self.matrix.row(2);
        let row3 = self.matrix.row(3);

        let m00 = row0[0];
        let m01 = row0[1];
        let m02 = row0[2];
        let m03 = row0[3];

        let m10 = row1[0];
        let m11 = row1[1];
        let m12 = row1[2];
        let m13 = row1[3];

        let m20 = row2[0];
        let m21 = row2[1];
        let m22 = row2[2];
        let m23 = row2[3];

        let m30 = row3[0];
        let m31 = row3[1];
        let m32 = row3[2];
        let m33 = row3[3];

        for i00 in 0..state.qstate.len() {
            if (i00 & bit0) != 0 || (i00 & bit1) != 0 {
                continue;
            }

            let i01 = i00 | bit1;
            let i10 = i00 | bit0;
            let i11 = i00 | bit0 | bit1;

            let a00 = state.qstate[i00];
            let a01 = state.qstate[i01];
            let a10 = state.qstate[i10];
            let a11 = state.qstate[i11];

            state.qstate[i00] = m00 * a00 + m01 * a01 + m02 * a10 + m03 * a11;
            state.qstate[i01] = m10 * a00 + m11 * a01 + m12 * a10 + m13 * a11;
            state.qstate[i10] = m20 * a00 + m21 * a01 + m22 * a10 + m23 * a11;
            state.qstate[i11] = m30 * a00 + m31 * a01 + m32 * a10 + m33 * a11;
        }
    }
}
