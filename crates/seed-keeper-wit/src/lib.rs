cargo_component_bindings::generate!();

use crate::bindings::exports::seed_keeper::wallet::{
    encrypted::Guest as SaverGuest, seed_getter::Guest as KeeperGuest,
};
use bindings::seed_keeper::wallet::config::get_config;

// use std::collections::HashMap;
// use std::sync::LazyLock;
use std::sync::OnceLock;

use seed_keeper_core::seed::rand_seed;
use seed_keeper_core::wrap::{decrypt, encrypt};
use seed_keeper_core::{derive_key, ExposeSecret};

/// The encrypted seed
static STORED_SEED: OnceLock<Vec<u8>> = OnceLock::new();

struct Component;

impl KeeperGuest for Component {
    /// Returns the decryted seed.
    /// The STORED_SEED is not set, it randdomly generate sit, uses the username and password from the config function to store it encrypted, and returns the plaintext.
    /// If the STORED_SEED is set, it uses the username and password from the config function to decrypt it and returns the plaintext.
    fn get_seed() -> Result<Vec<u8>, String> {
        let (key, encrypted) = keys_from_config()?;
        let decrypted = decrypt(key, &encrypted);
        Ok(decrypted)
    }
}

impl SaverGuest for Component {
    /// Returns the encrypted seed. Stores it in STORED_SEED if it is not already set.
    fn get_encrypted() -> Result<Vec<u8>, String> {
        let (_, encrypted) = keys_from_config()?;
        Ok(encrypted)
    }
}

/// Gets and sets the key from config
fn keys_from_config() -> Result<([u8; 32], Vec<u8>), String> {
    // get username and password from config
    let config = get_config().map_err(|e| {
        format!(
            "Provide a config with username and password at least 8 bytes long as an import function {:?}",
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

    Ok((key, encrypted.to_vec()))
}

/// Derive 32 bytes key from password and username
fn derive(pwd: Vec<u8>, username: Vec<u8>) -> Result<[u8; 32], String> {
    let derived_key = derive_key(pwd, username).map_err(|_| {
        "Failed to derive key from pasword and salt (username). Are they at least 8 bytes long?"
    })?;
    let key: [u8; 32] = (**derived_key.expose_secret())
        .try_into()
        .map_err(|_| "Failed to convert key to seed")?;

    Ok(key)
}
