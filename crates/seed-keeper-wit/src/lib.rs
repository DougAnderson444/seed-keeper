#![feature(lazy_cell)]

cargo_component_bindings::generate!();

use crate::bindings::exports::seed_keeper::wallet::seed_keeper::Guest;
use bindings::seed_keeper::wallet::seed_imports::get_config;

// use std::collections::HashMap;
// use std::sync::LazyLock;
use std::sync::OnceLock;

use seed_keeper_core::seed::rand_seed;
use seed_keeper_core::wrap::{decrypt, encrypt};
use seed_keeper_core::{derive_key, ExposeSecret};

/// The encrypted seed
static STORED_SEED: OnceLock<Vec<u8>> = OnceLock::new();

struct Component;

impl Guest for Component {
    /// Returns the decryted seed.
    /// The STORED_SEED is not set, it randdomly generate sit, uses the username and password from the config function to store it encrypted, and returns the plaintext.
    /// If the STORED_SEED is set, it uses the username and password from the config function to decrypt it and returns the plaintext.
    fn get_seed() -> Result<Vec<u8>, String> {
        // get username and password from config
        let config = get_config().map_err(|e| {
            format!(
                "Provide a config with username and password as an import function {:?}",
                e
            )
        })?;
        let username = config.username;
        let pwd = config.password;
        let maybe_encrypted: Option<Vec<u8>> = config.encrypted;

        let key = derive(pwd, username)?;

        let encrypted = STORED_SEED.get_or_init(|| {
            // if config included an encrypted seed, use that. Else, use rand.
            maybe_encrypted.unwrap_or_else(|| encrypt(key, &rand_seed()))
        });
        let decrypted = decrypt(key, &encrypted);

        Ok(decrypted)
    }
}

/// Derive 32 bytes key from password and username
fn derive(pwd: Vec<u8>, username: Vec<u8>) -> Result<[u8; 32], String> {
    let derived_key = derive_key(pwd, username)
        .map_err(|_| "Failed to derive key from pasword and salt (username)")?;
    let key: [u8; 32] = (**derived_key.expose_secret())
        .try_into()
        .map_err(|_| "Failed to convert key to seed")?;

    Ok(key)
}
