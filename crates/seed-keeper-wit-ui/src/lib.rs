//! This User Interface allows a user to generate a seed or load it from storage.
//!
#![allow(static_mut_refs)] // bindings always use static mut static_mut_refs

mod bindings;

mod input;
mod output;
mod page;
mod utils;

use input::Input;
use output::Output;
use page::Page;

use std::ops::Deref;
use std::sync::OnceLock;

use bindings::seed_keeper::wallet::config::{get_encrypted, set_config, Credentials};

use bindings::exports::seed_keeper::wit_ui::wurbo_out::Guest as WurboGuest;
use bindings::seed_keeper::wit_ui::wurbo_in;
use bindings::seed_keeper::wit_ui::wurbo_types::{self, Context};

use wurbo::jinja::{Entry, Index, Rest, Templates};
use wurbo::prelude_bindgen;

use crate::utils::Encrypted;

const INDEX_HTML: &str = "index.html";
const INPUT_HTML: &str = "input.html";
const OUTPUT_HTML: &str = "output.html";

struct Component;

bindings::export!(Component with_types_in bindings);

/// We need to provide the templates for the macro to pull in
fn get_templates() -> Templates {
    Templates::new(
        Index::new(INDEX_HTML, include_str!("templates/index.html")),
        Entry::new(OUTPUT_HTML, include_str!("templates/output.html")),
        Rest::new(vec![Entry::new(
            INPUT_HTML,
            include_str!("templates/input.html"),
        )]),
    )
}

// Macro builds the Component struct and implements the Guest trait for us, saving copy-and-paste
prelude_bindgen! {WurboGuest, Component, StructContext, Context, LAST_STATE}

/// PageContext is a struct of other structs that implement [Object],
/// which is why it is not a Newtype wrapper like the others are.
#[derive(Debug, Clone, Default)]
pub struct StructContext {
    page: Page,
    input: Input,
    output: Output,
    target: Option<String>,
}

impl StructContext {
    /// with this target template, instead of defaulting to entry or output template
    fn with_target(mut self, target: String) -> Self {
        self.target = Some(target);
        self
    }
}

impl Object for StructContext {
    fn get_value(self: &std::sync::Arc<Self>, key: &Value) -> Option<Value> {
        match key.as_str()? {
            "page" => Some(Value::from_object(self.page.clone())),
            "input" => Some(Value::from_object(self.input.clone())),
            "output" => Some(Value::from_object(self.output.clone())),
            _ => None,
        }
    }
}

/// We received Context from the WIT ABI and need to convert it to PageContext
impl From<&wurbo_types::Context> for StructContext {
    fn from(context: &wurbo_types::Context) -> Self {
        match context {
            wurbo_types::Context::AllContent(content) => StructContext::from(content.clone()),
            wurbo_types::Context::Username(u) => {
                let mut output = Output::from_lastest();
                output.username = u.into();
                StructContext::from(output).with_target(OUTPUT_HTML.to_string())
            }
            wurbo_types::Context::Password(p) => {
                let mut output = Output::from_lastest();
                output.password = p.into();
                StructContext::from(output).with_target(OUTPUT_HTML.to_string())
            }
            wurbo_types::Context::Encrypted(e) => {
                let mut output = Output::from_lastest();
                output.encrypted_seed = Encrypted::from(e);
                StructContext::from(output).with_target(OUTPUT_HTML.to_string())
            }
            wurbo_types::Context::Submit => StructContext::from(Output::from_lastest().load())
                .with_target(OUTPUT_HTML.to_string()),
        }
    }
}

/// We have all the content, convert it to PageContext
impl From<wurbo_types::Content> for StructContext {
    fn from(content: wurbo_types::Content) -> Self {
        StructContext {
            page: Page::from(&content.page),
            input: Input::from(&content),
            // We can use default for Output because the minijinja Object impl will
            // calculate the values from the above inouts for us
            output: Output::from(&content),
            target: None,
        }
    }
}

impl From<Output> for StructContext {
    fn from(output: Output) -> Self {
        let mut last = { LAST_STATE.lock().unwrap().clone().unwrap_or_default() };
        last.output = output;
        last
    }
}
