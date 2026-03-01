use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

use quilla::rand::{DynRng, default_rng};
use rand::{SeedableRng, rngs::Xoshiro128PlusPlus};

#[derive(Clone)]
#[wasm_bindgen]
pub struct Rng {
    inner: Rc<RefCell<DynRng>>,
}

#[wasm_bindgen]
impl Rng {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(default_rng())),
        }
    }

    pub fn seeded(seed: f64) -> Self {
        let seed = u64::from_le_bytes(seed.to_le_bytes());
        Self {
            inner: Rc::new(RefCell::new(Box::new(Xoshiro128PlusPlus::seed_from_u64(
                seed,
            )))),
        }
    }
}

impl Rng {
    pub fn as_dyn(&self) -> DynRng {
        Box::new(self.clone())
    }
}

impl Default for Rng {
    fn default() -> Self {
        Self::new()
    }
}

impl rand::TryRng for Rng {
    type Error = core::convert::Infallible;
    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        Ok(self.inner.borrow_mut().next_u32())
    }
    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        Ok(self.inner.borrow_mut().next_u64())
    }
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Self::Error> {
        self.inner.borrow_mut().fill_bytes(dest);
        Ok(())
    }
}
