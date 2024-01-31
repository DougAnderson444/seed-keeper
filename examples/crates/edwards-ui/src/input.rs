use super::*;

/// Input is the input form(s)
#[derive(Debug, Clone)]
pub(crate) struct Input(wurbo_types::Input);

impl StructObject for Input {
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "placeholder" => Some(Value::from(self.placeholder.clone())),
            "message_input" => Some(Value::from(rand_id())),
            "submit_button" => Some(Value::from(rand_id())),
            _ => None,
        }
    }
    /// So that debug will show the values
    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["placeholder", "message_input", "submit_button"])
    }
}

impl From<wurbo_types::Input> for Input {
    fn from(context: wurbo_types::Input) -> Self {
        Input(context)
    }
}

impl Deref for Input {
    type Target = wurbo_types::Input;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
