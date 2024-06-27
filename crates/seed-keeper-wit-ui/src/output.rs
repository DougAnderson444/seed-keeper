use super::*;

/// Event types which can can emitted from this UI and listened by others
use seed_keeper_events::{Context, Message};

static OUTPUT_ID: OnceLock<String> = OnceLock::new();

/// Output handles the storage of the values and the calculation of the length of the concatenated
#[derive(Debug, Default, Clone)]
pub(super) struct Output {
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) encrypted_seed: Encrypted,
    error_message: Option<String>,
}

impl Output {
    /// Create Output from the lastest state (LAST_STATE)
    pub(crate) fn from_lastest() -> Self {
        let state = { LAST_STATE.lock().unwrap().clone().unwrap_or_default() };
        state.output
    }

    /// Load the given username and password (and maybe given encrypted seed, if any) into the config and then
    /// get the encrypted seed back. Set all [Output] values to the encrypted seed, if changed.
    pub(crate) fn load(mut self) -> Self {
        self.error_message = None;

        if let Err(e) = set_config(&Credentials::from(self.clone())) {
            self.error_message = Some(format!("Error in Output setting config: {:?}", e));
            return self;
        }

        match get_encrypted() {
            Ok(encrypted) => {
                // if self.encrypted_seed is None, set it to the encrypted seed
                if self.encrypted_seed.is_none() {
                    self.encrypted_seed = Encrypted::from(Some(encrypted.clone()));
                }

                // if serde feature, emit the serialized encrypted seed as an event
                #[cfg(feature = "serde")]
                {
                    let Ok(serialized) =
                        serde_json::to_string(&Context::Event(Message::Encrypted {
                            seed: encrypted.clone(),
                        }))
                    else {
                        return self;
                    };
                    wurbo_in::emit(&serialized);

                    // // Also emit the Username
                    // let Ok(msg) = Message::Username(self.username.clone()).to_urlsafe() else {
                    //     return self;
                    // };
                    // let ctx = Context::Event(msg);
                    // let serialized_username = serde_json::to_string(&ctx).unwrap_or_default();
                    // crate::wurbo_in::emit(&serialized_username);
                }

                self
            }
            Err(e) => {
                self.error_message = Some(format!("Error in Output getting encrypted: {:?}", e));
                return self;
            }
        }
    }
}

impl From<Output> for Credentials {
    fn from(context: Output) -> Self {
        Credentials {
            username: context.username.into(),
            password: context.password.into(),
            encrypted: context.encrypted_seed.deref().clone(),
        }
    }
}

// impl From<&Content> for Output {
impl From<&wurbo_types::Content> for Output {
    fn from(content: &wurbo_types::Content) -> Self {
        let encrypted = Encrypted::from(content);

        let username: String = content.load.as_ref().map_or_else(
            || "".to_string(),
            |loaded_str| {
                let v: serde_json::Value =
                    serde_json::from_str(&loaded_str).unwrap_or(serde_json::Value::Null);

                match &v["username"] {
                    serde_json::Value::String(username) => username.clone(),
                    _ => "".to_string(),
                }
            },
        );

        Output {
            username,
            password: content
                .input
                .as_ref()
                .map(|c| c.placeholder.clone())
                .unwrap_or_default(),
            encrypted_seed: encrypted,
            error_message: None,
        }
    }
}

/// Implement [Object] for Output so we can use minijina to automagically calculate the length
/// of the username and password concatenated
impl Object for Output {
    fn get_value(self: &std::sync::Arc<Self>, key: &Value) -> Option<Value> {
        match key.as_str()? {
            // if self.id.is_some, use it, otherwise generate a new one
            "id" => Some(Value::from(OUTPUT_ID.get_or_init(|| rand_id()).to_owned())),
            "username" => Some(Value::from(self.username.clone())),
            "password" => Some(Value::from(self.password.clone())),
            // Show encrypted Vec as base64 string
            "encrypted_seed" => match get_encrypted() {
                Ok(encrypted) => Some(Value::from(Encrypted::from(Some(encrypted)).to_string())),
                _ => None,
            },
            "error_message" => match &self.error_message {
                Some(msg) => Some(Value::from(msg.clone())),
                None => None,
            },
            _ => None,
        }
    }
}
