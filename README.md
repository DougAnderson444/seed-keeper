# Seed Keeper Workspace

- [seed-keeper-core](crates/seed-keeper-core/): Core functionality for Seed generation & encryption.
- [seed-keeper-wallet](crates/seed-keeper-wallet/): Wasm Interface Type (WIT) Component for using Seed-Keeper-Core in Apps.
- [seed-keeper-wit-ui](crates/seed-keeper-wit-ui/): A User Interface built on top of Wasm Component Model for the Seed Keeper.

## Seed Keeper

The Seed Keeper is a tool for generating, encrypting and managing Seeds. It is used to generate, encrypt, decrypt, and view Seeds.

## Build

```bash
cargo component build --workspace --release
```

## Compose

Compose the `seed-keeper-wallet`, `seed-keeper-wit-ui`, and the seed consumers of your choice into a single Wasm component for use in an App using [`wasm-tools compose`](https://component-model.bytecodealliance.org/creating-and-consuming/composing.html):

```bash
wasm-tools compose ./target/wasm32-wasi/release/seed_keeper_wit_ui.wasm -d ./target/wasm32-wasi/release/seed_keeper_wit.wasm -o examples/composed-wallet.wasm
```

## Examples

We can compose together an Edwards25519 Signer and an Edwards25519 User Interface (UI) to create a mini wallet that can sign messages. If we start without the `seed-keeper` we would need to provide the import to get the seed each time (which we can do with JavaScript).

First we need to composed the Edwards Signer backend with the Edwards "input message & display signature" UI:

```bash
cargo component build --workspace --release
wasm-tools compose ./target/wasm32-wasi/release/edwards_ui.wasm -d ./target/wasm32-wasi/release/edwards_wit.wasm -o examples/edwards-only.wasm
```

Then we can load this edwards only composed component into our Svelte app, using a simple seed provider import.

We can see the interface for our new composed component by running:

```bash
wasm-tools component wit examples/edwards-only.wasm
```

Which shows our interfaces:

```bash
package root:component;

world root {
  import component:wurbo/wurbo-types;
  import component:wurbo/wurbo-in;  
  import component:wallet/config@0.1.0;

  export example:edwards-ui/wurbo-out;
}

```

## Tests

Run all tests:

```bash
cargo test --workspace --all-targets --exclude seed-keeper-wit-ui
```

We need to `--exclude` the `seed-keeper-wit-ui` from tests because it's `crate-type` is both `cdylib` (for `cargo component build` step) and `lib` (so that `events` can be exported), however this setup breaks `cargo test`.

---

# FUTURE PLANS:

## Plugins

A seed is not much good without functions that use the seed in a cryptographic fashion. The Seed Keeper is designed to be extensible, and plugins can be added to the Seed Keeper to provide additional functionality.

These plugins could be of Wasm Interface Types (WIT) components that are loaded into the App using the Seed Keeper. The Seed Keeper provides a way to load and unload plugins, and to call functions on the plugins.

## Build Your Own Wallet

If you have more than one app which uses a seed, you can compose these WIT components together with the Seed Keeper to create a wallet that can be used to manage multiple apps from the same wallet.

Each App should bring it's own User Interface, otherwise the Seed Keeper would have to build each conceivable UI for each app. The Seed Keeper provides the functionality, and the App provides the UI and usage implementation.

Another benefit is that the Seed Keeper staying lightwieght and loads quickly, and each app component is loaded asynchronusly as needed.
