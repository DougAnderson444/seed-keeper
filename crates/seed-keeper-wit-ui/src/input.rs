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
        // if context.load.encrypted is Some, use it,
        // the rest of Input is taken from context.input
        println!("context.load: {:?}", content.load);
        let encrypted = match &content.load {
            Some(loaded_str) => {
                println!("loaded_str: {:?}", loaded_str);
                // try to parse the JSON
                let v: serde_json::Value =
                    serde_json::from_str(&loaded_str).unwrap_or(serde_json::Value::Null);

                println!("v: {:?}", v);
                println!("v[\"encrypted\"]: {:?}", v["encrypted"]);

                match &v["encrypted"] {
                    serde_json::Value::Array(encrypted) => {
                        println!("encrypted array: {:?}", encrypted);
                        Some(
                            // encrypted into Vec<u8>
                            encrypted
                                .iter()
                                .map(|v| v.as_u64().unwrap_or_default() as u8)
                                .collect::<Vec<u8>>(),
                        )
                    }
                    // or it could be astring of numbers, likw 1,2,3,4,5...
                    serde_json::Value::String(encrypted) => {
                        println!("encrypted string: {:?}", encrypted);
                        let encrypted = encrypted
                            .split(',')
                            .map(|v| v.parse::<u8>().unwrap_or_default())
                            .collect::<Vec<u8>>();
                        Some(encrypted)
                    }
                    _ => None,
                }
            }
            None => None,
        };

        let encrypted: Encrypted = encrypted.into();

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
