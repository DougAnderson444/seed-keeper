use serde::{Deserialize, Serialize};
use wurbo::prelude::Base64JSON;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
// #[cfg_attr(feature = "serde", serde(tag = "tag", content = "val"))]
// #[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[non_exhaustive]
pub enum Context {
    Event(String),
}

/// The Messages eitted.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum Message {
    /// The encrypted seed
    Encrypted(Vec<u8>),
    /// The username
    Username(String),
}

impl Base64JSON for Message {}
