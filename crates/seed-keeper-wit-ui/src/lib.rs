//! This User Interface allows a user to generate a seed or load it from storage.
//!
cargo_component_bindings::generate!();

mod input;
mod output;
mod page;

use input::Input;
use output::Output;
use page::Page;

use std::ops::Deref;

use bindings::seed_keeper::wallet::config::{get_encrypted, set_config, Credentials};

use bindings::exports::seed_keeper::wit_ui::wurbo_out::Guest as WurboGuest;
use bindings::seed_keeper::wit_ui::wurbo_in;
use bindings::seed_keeper::wit_ui::wurbo_types::{self, Context};
use wurbo::jinja::{Entry, Index, Rest, Templates};
use wurbo::prelude_bindgen;

struct Component;

/// We need to provide the templates for the macro to pull in
fn get_templates() -> Templates {
    Templates::new(
        Index::new("page.html", include_str!("templates/page.html")),
        Entry::new("output.html", include_str!("templates/output.html")),
        Rest::new(vec![Entry::new(
            "input.html",
            include_str!("templates/input.html"),
        )]),
    )
}

// Macro builds the Component struct and implements the Guest trait for us, saving copy-and-paste
prelude_bindgen! {WurboGuest, Component, SeedUIContext, Context, LAST_STATE}

/// PageContext is a struct of other structs that implement [StructObject],
/// which is why it is not a Newtype wrapper like the others are.
#[derive(Debug, Clone, Default)]
pub struct SeedUIContext {
    page: Page,
    input: Input,
    pub(crate) output: Output,
}

impl StructObject for SeedUIContext {
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "page" => Some(Value::from_struct_object(self.page.clone())),
            "input" => Some(Value::from_struct_object(self.input.clone())),
            "output" => Some(Value::from_struct_object(self.output.clone())),
            _ => None,
        }
    }
    /// So that debug will show the values
    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["page", "input", "output"])
    }
}

/// We received Context from the WIT ABI and need to convert it to PageContext
impl From<&wurbo_types::Context> for SeedUIContext {
    fn from(context: &wurbo_types::Context) -> Self {
        let mut state = { LAST_STATE.lock().unwrap().clone().unwrap_or_default() };

        match context {
            wurbo_types::Context::AllContent(c) => SeedUIContext::from(c.clone()),
            wurbo_types::Context::Username(u) => {
                state.output.username = u.into();
                state
            }
            wurbo_types::Context::Password(p) => {
                state.output.password = p.into();
                state
            }
            wurbo_types::Context::Encrypted(e) => {
                state.output.encrypted = e.into();
                state
            }
            wurbo_types::Context::Submit => {
                state.output.seed = output::Seed::from(state.clone());
                state
            }
        }
    }
}

/// We have all the content, convert it to PageContext
impl From<wurbo_types::Content> for SeedUIContext {
    fn from(context: wurbo_types::Content) -> Self {
        SeedUIContext {
            page: Page::from(context.page),
            input: Input::from(context.input),
            // We can use default for Output because the minijinja StructObject impl will
            // calculate the values from the above inouts for us
            output: Output::default(),
        }
    }
}
