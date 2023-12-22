use super::*;

/// Output handles the storage of the values and the calculation of the length of the concatenated
#[derive(Debug, Default, Clone)]
pub(super) struct Output {
    id: Option<String>,
    pub(crate) message: Message,
    pub(crate) signature: Signature,
}

/// Impleent StructObject for Output so we can use minijina to automagically calculate the length
/// of the message and button concatenated
impl StructObject for Output {
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "message" => Some(Value::from_struct_object(self.message.clone())),
            "signature" => Some(Value::from_struct_object(self.signature.clone())),
            // if self.id.is_some, use it, otherwise generate a new one
            "id" => Some(Value::from(self.id.clone().unwrap_or(utils::rand_id()))),
            _ => None,
        }
    }

    /// So that debug will show the values
    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["id", "message"])
    }
}

/// Message captures is the message input value.
#[derive(Debug, Default, Clone)]
pub(crate) struct Message(Option<wurbo_types::Outrecord>);

impl StructObject for Message {
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "value" => Some(Value::from(
                // Deref self and use value if is_Some, otherwise use ""
                self.as_ref().map(|v| v.value.clone()).unwrap_or_default(),
            )),
            _ => None,
        }
    }

    /// So that debug will show the values
    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["value"])
    }
}

impl From<&wurbo_types::Outrecord> for Message {
    fn from(context: &wurbo_types::Outrecord) -> Self {
        Message(Some(context.clone()))
    }
}

impl From<Option<wurbo_types::Outrecord>> for Message {
    fn from(context: Option<wurbo_types::Outrecord>) -> Self {
        Message(context)
    }
}

impl Deref for Message {
    type Target = Option<wurbo_types::Outrecord>;

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
            .map(|v| v.value.clone())
            .unwrap_or_default()
            .into_bytes();
        if !v.is_empty() {
            if let Ok(s) = sign(&v) {
                self.0 = s;
            }
        }
    }
}

impl StructObject for Signature {
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "value" => Some(Value::from(self.0.clone())),
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
