mod bindgen {
    wasmtime::component::bindgen!("keeper"); // name of the world in the .wit file
}

use bindgen::exports::seed_keeper::wallet::config;

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use std::{
    env,
    path::{Path, PathBuf},
};
use thiserror::Error;
use wasmtime::component::{Component, Linker, ResourceTable};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::preview2::{WasiCtx, WasiCtxBuilder, WasiView};

struct MyCtx {
    wasi_ctx: Context,
}

struct Context {
    table: ResourceTable,
    wasi: WasiCtx,
}
impl WasiView for MyCtx {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.wasi_ctx.table
    }
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx.wasi
    }
}

/// The encrypted key
static ENCRYPTED_KEY: OnceLock<Vec<u8>> = OnceLock::new();

/// Implementing this trait gives us th ability to add_to_linker using SeedKeeper::add_to_linker
impl bindgen::seed_keeper::wallet::types::Host for MyCtx {}

#[derive(Error, Debug)]
pub enum TestError {
    /// From String
    #[error("Error message {0}")]
    Stringified(String),

    /// From Wasmtime
    #[error("Wasmtime: {0}")]
    Wasmtime(#[from] wasmtime::Error),

    /// From VarError
    #[error("VarError: {0}")]
    VarError(#[from] std::env::VarError),

    /// From io
    #[error("IO: {0}")]
    Io(#[from] std::io::Error),

    /// From serde_json
    #[error("Serde JSON: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

#[derive(Debug, Serialize, Deserialize)]
struct TestFixtures {
    seed: Vec<u8>,
    encrypted: Vec<u8>,
    username: Vec<u8>,
    password: Vec<u8>,
}

/// Utility function to get the workspace dir
pub fn workspace_dir() -> PathBuf {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;
    let cargo_path = Path::new(std::str::from_utf8(&output).unwrap().trim());
    cargo_path.parent().unwrap().to_path_buf()
}

#[cfg(test)]
mod wit_tests {

    use super::*;

    #[test]
    fn test_random() -> wasmtime::Result<(), TestError> {
        // get the target/wasm32-wasi/debug/CARGO_PKG_NAME.wasm file
        let pkg_name = std::env::var("CARGO_PKG_NAME")?.replace('-', "_");
        let workspace = workspace_dir();
        let wasm_path = format!("target/wasm32-wasi/debug/{}.wasm", pkg_name);
        let wasm_path = workspace.join(wasm_path);

        let mut config = Config::new();
        config.cache_config_load_default()?;
        config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
        config.wasm_component_model(true);

        let engine = Engine::new(&config)?;
        let component = Component::from_file(&engine, &wasm_path)?;

        let mut linker = Linker::new(&engine);
        // link imports like get_seed to our instantiation
        bindgen::Keeper::add_to_linker(&mut linker, |state: &mut MyCtx| state)?;
        // link the WASI imports to our instantiation
        wasmtime_wasi::preview2::command::sync::add_to_linker(&mut linker)?;

        let table = ResourceTable::new();
        let wasi: WasiCtx = WasiCtxBuilder::new().inherit_stdout().args(&[""]).build();
        let state = MyCtx {
            wasi_ctx: Context { table, wasi },
        };
        let mut store = Store::new(&engine, state);

        let (bindings, _) = bindgen::Keeper::instantiate(&mut store, &component, &linker)?;

        // First, call set_config with Credentials with username and password of 8 bytes each
        let config = config::Credentials {
            username: b"username1".to_vec(),
            password: b"password1".to_vec(),
            encrypted: None,
        };

        bindings
            .seed_keeper_wallet_config()
            .call_set_config(&mut store, &config)?
            .unwrap();

        // Now let's call the functions!
        // First, generate the seed by keeping the Credentials encrypted field as None
        let seed = bindings
            .seed_keeper_wallet_config()
            .call_get_seed(&mut store)?
            .unwrap();

        // seed should be 32 bytes
        assert_eq!(seed.len(), 32);
        // those bytes should be not be all zeros
        assert!(!seed.iter().all(|&x| x == 0));

        // Now get the encrypted seed
        let encrypted = bindings
            .seed_keeper_wallet_config()
            .call_get_encrypted(&mut store)?
            .unwrap();

        // encrypted should be 40 bytes of not all zeros
        assert_eq!(encrypted.len(), 40);
        assert!(!encrypted.iter().all(|&x| x == 0));

        Ok(())
    }

    #[test]
    fn test_deterministic() -> wasmtime::Result<(), TestError> {
        let cwd = std::env::current_dir().unwrap();
        let path = cwd.join("tests/fixtures/deterministic.json");
        let contents = std::fs::read_to_string(path).unwrap();
        let fixtures: TestFixtures = serde_json::from_str(&contents).unwrap();

        // set ENCRYPTED_KEY to the encrypted key from the fixtures
        let encr = ENCRYPTED_KEY.get_or_init(|| fixtures.encrypted.clone());

        // get the target/wasm32-wasi/debug/CARGO_PKG_NAME.wasm file
        let pkg_name = std::env::var("CARGO_PKG_NAME").unwrap().replace('-', "_");
        let workspace = workspace_dir();
        let wasm_path = format!("target/wasm32-wasi/debug/{}.wasm", pkg_name);
        let wasm_path = workspace.join(wasm_path);

        let mut config = Config::new();
        config.cache_config_load_default().unwrap();
        config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
        config.wasm_component_model(true);

        let engine = Engine::new(&config).unwrap();
        let component = Component::from_file(&engine, &wasm_path).unwrap();

        let mut linker = Linker::new(&engine);
        // link imports like get_seed to our instantiation
        bindgen::Keeper::add_to_linker(&mut linker, |state: &mut MyCtx| state).unwrap();
        // link the WASI imports to our instantiation
        wasmtime_wasi::preview2::command::sync::add_to_linker(&mut linker).unwrap();

        let table = ResourceTable::new();
        let wasi: WasiCtx = WasiCtxBuilder::new().inherit_stdout().args(&[""]).build();
        let state = MyCtx {
            wasi_ctx: Context { table, wasi },
        };
        let mut store = Store::new(&engine, state);

        let (bindings, _) = bindgen::Keeper::instantiate(&mut store, &component, &linker).unwrap();

        // First, call set_config with encr as the encrypted seed
        let config = config::Credentials {
            username: fixtures.username,
            password: fixtures.password,
            encrypted: Some(encr.clone()),
        };

        bindings
            .seed_keeper_wallet_config()
            .call_set_config(&mut store, &config)?
            .unwrap();

        let seed = bindings
            .seed_keeper_wallet_config()
            .call_get_seed(&mut store)?
            .unwrap();

        // Should match fixures.seed
        assert_eq!(seed, fixtures.seed);

        // Now let's call the functions!
        // First, generate the seed by keeping the Credentials encrypted field as None
        let encrypted = bindings
            .seed_keeper_wallet_config()
            .call_get_encrypted(&mut store)?
            .unwrap();

        // should match fixtures.encrypted
        assert_eq!(encrypted, fixtures.encrypted);
        // should also match encr
        assert_eq!(encrypted, *encr);

        Ok(())
    }
}
