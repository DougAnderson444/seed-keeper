# Seed Manager Workspace

- [seed-keeper-core](crates/seed-keeper-core/README.md): Core functionality for Seed generation & encryption.
- [seed-keeper-wit](crates/seed-site/README.md): (Coming soon) Wasm Interface Types for using Seed-Keeper-Core in WIT Components.

## Seed Manager

The Seed Manager is a tool for generating, encrypting and managing Seeds. It is used to generate, encrypt, decrypt, and view Seeds.

## Tests

Run all tests:

```bash
cargo test --workspace --all-targets
```

<------------------------------------------------------------------------------------------------------------------------>

# FUTURE PLANS:

## Plugins

A seed is not much good without functions that use the seed in a cryptographic fashion. The Seed Manager is designed to be extensible, and plugins can be added to the Seed Manager to provide additional functionality.

These plugins are Wasm Interface Types (WIT) components that are loaded into the App using the Seed Manager. The Seed Manager provides a way to load and unload plugins, and to call functions on the plugins.

## Build Your Own Wallet

If you have more than one app which uses a seed, you can compose these WIT components together with the Seed Keeper to create a wallet that can be used to manage multiple apps from the same wallet.

Each App should bring it's own User Interface, otherwise the Seed Manager would have to build each conceivable UI for each app. The Seed Manager provides the functionality, and the App provides the UI.

Another benefit is that the Seed Keeper staying lightwieght and loads quickly, and each app component is loaded asynchronusly as needed.
