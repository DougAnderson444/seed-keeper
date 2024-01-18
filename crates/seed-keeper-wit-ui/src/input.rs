use super::*;
use base64ct::{Base64UrlUnpadded, Encoding};

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
            // generate an html element id for the input field for the username
            "username_input" => Some(Value::from(utils::rand_id())),
            // generate an id for the input field for the password
            "password_input" => Some(Value::from(utils::rand_id())),
            "encrypted_seed_input" => Some(Value::from(utils::rand_id())),
            // generate an id for the submit button
            "submit" => Some(Value::from(utils::rand_id())),
            // if the user passed context contained the username, pass it to the front end
            "username" => Some(Value::from(
                self.as_ref()
                    .map(|c| c.username.as_ref().map(|u| u.as_str()).unwrap_or_default())
                    .unwrap_or_default(),
            )),
            // if the user passed context contained the encrypted value, pass it to the front end
            "encrypted_seed" => Some(Value::from(
                self.as_ref()
                    .map(|c| {
                        c.encrypted
                            .as_ref()
                            .map(|u| Base64UrlUnpadded::encode_string(u.as_slice()))
                            .unwrap_or_default()
                    })
                    .unwrap_or_default(),
            )),
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
