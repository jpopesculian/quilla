use alloc::boxed::Box;

pub type DynRng = Box<dyn rand::Rng>;

#[cfg(feature = "std")]
pub fn rng() -> DynRng {
    Box::new(rand::rng())
}

#[cfg(not(feature = "std"))]
#[inline]
pub fn rng() -> DynRng {
    Box::new(<rand::rngs::SmallRng as rand::SeedableRng>::seed_from_u64(
        0,
    ))
}
