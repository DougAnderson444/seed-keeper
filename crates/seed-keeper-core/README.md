# Seed Keeper Core

Seed Keeper Core is a small Rust library for deriving and encrypting keys and seeds.

Uses Argon2, AES Key Encryption Keys,

- [x] Derive a key from username & password (salt & passphrase).
- [x] Generate a random seed that zeroizes memory by default.
- [x] Encrypt the seed with the key, and decrypt.

## Roundtrip Usage

```rust
use seed_keeper_core::{derive_key}; // the main purpose of this library
use seed_keeper_core::wrap::{encrypt, decrypt}; // utils to encrypt and decrypt the seed
use seed_keeper_core::seed::{Seed, rand_seed}; // utils to generate a random seed
use seed_keeper_core::Zeroizing;

// Generate a secure random seed of 32 bytes:
let seed = rand_seed();
assert_eq!(seed.len(), 32);

// Derive key material from a username (salt) and password:

let password = "some random words that you made up, for sure!".to_string();
let salt = b"some@email.com"; // Salt should be unique per password

let key = derive_key(&password, salt).unwrap();

assert_eq!(
    key.as_ref(),
    [
         164, 103, 254, 113, 126, 241, 57, 240, 100, 56, 243, 125, 155, 224, 40, 242, 178,
         136, 222, 133, 220, 141, 127, 10, 88, 199, 181, 11, 241, 91, 149, 249
     ]
);

// Protect your new seed by encrypting it with the password and salt key:

let encrypted = encrypt(key.clone(), seed.clone()).unwrap();
let decrypted = decrypt(key.clone(), &encrypted).unwrap();
assert_eq!(*seed, *decrypted.as_slice());
```

## Full Credentials and Storage

The inputs for Aegon2 must be at least 8 characters long strings, so seed-keeper exports helpers to help ensure that your users input the minimum length bytes, and give helpful errors when they don't. To use these helpers, just use the `credentials` module.

```rust 
use seed_keeper_core::error;
use seed_keeper_core::credentials::{MinString, Credentials, Wallet};

fn it_works() -> Result<(), error::Error> {
    // Create a new wallet
    // [Credentials] supports Deserialization, so you can use it with serde_json from JavaScript
    let credentials = Credentials {
        username: MinString::new("username")?,
        password: MinString::new("password")?,
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
```
