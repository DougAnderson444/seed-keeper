// include readme
#![doc = include_str!("../README.md")]

pub mod error;
pub mod seed;
pub mod wrap;

pub use argon2::Error;
pub use secrecy::zeroize::Zeroizing;
pub use secrecy::{ExposeSecret, Secret, SecretBytesMut};

use argon2::Argon2;
use seed::Seed;

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
    /// Generate output key material using Argon2 passwrod hashing
    /// Function generates a [Seed] directly from a password and salt
    ///
    /// Password must be a minimum of 8 bytes
    /// Salt ust be a minimum of 4 bytes long
    ///
    /// Otherwise, an Argon2 [Error] is returned
    pub fn derive_key(&self) -> Result<Secret<Seed>, Error> {
        let mut output_key_material = Zeroizing::new([0u8; 32]);
        Argon2::default().hash_password_into(
            self.passphrase.expose_secret(),
            self.salt.expose_secret(),
            &mut *output_key_material,
        )?;

        Ok(Secret::new(Seed::new(output_key_material)))
    }
}

/// Generate output key material using Argon2 passwrod hashing
/// Function generates a [Seed] directly from a password and salt
///
/// Password must be a minimum of 8 bytes
/// Salt ust be a minimum of 4 bytes long
///
/// Otherwise, an Argon2 [Error] is returned
pub fn derive_key(pwd: impl AsRef<[u8]>, salt: impl AsRef<[u8]>) -> Result<Secret<Seed>, Error> {
    let mut output_key_material = Zeroizing::new([0u8; 32]); // default size is 32 bytes

    Argon2::default().hash_password_into(pwd.as_ref(), salt.as_ref(), &mut *output_key_material)?;

    Ok(Secret::new(Seed::new(output_key_material)))
}

#[cfg(test)]
mod tests {
    use super::*;

    use secrecy::DebugSecret;

    /// Debug Secret Seed
    impl DebugSecret for Seed {}

    #[test]
    fn it_works() -> Result<(), Error> {
        let salt = b"some@email.com"; // Salt should be unique per password
        let password = b"some random words that you made up, for sure!";

        let mut output_key_material_1 = Seed::new(Zeroizing::new([0u8; 32]));
        let mut output_key_material_2 = Seed::default(); // default size is 32 bytes
        let mut output_key_material_3: Vec<u8> = vec![0; 48]; // non-zero length vectors are ok too

        Argon2::default().hash_password_into(password, salt, &mut output_key_material_1)?;
        Argon2::default().hash_password_into(password, salt, &mut output_key_material_2)?;
        Argon2::default().hash_password_into(password, salt, &mut output_key_material_3)?;

        assert_eq!(output_key_material_1, output_key_material_2);

        drop(output_key_material_1);
        drop(output_key_material_2);
        drop(output_key_material_3);

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
}
