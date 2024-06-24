use super::*;

/// Input is the input form(s)
#[derive(Debug, Clone)]
pub(crate) struct Input(wurbo_types::Input);

impl Object for Input {
    fn get_value(self: &std::sync::Arc<Self>, key: &Value) -> Option<Value> {
        match key.as_str()? {
            "placeholder" => Some(Value::from(self.placeholder.clone())),
            "message_input" => Some(Value::from(rand_id())),
            "submit_button" => Some(Value::from(rand_id())),
            _ => None,
        }
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
