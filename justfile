build-examples:
  for dir in examples/crates/*; do \
    if [ -f $dir/Cargo.toml ]; then \
      cargo component build --manifest-path=$dir/Cargo.toml --workspace; \
      cargo component build --release --manifest-path=$dir/Cargo.toml --workspace; \
    fi \
  done

build: build-examples
  cargo component build --manifest-path=crates/seed-keeper-wallet/Cargo.toml
  cargo component build --manifest-path=crates/seed-keeper-wit-ui/Cargo.toml
  cargo component build --manifest-path=crates/seed-keeper-wallet/Cargo.toml --release
  cargo component build --manifest-path=crates/seed-keeper-wit-ui/Cargo.toml --release

compose: build
  wasm-tools compose --config examples/crates/aggregate-wit-ui/config.yml -o examples/aggregate.wasm target/wasm32-wasi/release/aggregate_wit_ui.wasm

preview: compose
  cd examples/sveltekit && npm run build && npm run preview -- --open

