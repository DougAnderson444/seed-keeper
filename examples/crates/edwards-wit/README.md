# Example Plugin

Usign Edwards curve to sign a message and verify the signature.

## Create a Plugin WIT Component

1. Create a component using [`cargo component`](https://github.com/bytecodealliance/cargo-component):

```bash
cargo component new --reactor <name>
```

2. Import the seed keeper get seed interface

```wit
world yourworld {

    import seed-keeper:wallet/config@0.1.0;

    // the rest of your WIT world
    export operations;
}
```

3. Add the path to the wallet wit file in your Cargo.toml. This is just the interface, not the implementation of it.

```toml
[package.metadata.component.target.dependencies]
"seed-keeper:wallet" = { path = "../path/to/seed-keeper-wallet/wit" }  # directory containing the WIT package
```

4. Export an interface which defines a `sign` func which takes a message and returns a signature.

```wit
/// WIT interface operations exported by yourworld 
interface operations {
    sign: func(message: list<u8>) -> list<u8>;
    verify: func(message: list<u8>, signature: list<u8>) -> bool;
}
```

## Compose

Compose this plugin together with the `seed-keeper-wallet` and the `seed-keeper-wit-ui` components.
