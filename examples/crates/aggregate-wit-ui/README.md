# Aggregate WIT UI

This UI WIT component aggreagtes the seed keeper UI and the edwards UI together.

It is responsible for parsing the contexts, calling render with that context on the appropriate child UI, then sending that HTML to the DOM for rendering.

It's a traffic cop. All it does it parse and direct traffic to delegated components.

# Usage

Use this component as the primary component when composing the Wallet.

The seed keeper and edwards UIs are passed in as dependencies.

```bash
# from root dir
wasm-tools compose --config examples/crates/aggregate-wit-ui/config.yml -o examples/aggregate.wasm target/wasm32-wasi/release/aggregate_wit_ui.wasm

# from this dir
wasm-tools compose -c config.yml -o ../../aggregate.wasm ../../../target/wasm32-wasi/release/aggregate_wit_ui.wasm 
```
