//! Events which can be emitted from this component.
//! Match on the received json string to listen for these events.

/// Events which can be emitted from this component.
/// If the `serde` feature is enabled, these events will be serialized
/// with an adjacent `tag` and `val` to match the style that wit-component uses,
/// so that when this event reaches the Context Router in `wurbo`, it can be routed accordingly.
/// See <https://serde.rs/enum-representations.html#adjacently-tagged> for more information.
///
/// This enum is non-exhaustive, so that new events can be added in the future without breaking
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "tag", content = "val"))]
#[non_exhaustive]
pub enum Contexts {
    Events(Event),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "tag", content = "val"))]
#[non_exhaustive]
pub enum Event {
    /// The encrypted seed
    Encrypted(Vec<u8>),
}
