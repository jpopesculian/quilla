use alloc::vec::Vec;

pub struct Circuit<O> {
    qbits: usize,
    cbits: usize,
    operations: Vec<O>,
}
