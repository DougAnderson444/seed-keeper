cargo_component_bindings::generate!();

mod input;
mod output;
mod page;

use input::Input;
use output::Output;
use page::Page;

use bindings::example::edwards_ui::wurbo_in;
use bindings::example::edwards_ui::wurbo_types::{self, Context};
use bindings::exports::example::edwards_ui::wurbo_out::Guest as WurboGuest;
use bindings::seed_keeper::edwards_wit::operations;

use wurbo::jinja::{Entry, Index, Rest, Templates};
use wurbo::prelude_bindgen;

use std::ops::Deref;

struct Component;

/// We need to provide the templates for the macro to pull in
fn get_templates() -> Templates {
    let templates = Templates::new(
        Index::new("page.html", include_str!("templates/page.html")),
        Entry::new("output.html", include_str!("templates/output.html")),
        Rest::new(vec![Entry::new(
            "input.html",
            include_str!("templates/input.html"),
        )]),
    );
    templates
}

// Macro builds the Component struct and implements the Guest trait for us, saving copy-and-paste
prelude_bindgen! {WurboGuest, Component, PageContext, Context, LAST_STATE}

/// PageContext is a struct of other structs that implement [StructObject],
/// which is why it is not a Newtype wrapper like the others are.
#[derive(Debug, Clone)]
pub struct PageContext {
    page: Page,
    input: Input,
    pub(crate) output: Output,
    target: Option<String>,
}

impl StructObject for PageContext {
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

/// We received Context from the WIT ABO and need to convert it to PageContext
impl From<&wurbo_types::Context> for PageContext {
    fn from(context: &wurbo_types::Context) -> Self {
        // Output is not a type of context, because it is calculated from the other values
        match context {
            wurbo_types::Context::AllContent(c) => PageContext::from(c.clone()),
            wurbo_types::Context::Message(u) => PageContext::from(output::Message::from(u)),
            // Submit doesn't provide any context (just an action), we use the output.message STATE value
            wurbo_types::Context::Submit(_) => PageContext::from(output::Signature::default()),
        }
    }
}

/// We have all the content, convert it to PageContext
impl From<wurbo_types::Content> for PageContext {
    fn from(context: wurbo_types::Content) -> Self {
        PageContext {
            page: Page::from(context.page),
            input: Input::from(context.input),
            // We can use default for Output because the minijinja StructObject impl will
            // calculate the values from the above inouts for us
            output: Output::default(),
            target: None,
        }
    }
}

impl From<output::Message> for PageContext {
    fn from(message: output::Message) -> Self {
        // Safe to unwrap here because render on all page content will always be called first
        let mut state = {
            LAST_STATE
                .lock()
                .unwrap()
                .clone()
                .expect("AllContent should be rendered first before updates via Message happen")
        };
        state.output.message = message;
        state
    }
}

impl From<output::Signature> for PageContext {
    fn from(mut sig: output::Signature) -> Self {
        let mut state = {
            LAST_STATE
                .lock()
                .unwrap()
                .clone()
                .expect("AllContent should be rendered first before this happens")
        };
        sig.sign(&state.output.message);
        state.output.signature = sig;
        state
    }
}

impl From<output::Output> for PageContext {
    fn from(output: output::Output) -> Self {
        // Safe to unwrap here because render on all page content will always be called first
        let mut state = {
            LAST_STATE
                .lock()
                .unwrap()
                .clone()
                .expect("AllContent should be rendered first before updates to Output happen")
        };
        state.output = output;
        state
    }
}
