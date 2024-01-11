# Seed Keeper Utilities

These are optional utilities that don't need to be in the core crate but also cannot be in the wit-component crate because they are used by other libraries.

For example, the `Event` enum is shared by both the `seed-keeper-wit-ui` and any listener for that event from other libraries. In order to allows other libraries to import the same type, it needs to reside outside the component crate. So it resides here instead.
