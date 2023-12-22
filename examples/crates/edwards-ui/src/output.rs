use super::*;

/// Output handles the storage of the values and the calculation of the length of the concatenated
#[derive(Debug, Default, Clone)]
pub(super) struct Output {
    id: Option<String>,
    pub(crate) message: Message,
}

impl Output {
    /// Load the given message and button into the config and then
    /// get the encrypted seed back
    fn signature(&self) -> Value {
        // sign mesage and return Value as signature only if message is not empty
        // if message is empty, return Value as empty string
        Value::from(
            self.message
                .as_ref()
                .map(|v| match v.value.as_str() {
                    "" => "".to_string(),
                    msg => format!(
                        "{:#x?}",
                        sign(msg.as_bytes())
                            .map_err(|e| format!("{:?}", e))
                            .expect("error to be string")
                    ),
                })
                .unwrap_or_default(),
        )
    }
}

/// Impleent StructObject for Output so we can use minijina to automagically calculate the length
/// of the message and button concatenated
impl StructObject for Output {
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "message" => Some(Value::from_struct_object(self.message.clone())),
            "signature" => Some(self.signature()),
            // if self.id.is_some, use it, otherwise generate a new one
            "id" => Some(Value::from(self.id.clone().unwrap_or(utils::rand_id()))),
            _ => None,
        }
    }

    /// So that debug will show the values
    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["id", "message", "signature"])
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
