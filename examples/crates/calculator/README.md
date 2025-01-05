Compose this together with the math:adder by 

```bash
wasm-tools compose ./target/wasm32-wasip1/release/calculator.wasm -d ./target/wasm32-wasip1/release/math.wasm -o ./examples/composed-calculator.wasm
wasm-tools component wit ./examples/composed-calculator.wasm
```

or, using a config file:

```bash
wasm-tools compose --config config.yml -o ./examples/composed-calculator.wasm ../../../target/wasm32-wasip1/release/calculator.wasm
```

Then we can load this composed component into our Svelte app, using a simple seed provider import.
