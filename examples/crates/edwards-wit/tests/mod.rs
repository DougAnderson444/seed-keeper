//! A module to test the edwards-wit component in Rust.
//!
//! Note: In order for this to run, we need to include the WIT dependencies in ./wit/deps/*,
//! which is copy and paste from the source directory.
mod bindgen {
    wasmtime::component::bindgen!("example"); // name of the world in the .wit file
}

use serde::{Deserialize, Serialize};
use std::{
    env,
    path::{Path, PathBuf},
};
use thiserror::Error;
use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::preview2::{Table, WasiCtx, WasiCtxBuilder, WasiView};

struct MyCtx {
    wasi_ctx: Context,
}

struct Context {
    table: Table,
    wasi: WasiCtx,
}
impl WasiView for MyCtx {
    fn table(&self) -> &Table {
        &self.wasi_ctx.table
    }
    fn table_mut(&mut self) -> &mut Table {
        &mut self.wasi_ctx.table
    }
    fn ctx(&self) -> &WasiCtx {
        &self.wasi_ctx.wasi
    }
    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx.wasi
    }
}

/// Implementing this trait gives us
/// - the ability to add_to_linker using SeedKeeper::add_to_linker
/// - call get_seed from inside out component
///
/// Normally this would be implemented by another WIT component that is composed with this
/// component, but for testing we mock it up below.
impl bindgen::seed_keeper::wallet::seed_getter::Host for MyCtx {
    fn get_seed(&mut self) -> Result<Result<Vec<u8>, String>, wasmtime::Error> {
        let seed = vec![1u8; 32];
        Ok(Ok(seed))
    }
}

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

impl From<String> for TestError {
    fn from(s: String) -> Self {
        TestError::Stringified(s)
    }
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
mod edwards_example_wit_tests {

    use super::*;

    #[test]
    fn test_roundtrip_sign_and_verify() -> wasmtime::Result<(), TestError> {
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
        bindgen::Example::add_to_linker(&mut linker, |state: &mut MyCtx| state)?;
        // link the WASI imports to our instantiation
        wasmtime_wasi::preview2::command::sync::add_to_linker(&mut linker)?;

        let table = Table::new();
        let wasi: WasiCtx = WasiCtxBuilder::new().inherit_stdout().args(&[""]).build();
        let state = MyCtx {
            wasi_ctx: Context { table, wasi },
        };
        let mut store = Store::new(&engine, state);

        let (bindings, _) = bindgen::Example::instantiate(&mut store, &component, &linker)?;

        // use bindings to sign a message
        let message = b"hello world";
        let signature = bindings
            .component_edwards_wit_operations()
            .call_sign(&mut store, message)??;

        // use bindings to verify the signature
        let is_valid = bindings
            .component_edwards_wit_operations()
            .call_verify(&mut store, message, &signature)??;

        assert!(is_valid);

        Ok(())
    }
}
