use super::*;

/// Event types which can can emitted from this UI and listened by others
use events::{Context, Message};

/// Output handles the storage of the values and the calculation of the length of the concatenated
#[derive(Debug, Default, Clone)]
pub(super) struct Output {
    id: Option<String>,
    pub(crate) username: Username,
    pub(crate) password: Password,
    pub(crate) encrypted: Encrypted,
    pub(crate) seed: Option<Value>,
}

impl Output {
    /// Load the given username and password into the config and then
    /// get the encrypted seed back
    pub(crate) fn seed(&self) -> Value {
        let config = Credentials {
            username: self
                .username
                .as_ref()
                .map(|v| v.clone())
                .unwrap_or_default()
                .into(),
            password: self
                .password
                .as_ref()
                .map(|v| v.clone())
                .unwrap_or_default()
                .into(),
            encrypted: self.encrypted.clone().into(),
        };

        if let Err(e) = set_config(&config) {
            return Value::from(format!("Error in Output setting config: {:?}", e));
        }

        match get_encrypted() {
            Ok(encrypted) => {
                // if serde feature, emit the serialized encrypted seed as an event
                #[cfg(feature = "serde")]
                {
                    // WIT expects variants to be {tag: _, val: _} in lowercase,
                    // which we cannot get using serde
                    // let ctx = Context::new("events", &Message::Encrypted(encrypted.clone()));
                    let ctx = Context::Event(Message::Encrypted(encrypted.clone()));
                    let serialized =
                        serde_json::to_string(&ctx).expect("to be able to serialize Context");
                    crate::wurbo_in::emit(&serialized);
                }

                Value::from(encrypted)
            }
            Err(e) => Value::from(format!("Error in Output getting encrypted: {:?}", e)),
        }
    }

    /// Calculate the length of the username and password concatenated
    fn calculate(&self) -> Value {
        Value::from(*&self.concat().len())
    }

    /// Concatenate the username and password
    fn concat(&self) -> String {
        format!(
            "{}{}",
            self.username
                .as_ref()
                .map(|v| v.clone())
                .unwrap_or_default(),
            self.password
                .as_ref()
                .map(|v| v.clone())
                .unwrap_or_default()
        )
    }
}

/// Implement StructObject for Output so we can use minijina to automagically calculate the length
/// of the username and password concatenated
impl StructObject for Output {
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "username" => Some(Value::from_struct_object(self.username.clone())),
            "password" => Some(Value::from_struct_object(self.password.clone())),
            "value" => Some(Value::from(self.concat())),
            // self.username
            "count" => Some(Value::from(self.calculate())),
            "seed" => match &self.seed {
                Some(seed) => Some(Value::from(seed.clone())),
                None => Some(Value::from("Enter username and password to create a seed.")),
            },
            // if self.id.is_some, use it, otherwise generate a new one
            "id" => Some(Value::from(self.id.clone().unwrap_or(utils::rand_id()))),
            _ => None,
        }
    }

    /// So that debug will show the values
    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["value", "count", "id", "username", "password"])
    }
}

/// Username captures is the username input value.
#[derive(Debug, Default, Clone)]
pub(crate) struct Username(Option<String>);

impl StructObject for Username {
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

impl From<&String> for Username {
    fn from(context: &String) -> Self {
        Username(Some(context.clone()))
    }
}

impl From<Option<String>> for Username {
    fn from(context: Option<String>) -> Self {
        Username(context)
    }
}

impl Deref for Username {
    type Target = Option<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// [Password] is the password field in the form
/// We wrap it as a newtype so that we can impl [StructObject] for it
/// We impl [Deref] so we can access the inner of the Rust smart pointer
#[derive(Debug, Default, Clone)]
pub(crate) struct Password(Option<String>);

impl StructObject for Password {
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

impl From<&String> for Password {
    fn from(context: &String) -> Self {
        Password(Some(context.clone()))
    }
}

impl From<Option<String>> for Password {
    fn from(context: Option<String>) -> Self {
        Password(context)
    }
}

impl Deref for Password {
    type Target = Option<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// [Encrypted] is the encrypted seed passed from the User, if any.
#[derive(Debug, Clone, Default)]
pub(super) struct Encrypted(Option<wurbo_types::Encrypted>);

impl From<Encrypted> for Option<Vec<u8>> {
    fn from(context: Encrypted) -> Self {
        context.0
    }
}

impl From<&Vec<u8>> for Encrypted {
    fn from(context: &Vec<u8>) -> Self {
        Encrypted(Some(context.clone().into()))
    }
}

impl Deref for Encrypted {
    type Target = Option<Vec<u8>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
