use super::*;

/// Input is the input form(s)
#[derive(Debug, Clone, Default)]
pub(crate) struct Input(Option<wurbo_types::Input>);

impl StructObject for Input {
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "placeholder" => Some(Value::from(
                self.as_ref()
                    .map(|c| c.placeholder.clone())
                    .unwrap_or_default(),
            )),
            "username_input" => Some(Value::from(utils::rand_id())),
            "password_input" => Some(Value::from(utils::rand_id())),
            "submit" => Some(Value::from(utils::rand_id())),
            _ => None,
        }
    }
    /// So that debug will show the values
    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["placeholder", "username_input", "password_input"])
    }
}

impl From<wurbo_types::Input> for Input {
    fn from(context: wurbo_types::Input) -> Self {
        Input(Some(context))
    }
}

impl From<Option<wurbo_types::Input>> for Input {
    fn from(context: Option<wurbo_types::Input>) -> Self {
        Input(context)
    }
}

impl Deref for Input {
    type Target = Option<wurbo_types::Input>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
