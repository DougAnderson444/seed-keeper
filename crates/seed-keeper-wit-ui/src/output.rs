use super::*;

use base64ct::{Base64UrlUnpadded, Encoding};
/// Event types which can can emitted from this UI and listened by others
use events::{Context, Message};

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
                    self.encrypted_seed = Encrypted(Some(encrypted.clone()));
                }

                // if serde feature, emit the serialized encrypted seed as an event
                #[cfg(feature = "serde")]
                {
                    // WIT expects variants to be {tag: _, val: _} in lower kebab-case,
                    // which we can get using serde rename_all, baked into the Context and Messag enums
                    let Ok(msg) = Message::Encrypted(encrypted.clone()).to_urlsafe() else {
                        return self;
                    };

                    let ctx = Context::Event(msg);
                    let serialized = serde_json::to_string(&ctx).unwrap_or_default();
                    crate::wurbo_in::emit(&serialized);

                    // Also emit the Username
                    let Ok(msg) = Message::Username(self.username.clone()).to_urlsafe() else {
                        return self;
                    };
                    let ctx = Context::Event(msg);
                    let serialized_username = serde_json::to_string(&ctx).unwrap_or_default();
                    crate::wurbo_in::emit(&serialized_username);
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

/// Implement StructObject for Output so we can use minijina to automagically calculate the length
/// of the username and password concatenated
impl StructObject for Output {
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            // if self.id.is_some, use it, otherwise generate a new one
            "id" => Some(Value::from(OUTPUT_ID.get_or_init(|| rand_id()).to_owned())),
            "username" => Some(Value::from(self.username.clone())),
            "password" => Some(Value::from(self.password.clone())),
            // Show encrypted Vec as base64 string
            "seed" => Some(Value::from(self.encrypted_seed.to_string())),
            "error_message" => match &self.error_message {
                Some(msg) => Some(Value::from(msg.clone())),
                None => None,
            },
            _ => None,
        }
    }

    /// So that debug will show the values
    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["id", "username", "password", "seed"])
    }
}

/// [Encrypted] is the encrypted seed passed from the User, if any.
#[derive(Debug, Clone, Default)]
pub(super) struct Encrypted(Option<Vec<u8>>);

impl ToString for Encrypted {
    fn to_string(&self) -> String {
        // encode to base64
        Base64UrlUnpadded::encode_string(&self.as_ref().unwrap_or(&vec![]))
    }
}

/// Decode from base64 into bytes
impl From<&String> for Encrypted {
    fn from(context: &String) -> Self {
        Encrypted(Some(
            Base64UrlUnpadded::decode_vec(context).unwrap_or_default(),
        ))
    }
}

impl Deref for Encrypted {
    type Target = Option<Vec<u8>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
