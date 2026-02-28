use alloc::boxed::Box;

use rand::SeedableRng;
use rand::rngs::Xoshiro128PlusPlus;

pub type DynRng = Box<dyn rand::Rng>;

#[inline]
pub fn default_rng() -> DynRng {
    Box::new(Xoshiro128PlusPlus::seed_from_u64(0))
}
