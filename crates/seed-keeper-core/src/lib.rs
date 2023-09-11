use std::ops::Deref;
use std::ops::DerefMut;

use argon2::Argon2;
use argon2::Error;
use secrecy::{CloneableSecret, DebugSecret, ExposeSecret, Secret, Zeroize};

pub mod wrap;

pub struct Input {
    passphrase: Secret<Vec<u8>>,
    salt: Secret<Vec<u8>>,
}

impl Input {
    pub fn new(pwd: &[u8], salt: &[u8]) -> Self {
        Self {
            passphrase: Secret::new(pwd.to_vec()),
            salt: Secret::new(salt.to_vec()),
        }
    }

    pub fn generate_seed(&self) -> Result<SecretSeed, Error> {
        let mut output_key_material = [0u8; 32]; // default size is 32 bytes

        Argon2::default().hash_password_into(
            self.passphrase.expose_secret(),
            self.salt.expose_secret(),
            &mut output_key_material,
        )?;

        Ok(SecretSeed::new(Seed(output_key_material)))
    }
}

pub fn generate_seed(pwd: &[u8], salt: &[u8]) -> Result<SecretSeed, Error> {
    let mut output_key_material = [0u8; 32]; // default size is 32 bytes

    Argon2::default().hash_password_into(pwd, salt, &mut output_key_material)?;

    Ok(SecretSeed::new(Seed(output_key_material)))
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Seed([u8; 32]);

impl Seed {
    pub fn new(seed: [u8; 32]) -> Self {
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
        &self.0
    }
}

impl DerefMut for Seed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<[u8]> for Seed {
    fn as_ref(&self) -> &[u8] {
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

        let input = Input {
            passphrase: Secret::new(password.to_vec()),
            salt: Secret::new(salt.to_vec()),
        };

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
        let password = b"some random words that you made up, for sure!";
        let salt = b"some@email.com"; // Salt should be unique per password

        let input = Input::new(password, salt);

        let seed = input.generate_seed()?;

        assert_eq!(
            seed.expose_secret(),
            &Seed([
                164, 103, 254, 113, 126, 241, 57, 240, 100, 56, 243, 125, 155, 224, 40, 242, 178,
                136, 222, 133, 220, 141, 127, 10, 88, 199, 181, 11, 241, 91, 149, 249
            ])
        );

        // print out Seed
        println!("Seed {:?}", seed);

        // also for direct fn
        let seed = generate_seed(password, salt)?;

        assert_eq!(
            seed.expose_secret(),
            &Seed([
                164, 103, 254, 113, 126, 241, 57, 240, 100, 56, 243, 125, 155, 224, 40, 242, 178,
                136, 222, 133, 220, 141, 127, 10, 88, 199, 181, 11, 241, 91, 149, 249
            ])
        );

        Ok(())
    }
}
