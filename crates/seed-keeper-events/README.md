# Seed Keeper Events

This code gets it's own crate because it's used by both the Seed Keeper WIT UI and other WIT UI crates that listen on these emitted events. Thus it need to have `crate-type = ["lib"]` which clashes with cdylib when `cargo test` is run.

## Base64 URL Unpadded

The Protocol is to use Base64 URL unpadded strings for bytes arrays.

When passing byte arrays between WebAssembly and JavaScript (and back!) there is a conversion issue between plain Arrays and TypedArrays. To avoid this issue, we convert the byte array to a base64 URL unpadded string. The reason we use Base64Url is so that data can be passed via URL, see [RFC 4648](https://www.rfc-editor.org/rfc/rfc4648).
