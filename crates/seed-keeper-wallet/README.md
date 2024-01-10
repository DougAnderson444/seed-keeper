# Seed Keeper WIT Component

WIT Wasm Component for Seed Keeper, meant to be composed with users of the seed keeper to build your own Wasm WIT Wallet (WWW).

# Build

Built with [`cargo component`](https://github.com/bytecodealliance/cargo-component).

From this directory, run:

```bash
cargo component build --release
```

which will place the built `.wasm` file in `target/wasm32-unknown-unknown/release/seed_keeper_wit.wasm`.

# Test

```bash
cargo component build
cargo test
```

