// include readme
#![doc = include_str!("../README.md")]

use std::ops::Deref;
use std::ops::DerefMut;

use argon2::Argon2;
use argon2::Error;
use rand::prelude::*;
use secrecy::zeroize::Zeroizing;
use secrecy::{CloneableSecret, DebugSecret, Zeroize};
pub use secrecy::{ExposeSecret, Secret, SecretBytesMut};

pub mod wrap;

/// Use [Input] if you want to persist state of the passphrase and salt.
///
/// If you are looking for a one-time use, use [derive_key] function instead.
pub struct Input {
    passphrase: Secret<Vec<u8>>,
    salt: Secret<Vec<u8>>,
}

impl Input {
    /// Takes any password an salt inputs which derefs into [u8]
    pub fn new(pwd: impl AsRef<[u8]>, salt: impl AsRef<[u8]>) -> Self {
        Self {
            passphrase: Secret::new(pwd.as_ref().to_vec()),
            salt: Secret::new(salt.as_ref().to_vec()),
        }
    }

    /// Generates and returns a [Seed], wrapped in [Secret]
    pub fn derive_key(&self) -> Result<SecretSeed, Error> {
        let mut output_key_material = [0u8; 32]; // default size is 32 bytes

        Argon2::default().hash_password_into(
            self.passphrase.expose_secret(),
            self.salt.expose_secret(),
            &mut output_key_material,
        )?;

        Ok(SecretSeed::new(Seed(output_key_material.into())))
    }
}

/// Generate output key material using Argon2 passwrod hashing
/// Function generates a [Seed] directly from a password and salt
pub fn derive_key(pwd: impl AsRef<[u8]>, salt: impl AsRef<[u8]>) -> Result<SecretSeed, Error> {
    let mut output_key_material = [0u8; 32]; // default size is 32 bytes

    Argon2::default().hash_password_into(pwd.as_ref(), salt.as_ref(), &mut output_key_material)?;

    Ok(SecretSeed::new(Seed(output_key_material.into())))
}

/// Generates and returns a random [Seed], wrapped in [Secret]
///
/// Uses [rand::thread_rng] to generate a random [Seed]
///
/// # Example
///
/// ```rust
/// use seed_keeper_core::{rand_seed, Seed, Secret, ExposeSecret};
///
/// let seed: Secret<Seed> = rand_seed();
/// assert_eq!(seed.expose_secret().len(), 32);
/// ````
pub fn rand_seed() -> Secret<Seed> {
    let mut rng = rand::thread_rng();
    let mut output_key_material = Zeroizing::new([0u8; 32]); // default size is 32 bytes

    rng.fill_bytes(&mut *output_key_material);

    SecretSeed::new(Seed(Zeroizing::new(*output_key_material)))
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

/// Permits cloning
impl CloneableSecret for Seed {}

/// Provides a `Debug` impl (by default `[[REDACTED]]`)
impl DebugSecret for Seed {}

/// Use this alias when storing secret values
pub type SecretSeed = Secret<Seed>;

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
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), Error> {
        let salt = b"some@email.com"; // Salt should be unique per password
        let password = b"some random words that you made up, for sure!";

        let mut output_key_material_1 = [0u8; 32]; // Can be any desired size
        let mut output_key_material_2 = Seed::default(); // default size is 32 bytes
        let mut output_key_material_3: Vec<u8> = vec![0; 48]; // non-zero length vectors are ok too

        Argon2::default().hash_password_into(password, salt, &mut output_key_material_1)?;
        Argon2::default().hash_password_into(password, salt, &mut output_key_material_2)?;
        Argon2::default().hash_password_into(password, salt, &mut output_key_material_3)?;

        assert_eq!(output_key_material_1, output_key_material_2);

        output_key_material_1.zeroize();
        output_key_material_2.zeroize();

        Ok(())
    }

    #[test]
    fn api_works() -> Result<(), Error> {
        let password = "some random words that you made up, for sure!".to_string();
        let salt = b"some@email.com"; // Salt should be unique per password

        let input = Input::new(&password, salt);

        let seed = input.derive_key()?;

        assert_eq!(
            **seed.expose_secret(),
            [
                164, 103, 254, 113, 126, 241, 57, 240, 100, 56, 243, 125, 155, 224, 40, 242, 178,
                136, 222, 133, 220, 141, 127, 10, 88, 199, 181, 11, 241, 91, 149, 249
            ]
        );

        // print out Seed
        println!("Seed {:?}", seed);

        // also for direct fn
        let seed = derive_key(password, salt)?;

        assert_eq!(
            **seed.expose_secret(),
            [
                164, 103, 254, 113, 126, 241, 57, 240, 100, 56, 243, 125, 155, 224, 40, 242, 178,
                136, 222, 133, 220, 141, 127, 10, 88, 199, 181, 11, 241, 91, 149, 249
            ]
        );

        Ok(())
    }

    // using rand_seed works too
    #[test]
    fn rand_seed_works() -> Result<(), Error> {
        let seed = rand_seed();

        assert_eq!(seed.expose_secret().len(), 32);

        Ok(())
    }
}
