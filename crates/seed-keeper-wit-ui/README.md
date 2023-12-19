# Seed Keeper WIT UI

A User Interface (UI) for Seed Keeper WIT build with WebAssebly, Wurbo (minijinja), and HTML templates. 

HTML templates can be composed into a larger application for code re-use.

# Development

Wurbo 0.1.1 requires `Page`, `Input` and `Output` contexts.

# Build

Built with [`cargo component`](https://github.com/bytecodealliance/cargo-component):

```bash
cargo component build --release
```

# Use

To be useful, you compose the User Interface with the seed-keeper-wit component using `wasm-tools`
