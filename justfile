build:
  cargo component build --workspace
  cargo component build --workspace --release

compose: build
  wasm-tools compose --config examples/crates/aggregate-wit-ui/config.yml -o examples/aggregate.wasm target/wasm32-wasi/release/aggregate_wit_ui.wasm

preview: compose
  cd examples/sveltekit && npm run build && npm run preview -- --open

