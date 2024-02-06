use serde::{Deserialize, Serialize};
use serde_with::base64::{Base64, Standard};
use serde_with::formats::Padded;
use serde_with::serde_as;
use wurbo::prelude::Base64JSON;

/// The Context of the event.
/// WIT expects variants to be {tag: _, val: _} in lower kebab-case,
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "tag", content = "val")]
#[non_exhaustive]
pub enum Context {
    Event(Message),
}

/// The Messages eitted.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "tag", content = "val")]
#[non_exhaustive]
pub enum Message {
    /// The encrypted seed, serialized as base64 to avoid missing TypedArray issued in JavaScript
    /// with Uint8Array.
    Encrypted {
        #[serde_as(as = "Base64<Standard, Padded>")]
        seed: Vec<u8>,
    },
    /// The username
    Username(String),
}

impl Base64JSON for Context {}
