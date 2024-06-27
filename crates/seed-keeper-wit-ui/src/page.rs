use super::*;

static PAGE_ID: OnceLock<String> = OnceLock::new();

/// Page is the wrapper for Input and Output
#[derive(Debug, Clone, Default)]
pub(crate) struct Page(Option<wurbo_types::Page>);

impl Object for Page {
    fn get_value(self: &std::sync::Arc<Self>, key: &Value) -> Option<Value> {
        match key.as_str()? {
            "title" => Some(Value::from(
                self.as_ref()
                    .as_ref()
                    .map(|v| v.title.clone())
                    .unwrap_or_default(),
            )),
            "id" => Some(Value::from(PAGE_ID.get_or_init(|| rand_id()).to_owned())),
            _ => None,
        }
    }
}

impl From<&wurbo_types::Page> for Page {
    fn from(context: &wurbo_types::Page) -> Self {
        Page(Some(context.clone()))
    }
}

impl From<&Option<wurbo_types::Page>> for Page {
    fn from(context: &Option<wurbo_types::Page>) -> Self {
        Page(context.clone())
    }
}

impl Deref for Page {
    type Target = Option<wurbo_types::Page>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
