use alloc::boxed::Box;

pub type DynRng = Box<dyn rand::Rng>;

#[inline]
pub fn default_rng() -> DynRng {
    #[cfg(feature = "thread_rng")]
    {
        Box::new(rand::rng())
    }
    #[cfg(not(feature = "thread_rng"))]
    {
        use rand::{SeedableRng, rngs::Xoshiro128PlusPlus};
        Box::new(Xoshiro128PlusPlus::seed_from_u64(0))
    }
}
