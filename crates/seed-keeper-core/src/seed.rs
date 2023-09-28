//! Utilities for using secure Seeds
use rand::prelude::*;
use std::ops::Deref;
use std::ops::DerefMut;
use zeroize::Zeroize;
use zeroize::{ZeroizeOnDrop, Zeroizing};

/// Generates and returns a random [Seed], with memory [Zeroize]d.
///
/// Uses [rand::thread_rng] to generate a random [Seed]
///
/// # Example
///
/// ```rust
/// use seed_keeper_core::seed::{rand_seed, Seed};
///
/// let seed: Seed = rand_seed();
/// assert_eq!(seed.len(), 32);
/// ````
pub fn rand_seed() -> Seed {
    let mut rng = rand::thread_rng();
    let mut output_key_material = Zeroizing::new([0u8; 32]); // default size is 32 bytes

    rng.fill_bytes(&mut *output_key_material);

    Seed::new(Zeroizing::new(*output_key_material))
}

/// Seed is a wrapper around [u8; 32] to ensure it is always 32 bytes
///
/// To ensure users don't expose bytes to vulnerable memory,
/// we insist they wrap their bytes in [Zeroizing] when they pass
/// the bytes to [Seed].
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Seed(Zeroizing<[u8; 32]>);

impl Seed {
    /// Creates a new [Seed] from a [u8; 32]
    pub fn new(seed: Zeroizing<[u8; 32]>) -> Self {
        Self(seed)
    }
}

impl Zeroize for Seed {
    fn zeroize(&mut self) {
        self.0.zeroize();
    }
}

/// Zeroizes on drop
impl ZeroizeOnDrop for Seed {}

impl Deref for Seed {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl DerefMut for Seed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}

impl AsRef<[u8]> for Seed {
    fn as_ref(&self) -> &[u8] {
        &*self.0
    }
}

impl AsRef<[u8; 32]> for Seed {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl PartialEq<Seed> for [u8] {
    fn eq(&self, other: &Seed) -> bool {
        self[..] == other[..]
    }
}

impl PartialEq<Seed> for [u8; 32] {
    fn eq(&self, other: &Seed) -> bool {
        self[..] == other[..]
    }
}

#[cfg(test)]
mod seed_tests {
    use super::*;

    // using rand_seed works too
    #[test]
    fn rand_seed_works() {
        let seed = rand_seed();

        assert_eq!(seed.len(), 32);
    }
}
