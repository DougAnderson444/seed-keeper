use std::{marker::PhantomData, ops::Deref};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
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

#[derive(Debug, Zeroize, ZeroizeOnDrop)]
pub struct MinString<const N: usize> {
    #[zeroize]
    value: String,
    _marker: PhantomData<()>,
}

// MinString can be referenced as a string and str
impl<const N: usize> AsRef<str> for MinString<N> {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl<const N: usize> AsRef<String> for MinString<N> {
    fn as_ref(&self) -> &String {
        &self.value
    }
}

impl<const N: usize> AsMut<str> for MinString<N> {
    fn as_mut(&mut self) -> &mut str {
        &mut self.value
    }
}

impl<const N: usize> AsMut<String> for MinString<N> {
    fn as_mut(&mut self) -> &mut String {
        &mut self.value
    }
}

impl<const N: usize> std::fmt::Display for MinString<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl<const N: usize> Serialize for MinString<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.value)
    }
}
impl<'de, const N: usize> Deserialize<'de> for MinString<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        MinString::new(&value).map_err(serde::de::Error::custom)
    }
}

impl Default for MinString<8> {
    fn default() -> Self {
        MinString {
            // a string 8 characters long
            value: String::from(" ".repeat(8)),
            _marker: PhantomData,
        }
    }
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

    /// Returns the seed, decrypted
    pub fn seed(&self) -> &[u8] {
        &self.seed
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

        println!("{:?}", credentials);

        let wallet = Wallet::new(credentials)?;

        // Encrypt the seed
        let encrypted_seed_2 = wallet.encrypted_seed()?;

        assert!(!encrypted_seed_2.is_empty());

        // Should match
        assert_eq!(encrypted_seed, encrypted_seed_2);

        Ok(())
    }

    #[test]
    fn test_json_credentials_roundtrip() -> Result<(), error::Error> {
        let credentials = Credentials {
            username: MinString::new("username")?,
            password: MinString::new("password")?,
            encrypted_seed: None,
        };

        let json = serde_json::to_string(&credentials).map_err(|e| e.to_string())?;

        let credentials: Credentials = serde_json::from_str(&json).map_err(|e| e.to_string())?;

        assert_eq!(credentials.username.value(), "username");
        assert_eq!(credentials.password.value(), "password");

        Ok(())
    }

    #[test]
    fn it_fails_too_short() -> Result<(), String> {
        let json = r#"{"username":"user","password":"pass","encrypted_seed":null}"#;

        // it should deserialize the json
        let credentialss: Result<Credentials, _> = serde_json::from_str(json);

        assert!(credentialss.is_err());

        Ok(())
    }

    #[test]
    fn test_works_long_enough() -> Result<(), error::Error> {
        let json = r#"{"username":"username","password":"password","encrypted_seed":null}"#;

        // it should deserialize the json
        let credentials: Credentials = serde_json::from_str(json).map_err(|e| e.to_string())?;

        assert_eq!(credentials.username.value(), "username");
        assert_eq!(credentials.password.value(), "password");

        Ok(())
    }

    #[test]
    fn test_works_with_seed() -> Result<(), error::Error> {
        let json = r#"{"username":"username","password":"password","encrypted_seed":[46,236,62,136,201,70,17,15,212,216,99,70,0,242,150,190,15,58,71,131,148,196,18,158,104,110,121,170,241,22,47,63,211,192,118,233,214,196,223,34]}"#;

        // it should deserialize the json
        let credentials: Credentials = serde_json::from_str(json).map_err(|e| e.to_string())?;

        assert_eq!(credentials.username.value(), "username");
        assert_eq!(credentials.password.value(), "password");

        Ok(())
    }

    // test default MinString
    #[test]
    fn test_default_credentials() -> Result<(), error::Error> {
        let len = 8;

        let username = MinString::<8>::default();
        let password = MinString::<8>::default();

        assert_eq!(username.len(), len);
        assert_eq!(password.len(), len);

        let cred = Credentials::default();

        assert_eq!(cred.username.len(), len);
        assert_eq!(cred.password.len(), len);

        Ok(())
    }
}
