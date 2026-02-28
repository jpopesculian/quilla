use num_complex::Complex;
use num_traits::Num;

use crate::state_vector::{StateVector, StateVectorOperation};

#[derive(Clone, Debug)]
pub struct UnitaryGate<T, const N: usize> {
    matrix: Vec<Complex<T>>,
    targets: [usize; N],
}

impl<T, const N: usize> UnitaryGate<T, N> {
    pub(crate) fn new(matrix: Vec<Complex<T>>, targets: [usize; N]) -> Self {
        assert_eq!(matrix.len(), (1 << N) * (1 << N));
        Self { matrix, targets }
    }

    #[inline]
    pub(crate) fn get(&self, i: usize, j: usize) -> Complex<T>
    where
        T: Copy,
    {
        self.matrix[i * (1 << N) + j]
    }
}

impl<T> StateVectorOperation<T> for UnitaryGate<T, 1>
where
    T: Num + Copy,
{
    fn apply_to(&self, state: &mut StateVector<T>, _rng: &mut crate::rand::DynRng) {
        let target = self.targets[0];
        let bit = 1 << target;

        let m00 = self.get(0, 0);
        let m01 = self.get(0, 1);
        let m10 = self.get(1, 0);
        let m11 = self.get(1, 1);

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

        let m00 = self.get(0, 0);
        let m01 = self.get(0, 1);
        let m02 = self.get(0, 2);
        let m03 = self.get(0, 3);

        let m10 = self.get(1, 0);
        let m11 = self.get(1, 1);
        let m12 = self.get(1, 2);
        let m13 = self.get(1, 3);

        let m20 = self.get(2, 0);
        let m21 = self.get(2, 1);
        let m22 = self.get(2, 2);
        let m23 = self.get(2, 3);

        let m30 = self.get(3, 0);
        let m31 = self.get(3, 1);
        let m32 = self.get(3, 2);
        let m33 = self.get(3, 3);

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
