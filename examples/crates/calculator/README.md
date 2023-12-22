Compose this together with the math:adder by 

```bash
wasm-tools compose ./target/wasm32-wasi/release/calculator.wasm -d ./target/wasm32-wasi/release/math.wasm -o ./examples/composed-calculator.wasm
wasm-tools component wit ./examples/composed-calculator.wasm
```

Then we can load this composed component into our Svelte app, using a simple seed provider import.
