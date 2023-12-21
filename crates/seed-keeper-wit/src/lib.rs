#![feature(lazy_cell)]

cargo_component_bindings::generate!();

use crate::bindings::exports::component::wallet::{
    config::Guest as ConfigGuest, encrypted::Guest as SaverGuest, seed_getter::Guest as KeeperGuest,
};
use bindings::component::wallet::types::Credentials;

use seed_keeper_core::seed::rand_seed;
use seed_keeper_core::wrap::{decrypt, encrypt};
use seed_keeper_core::{derive_key, ExposeSecret};
use std::sync::OnceLock;
use std::sync::{LazyLock, Mutex};

/// The encrypted seed
static STORED_SEED: OnceLock<Vec<u8>> = OnceLock::new();

/// The current config of the seed keeper. Can be updated by the user as desired to set new username, password, and encrypted seed.
static CONFIG: LazyLock<Mutex<Option<Credentials>>> = LazyLock::new(|| Mutex::new(None));

struct Component;

impl KeeperGuest for Component {
    /// Returns the decryted seed.
    /// The STORED_SEED is not set, it randdomly generate sit, uses the username and password from the config function to store it encrypted, and returns the plaintext.
    /// If the STORED_SEED is set, it uses the username and password from the config function to decrypt it and returns the plaintext.
    fn get_seed() -> Result<Vec<u8>, String> {
        let (key, encrypted) = keys_from_config()?;
        let decrypted = decrypt(key, &encrypted).map_err(|e| e.to_string())?;
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

/// Sets the Config of the Seed-Keeper (username, password, and optionally encrypted seed)
impl ConfigGuest for Component {
    fn set_config(config: Credentials) -> Result<(), String> {
        // Set or update the CONFIG for this component
        // First check to ensure the username and passqord are at least 8 bytes long,
        // if they are not, return an error
        if config.username.len() < 8 || config.password.len() < 8 {
            return Err("Username and password must be at least 8 bytes long".to_string());
        }
        // If the encrypted seed is provided, check to ensure it is 40 bytes long,
        // if it is not, return an error
        if let Some(encrypted) = &config.encrypted {
            if encrypted.len() != 40 {
                return Err("Encrypted seed must be 40 bytes long".to_string());
            }
        }
        // Set the CONFIG for this component
        // This is a LazyLock, so it can be set multiple times, but only the first time will be used
        *CONFIG.lock().unwrap() = Some(config);

        Ok(())
    }
}

/// Gets the config from CONFIG, if it is set
fn get_config() -> Result<Credentials, String> {
    // Get the CONFIG for this component
    let config = CONFIG.lock().unwrap();
    // If the config is set, return it
    match &*config {
        Some(config) => Ok(config.clone()),
        // If the config is not set, return an error
        None => {
            Err("Set a config with username and password at least 8 bytes long first, using set_config()".to_string())
        }
    }
}

/// Gets and sets the key from config
fn keys_from_config() -> Result<([u8; 32], Vec<u8>), String> {
    // get username and password from config
    let config = get_config().map_err(|e| {
        format!(
            "Set a config with username and password at least 8 bytes long first {:?}",
            e
        )
    })?;
    let username = config.username;
    let pwd = config.password;
    let maybe_encrypted: Option<Vec<u8>> = config.encrypted;

    let key = derive(pwd, username)?;

    let encrypted = STORED_SEED.get_or_init(|| {
        // if config included an encrypted seed, use that. Else, use rand.
        maybe_encrypted.unwrap_or_else(|| {
            encrypt(key, &rand_seed())
                .map_err(|e| e.to_string())
                .unwrap()
        })
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
