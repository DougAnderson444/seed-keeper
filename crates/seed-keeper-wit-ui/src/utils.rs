//! This module contains the utility types and functions used in the seed-keeper-wit-ui crate.
use super::*;
use base64ct::{Base64UrlUnpadded, Encoding};

/// [Encrypted] [Output] is the encrypted seed passed from the User, if any.
#[derive(Debug, Clone, Default)]
pub(super) struct Encrypted(Option<Vec<u8>>);

/// impl From Option<Vec<u8>> for Encrypted
impl From<Option<Vec<u8>>> for Encrypted {
    fn from(context: Option<Vec<u8>>) -> Self {
        Encrypted(context)
    }
}

impl ToString for Encrypted {
    fn to_string(&self) -> String {
        // encode to base64
        Base64UrlUnpadded::encode_string(&self.as_ref().unwrap_or(&vec![]))
    }
}

/// Decode from base64 into bytes
impl From<&String> for Encrypted {
    fn from(context: &String) -> Self {
        Encrypted(Some(
            Base64UrlUnpadded::decode_vec(context).unwrap_or_default(),
        ))
    }
}
impl From<&wurbo_types::Content> for Encrypted {
    fn from(content: &wurbo_types::Content) -> Self {
        // if context.load.encrypted is Some, use it,
        // the rest of Input is taken from context.input
        let encrypted = match &content.load {
            Some(loaded_str) => {
                // try to parse the JSON
                let v: serde_json::Value =
                    serde_json::from_str(&loaded_str).unwrap_or(serde_json::Value::Null);

                match &v["encrypted"] {
                    serde_json::Value::Array(encrypted) => {
                        Some(
                            // encrypted into Vec<u8>
                            encrypted
                                .iter()
                                .map(|v| v.as_u64().unwrap_or_default() as u8)
                                .collect::<Vec<u8>>(),
                        )
                    }
                    // or it could be astring of numbers, likw 1,2,3,4,5...
                    serde_json::Value::String(encrypted) => {
                        let encrypted = encrypted
                            .split(',')
                            .map(|v| v.parse::<u8>().unwrap_or_default())
                            .collect::<Vec<u8>>();
                        Some(encrypted)
                    }
                    _ => None,
                }
            }
            None => None,
        };

        encrypted.into()
    }
}

impl Deref for Encrypted {
    type Target = Option<Vec<u8>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
