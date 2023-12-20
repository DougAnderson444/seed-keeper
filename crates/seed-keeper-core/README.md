# Seed Keeper Core

Seed Keeper Core is a small Rust library for deriving and encrypting keys and seeds.

Uses Argon2, AES Key Encryption Keys,

- [x] Derive a key from username & password (salt & passphrase).
- [x] Generate a random seed that zeroizes memory by default.
- [x] Encrypt the seed with the key, and decrypt.

## Roundtrip Usage

```rust
use seed_keeper_core::{derive_key}; // the main purpose of this library
use seed_keeper_core::{Secret, ExposeSecret}; // re-exports to use the derived key
use seed_keeper_core::wrap::{encrypt, decrypt}; // utils to encrypt and decrypt the seed
use seed_keeper_core::seed::{Seed, rand_seed}; // utils to generate a random seed

// Generate a secure random seed of 32 bytes:
let seed: Seed = rand_seed();
assert_eq!(seed.len(), 32);

// Derive key material from a username (salt) and password:

let password = "some random words that you made up, for sure!".to_string();
let salt = b"some@email.com"; // Salt should be unique per password

let key = derive_key(&password, salt).unwrap();

assert_eq!(
    **key.expose_secret(),
    [
         164, 103, 254, 113, 126, 241, 57, 240, 100, 56, 243, 125, 155, 224, 40, 242, 178,
         136, 222, 133, 220, 141, 127, 10, 88, 199, 181, 11, 241, 91, 149, 249
     ]
);

// Protect your new seed by encrypting it with the password and salt key:

let encrypted = encrypt(
        (**key.expose_secret()).try_into().unwrap(), // Deref &Seed to [u8; 32]
        &seed,
    ).unwrap();
let decrypted = decrypt((**key.expose_secret()).try_into().unwrap(), &encrypted).unwrap();
assert_eq!(*seed, *decrypted.as_slice());
```
