//! A module to test the edwards-wit component in Rust.
//!
//! Note: In order for this to run, we need to include the WIT dependencies in ./wit/deps/*,
//! which is copy and paste from the source directory.
mod bindgen {
    wasmtime::component::bindgen!("agg");
}

use std::{
    env,
    path::{Path, PathBuf},
};
use thiserror::Error;
use wasmtime::component::{Component, Linker, ResourceTable};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::preview2::{WasiCtx, WasiCtxBuilder, WasiView};

use bindgen::exports::wallet::aggregate_wit_ui::wurbo_out::Context as AggregateWitUiContext;
use bindgen::wallet::aggregate_wit_ui;

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

/// Implementing this trait gives us
/// - the ability to add_to_linker using SeedKeeper::add_to_linker
/// - call get_seed from inside out component
///
/// Normally this would be implemented by another WIT component that is composed with this
/// component, but for testing we mock it up below.
impl bindgen::example::edwards_ui::wurbo_types::Host for MyCtx {}

impl bindgen::example::edwards_ui::wurbo_out::Host for MyCtx {
    fn render(
        &mut self,
        _ctx: bindgen::example::edwards_ui::wurbo_out::Context,
    ) -> wasmtime::Result<Result<String, String>> {
        // return some html as string
        Ok(Ok("edwards ui for testing".to_string()))
    }

    fn activate(&mut self, _selectors: Option<Vec<String>>) -> wasmtime::Result<()> {
        Ok(())
    }

    fn customize(
        &mut self,
        _templates: Vec<(String, String)>,
    ) -> wasmtime::Result<Result<(), String>> {
        Ok(Ok(()))
    }
}

impl bindgen::seed_keeper::wit_ui::wurbo_types::Host for MyCtx {}

impl bindgen::seed_keeper::wit_ui::wurbo_out::Host for MyCtx {
    fn render(
        &mut self,
        _ctx: bindgen::seed_keeper::wit_ui::wurbo_out::Context,
    ) -> wasmtime::Result<Result<String, String>> {
        // return some html as string
        Ok(Ok("seed keeper ui for testing".to_string()))
    }

    fn activate(&mut self, _selectors: Option<Vec<String>>) -> wasmtime::Result<()> {
        Ok(())
    }

    fn customize(
        &mut self,
        _templates: Vec<(String, String)>,
    ) -> wasmtime::Result<Result<(), String>> {
        Ok(Ok(()))
    }
}

impl bindgen::wallet::aggregate_wit_ui::wurbo_types::Host for MyCtx {}

impl bindgen::wallet::aggregate_wit_ui::wurbo_in::Host for MyCtx {
    fn addeventlistener(
        &mut self,
        _details: bindgen::wallet::aggregate_wit_ui::wurbo_in::ListenDetails,
    ) -> wasmtime::Result<()> {
        Ok(())
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
}

impl From<String> for TestError {
    fn from(s: String) -> Self {
        TestError::Stringified(s)
    }
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
mod aggregate_example_tests {

    use crate::bindgen::{
        seed_keeper::wit_ui::wurbo_types::Page,
        wallet::aggregate_wit_ui::wurbo_types::{EdwardsContext, SeedContext},
    };

    use super::*;

    #[test]
    fn test_initial_load() -> wasmtime::Result<(), TestError> {
        let pkg_name = std::env::var("CARGO_PKG_NAME")?.replace('-', "_");
        let workspace = workspace_dir();
        let wasm_path = format!("target/wasm32-wasip1/debug/{}.wasm", pkg_name);
        let wasm_path = workspace.join(wasm_path);

        let mut config = Config::new();
        config.cache_config_load_default()?;
        config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
        config.wasm_component_model(true);

        let engine = Engine::new(&config)?;
        let component = Component::from_file(&engine, &wasm_path)?;

        let mut linker = Linker::new(&engine);
        // link imports like get_seed to our instantiation
        bindgen::Agg::add_to_linker(&mut linker, |state: &mut MyCtx| state)?;
        // link the WASI imports to our instantiation
        wasmtime_wasi::preview2::command::sync::add_to_linker(&mut linker)?;

        let table = ResourceTable::new();
        let wasi: WasiCtx = WasiCtxBuilder::new().inherit_stdout().args(&[""]).build();
        let state = MyCtx {
            wasi_ctx: Context { table, wasi },
        };
        let mut store = Store::new(&engine, state);

        let (bindings, _) = bindgen::Agg::instantiate(&mut store, &component, &linker)?;

        // Use bindings
        // Call render with initial data, should return all HTML
        // Initial data is WIT variant represented in JSON:
        // Now we should be able to pass this content as context to render()

        let seed_ui = SeedContext::AllContent(bindgen::seed_keeper::wit_ui::wurbo_types::Content {
            page: Some(Page {
                title: "a title for the page".to_string(),
            }),
            input: Some(bindgen::seed_keeper::wit_ui::wurbo_types::Input {
                placeholder: "a placeholder".to_string(),
                encrypted_seed: None,
            }),
            load: None,
        });

        let edwards_ui =
            EdwardsContext::AllContent(bindgen::example::edwards_ui::wurbo_types::Content {
                page: bindgen::example::edwards_ui::wurbo_types::Page {
                    title: "a title for the page".to_string(),
                },
                input: bindgen::example::edwards_ui::wurbo_types::Input {
                    placeholder: "a placeholder".to_string(),
                },
                output: None,
            });
        let all_context =
            AggregateWitUiContext::AllContent(aggregate_wit_ui::wurbo_types::Content {
                app: aggregate_wit_ui::wurbo_types::App {
                    title: "a title for the app".to_string(),
                },
                seed_ui: seed_ui.clone(),
                edwards_ui: edwards_ui.clone(),
            });

        let result = bindings
            .wallet_aggregate_wit_ui_wurbo_out()
            .call_render(&mut store, &all_context)?
            .expect("render should work in success tests");

        // The result should be a string of HTML
        eprintln!("result: {}", result);

        // should be able to also call aggregation_activate()
        bindings
            .wallet_aggregate_wit_ui_aggregation()
            .call_activates(&mut store, None)?;

        // now pass
        // only seed_ui as context for render, should get only seed UI HTML
        let result = bindings
            .wallet_aggregate_wit_ui_wurbo_out()
            .call_render(&mut store, &AggregateWitUiContext::Seed(seed_ui))?
            .expect("render should work in success tests");

        eprintln!("seed_ui result: {}", result);

        // same with edwards
        let result = bindings
            .wallet_aggregate_wit_ui_wurbo_out()
            .call_render(&mut store, &AggregateWitUiContext::Edwards(edwards_ui))?
            .expect("render should work in success tests");

        eprintln!("edwards_ui result: {}", result);

        Ok(())
    }
}
