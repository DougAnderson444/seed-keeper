use super::*;

/// Input is the input form(s)
#[derive(Debug, Clone, Default)]
pub(crate) struct Input(Option<wurbo_types::Input>);

impl Object for Input {
    fn get_value(self: &std::sync::Arc<Self>, key: &Value) -> Option<Value> {
        match key.as_str()? {
            "id" => Some(Value::from(rand_id())),
            "placeholder" => Some(Value::from(
                self.as_ref()
                    .as_ref()
                    .map(|c| c.placeholder.clone())
                    .unwrap_or_default(),
            )),
            // copy encrypted_seed from Input, if any
            "encrypted_seed" => match self.as_ref().as_ref() {
                Some(val) => match &val.encrypted_seed {
                    Some(encrypted) => Some(Value::from(encrypted.clone())),
                    None => None,
                },
                None => None,
            },
            _ => None,
        }
    }
}

impl From<wurbo_types::Input> for Input {
    fn from(context: wurbo_types::Input) -> Self {
        Input(Some(context))
    }
}

impl From<&wurbo_types::Content> for Input {
    fn from(content: &wurbo_types::Content) -> Self {
        let encrypted = Encrypted::from(content);

        Input(Some(wurbo_types::Input {
            placeholder: content
                .input
                .as_ref()
                .map(|c| c.placeholder.clone())
                .unwrap_or_default(),
            encrypted_seed: encrypted.to_string().into(),
        }))
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
