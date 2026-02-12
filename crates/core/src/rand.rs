#[cfg(feature = "std")]
pub use rand::rng;

#[cfg(not(feature = "std"))]
#[inline]
pub fn rng() -> rand::rngs::SmallRng {
    rand::SeedableRng::seed_from_u64(0)
}
