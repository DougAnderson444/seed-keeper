# Seed Keeper Core

Seed Keeper Core is a small Rust library for managing a seed phrase and deriving keys from it.

## Usage

```rust
use seed_keeper_core::{rand_seed, Seed, Secret, ExposeSecret, derive_key};
use seed_keeper_core::wrap::{encrypt, decrypt};

// Generate a secure random seed of 32 bytes:

let seed: Secret<Seed> = rand_seed();
assert_eq!(seed.expose_secret().len(), 32);

// Derive key material from a username (salt) and password:

let password = "some random words that you made up, for sure!".to_string();
let salt = b"some@email.com"; // Salt should be unique per password

let key = derive_key(&password, salt).unwrap();

assert_eq!(
    key.expose_secret(),
    &Seed::new([
         164, 103, 254, 113, 126, 241, 57, 240, 100, 56, 243, 125, 155, 224, 40, 242, 178,
         136, 222, 133, 220, 141, 127, 10, 88, 199, 181, 11, 241, 91, 149, 249
     ])
);

// Protect your new seed by encrypting it with the password and salt key:

let encrypted = encrypt(
        (**key.expose_secret()).try_into().unwrap(), // Deref &Seed to [u8; 32]
        seed.expose_secret(),
    );
let decrypted = decrypt((**key.expose_secret()).try_into().unwrap(), &encrypted);
assert_eq!(**seed.expose_secret(), *decrypted.as_slice());
```
