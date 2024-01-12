use super::*;
use std::sync::OnceLock;

static OUTPUT_ID: OnceLock<String> = OnceLock::new();

/// Output handles the storage of the values and the calculation of the length of the concatenated
#[derive(Debug, Default, Clone)]
pub(super) struct Output {
    pub(crate) message: Message,
    pub(crate) signature: Signature,
    pub(crate) log: String,
}

/// Impleent StructObject for Output so we can use minijina to automagically calculate the length
/// of the message and button concatenated
impl StructObject for Output {
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "message" => Some(Value::from_struct_object(self.message.clone())),
            "signature" => Some(Value::from_struct_object(self.signature.clone())),
            "log" => Some(Value::from(self.log.clone())),
            // if self.id.is_some, use it, otherwise generate a new one
            "id" => Some(Value::from(
                OUTPUT_ID.get_or_init(|| utils::rand_id()).to_owned(),
            )),
            _ => None,
        }
    }

    /// So that debug will show the values
    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["id", "message", "signature", "log"])
    }
}

/// Message captures is the message input value.
#[derive(Debug, Default, Clone)]
pub(crate) struct Message(Option<String>);

impl StructObject for Message {
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "value" => Some(Value::from(
                // Deref self and use value if is_Some, otherwise use ""
                self.as_ref().map(|v| v.clone()).unwrap_or_default(),
            )),
            _ => None,
        }
    }

    /// So that debug will show the values
    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["value"])
    }
}

impl From<&String> for Message {
    fn from(context: &String) -> Self {
        Message(Some(context.clone()))
    }
}

impl From<Option<String>> for Message {
    fn from(context: Option<String>) -> Self {
        Message(context)
    }
}

impl Deref for Message {
    type Target = Option<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Submit triggers the message input value to be signed
#[derive(Debug, Default, Clone)]
pub(crate) struct Signature(Vec<u8>);

impl Signature {
    /// Load the given message and button into the config and then
    /// get the encrypted seed back
    pub(crate) fn sign(&mut self, msg: &Message) {
        let v = msg
            .as_ref()
            .map(|v| v.clone())
            .unwrap_or_default()
            .into_bytes();
        if !v.is_empty() {
            if let Ok(s) = operations::sign(&v) {
                self.0 = s;
            } else {
                self.0 = vec![69, 69, 69];
            }
        }
    }
}

impl StructObject for Signature {
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "value" => Some(Value::from(self.0.clone())),
            "hex" => {
                let literal = format!("{:X?}", self.0.clone());
                // remove the [], the delimiter "," and the spaces
                let literal = literal.replace("[", "").replace("]", "").replace(", ", "");
                Some(Value::from(literal))
            }
            _ => None,
        }
    }

    /// So that debug will show the values
    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["value"])
    }
}

impl Deref for Signature {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
