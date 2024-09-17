use std::{marker::PhantomData, ops::Deref};

use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};

use crate::{
    derive_key,
    error::{self, Error},
    seed::rand_seed,
    wrap::{decrypt, encrypt},
};

/// Username, password and Option of Encrypted seed
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Credentials {
    pub username: MinString<8>,
    pub password: MinString<8>,
    pub encrypted_seed: Option<Vec<u8>>,
}

impl Credentials {
    pub fn new(
        username: &str,
        password: &str,
        encrypted_seed: Option<Vec<u8>>,
    ) -> Result<Self, error::Error> {
        Ok(Credentials {
            username: MinString::new(username)?,
            password: MinString::new(password)?,
            encrypted_seed,
        })
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Zeroize, ZeroizeOnDrop)]
pub struct MinString<const N: usize> {
    #[zeroize]
    value: String,
    _marker: PhantomData<()>,
}

impl<const N: usize> MinString<N> {
    pub fn new(value: &str) -> Result<Self, error::Error> {
        if value.len() >= N {
            Ok(MinString {
                value: value.to_string(),
                _marker: PhantomData,
            })
        } else {
            Err(Error::ValueTooShort(N))
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl<const N: usize> Deref for MinString<{ N }> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

pub struct Wallet {
    username: MinString<8>,
    password: MinString<8>,
    seed: Zeroizing<Vec<u8>>,
}

impl Wallet {
    /// Creates a new Wallet from the given credentials
    pub fn new(credentials: Credentials) -> Result<Self, error::Error> {
        let seed = match credentials.encrypted_seed {
            Some(encrypted_seed) => {
                let key = derive_key(
                    credentials.username.value().as_bytes(),
                    credentials.password.value().as_bytes(),
                )
                .map_err(|e| e.to_string())?;

                // Decrypt the given seed with the key, if it fails the username or password is wrong & return error
                let decrypted = decrypt(key.clone(), &encrypted_seed)?;

                if decrypted.len() != 32 {
                    return Err(Error::SeedLength);
                }

                decrypted
            }
            None => Zeroizing::new(rand_seed().as_slice().to_vec()),
        };

        Ok(Wallet {
            seed,
            username: credentials.username,
            password: credentials.password,
        })
    }

    /// Returns the encrypted seed
    pub fn encrypted_seed(&self) -> Result<Vec<u8>, error::Error> {
        let key = derive_key(
            self.username.value().as_bytes(),
            self.password.value().as_bytes(),
        )?;

        encrypt(key, self.seed.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), error::Error> {
        // Create a new wallet
        // [Credentials] supports Deserialization, so you can use it with serde_json from JavaScript
        let credentials = Credentials {
            username: MinString::<8>::new("username")?,
            password: MinString::<8>::new("password")?,
            encrypted_seed: None,
        };

        let wallet = Wallet::new(credentials)?;

        // Encrypt the seed
        let encrypted_seed = wallet.encrypted_seed()?;

        // Create a new wallet with the encrypted seed
        let credentials = Credentials {
            username: MinString::new("username")?,
            password: MinString::new("password")?,
            encrypted_seed: Some(encrypted_seed.clone()),
        };

        let wallet = Wallet::new(credentials)?;

        // Encrypt the seed
        let encrypted_seed_2 = wallet.encrypted_seed()?;

        assert!(!encrypted_seed_2.is_empty());

        // Should match
        assert_eq!(encrypted_seed, encrypted_seed_2);

        Ok(())
    }
}
