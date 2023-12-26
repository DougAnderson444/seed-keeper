//! The render function here acts more like a Router.
//!
//! During initialization, it takes the context and
//! loads it into each of the child components to render the whole app.
//! Once initialized, it will route updates to the appropriate child component,
//! depending on the context.
//!
//! The structObject impl in this case of the Router should simply call render()
//! on the appropriate child component, which returns a string of HTML, which
//! this router passes through back up to the DOM page.
//!
//! Since the child components are unaware they are in a larger app, the App template needs to
//! wrap the child templates in a larger context variant by specifying a
//! data-attribute tag for the router. This is how the router knows which component to render.
//!
//! Each child component takes care of its own inputs/output, so the Router doesn't
//! need to worry about it.
//!
cargo_component_bindings::generate!();

use crate::bindings::exports::wallet::aggregate_wit_ui::wurbo_out::Guest as WurboGuest;
use crate::bindings::wallet::aggregate_wit_ui::wurbo_in;
use crate::bindings::wallet::aggregate_wit_ui::wurbo_types::{self, Context};
use bindings::example::edwards_ui;
use bindings::seed_keeper::wit_ui;

use wurbo::jinja::{Entry, Index, Rest, Templates};
use wurbo::prelude_bindgen;

use std::ops::Deref;

struct Component;

/// We need to provide the templates for the macro to pull in
fn get_templates() -> Templates {
    Templates::new(
        Index::new("page.html", include_str!("templates/page.html")),
        Entry::new("output.html", include_str!("templates/output.html")),
        Rest::new(vec![]),
    )
}

// Macro builds the Component struct and implements the Guest trait for us, saving copy-and-paste
prelude_bindgen! {WurboGuest, Component, AppContext, Context, LAST_STATE}

/// AppContent is all the content for the entire app. It's comprised of the content of all the
/// components.
#[derive(Debug, Clone)]
struct AppContext {
    app: App,
    seed_keeper: SeedKeeper,
    edwards: Edwards,
}

impl StructObject for AppContext {
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "app" => Some(Value::from_struct_object(self.app.clone())),
            "seed_ui" => Some(Value::from_struct_object(self.seed_keeper.clone())),
            "edwards_ui" => Some(Value::from_struct_object(self.edwards.clone())),
            _ => None,
        }
    }
    /// So that debug will show the values
    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["app", "seed_ui", "edwards_ui"])
    }
}

/// Route the recieve Context into the appropriate AppContext
impl From<&wurbo_types::Context> for AppContext {
    fn from(context: &wurbo_types::Context) -> Self {
        match context {
            wurbo_types::Context::AllContent(c) => AppContext::from(c.clone()),
            wurbo_types::Context::Seed(s) => AppContext::from(SeedKeeper::from(s)),
            wurbo_types::Context::Edwards(ed) => AppContext::from(Edwards::from(ed)),
        }
    }
}

/// We have all the content, convert it to AppContext
impl From<wurbo_types::Content> for AppContext {
    fn from(context: wurbo_types::Content) -> Self {
        AppContext {
            app: App::from(context.app),
            // We pass props since initial content could have the encrypted seed for the seed keeper
            seed_keeper: SeedKeeper::from(context.seed_ui),
            // We could have an initial message to sign or verify too...
            edwards: Edwards::from(context.edwards_ui),
        }
    }
}

impl From<SeedKeeper> for AppContext {
    fn from(context: SeedKeeper) -> Self {
        let mut state = { LAST_STATE.lock().unwrap().clone().unwrap() };
        state.seed_keeper = context;
        state
    }
}

impl From<Edwards> for AppContext {
    fn from(context: Edwards) -> Self {
        let mut state = { LAST_STATE.lock().unwrap().clone().unwrap() };
        state.edwards = context;
        state
    }
}

// Some App properties
#[derive(Debug, Clone)]
pub(crate) struct App(wurbo_types::App);

impl StructObject for App {
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "title" => Some(Value::from(self.title.clone())),
            "id" => Some(Value::from(utils::rand_id())),
            _ => None,
        }
    }
    /// So that debug will show the values
    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["title", "id"])
    }
}

impl From<wurbo_types::App> for App {
    fn from(context: wurbo_types::App) -> Self {
        App(context)
    }
}

impl Deref for App {
    type Target = wurbo_types::App;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Debug, Clone)]
struct SeedKeeper(Option<wurbo_types::SeedContext>);

/// Implement StructObject for SeedKeeper so that we can use it in the template
/// The main point of this impl is to call render(ctx) on the SeedKeeperUIContext
/// and return the HTML string as the Value
impl StructObject for SeedKeeper {
    fn get_field(&self, name: &str) -> Option<Value> {
        match (name, wit_ui::wurbo_out::render(&self.into())) {
            ("html", Ok(html)) => Some(Value::from(html)),
            _ => None,
        }
    }
    /// So that debug will show the values
    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["html"])
    }
}

impl From<Option<wurbo_types::SeedContext>> for SeedKeeper {
    fn from(context: Option<wurbo_types::SeedContext>) -> Self {
        SeedKeeper(context)
    }
}

impl From<wurbo_types::SeedContext> for SeedKeeper {
    fn from(context: wurbo_types::SeedContext) -> Self {
        SeedKeeper(Some(context))
    }
}

impl From<&wit_ui::wurbo_types::Context> for SeedKeeper {
    fn from(context: &wit_ui::wurbo_types::Context) -> Self {
        SeedKeeper::from(context.clone())
    }
}

impl From<SeedKeeper> for wit_ui::wurbo_types::Context {
    fn from(context: SeedKeeper) -> Self {
        context.0.unwrap().into()
    }
}

impl From<&SeedKeeper> for wit_ui::wurbo_types::Context {
    fn from(context: &SeedKeeper) -> Self {
        context.0.clone().unwrap().into()
    }
}

impl Deref for SeedKeeper {
    type Target = wurbo_types::SeedContext;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
}

#[derive(Debug, Clone)]
struct Edwards(Option<wurbo_types::EdwardsContext>);

/// Implement StructObject for Edwards so that we can use it in the template
/// The main point of this impl is to call render(ctx) on the EdwardsUIContext
/// and return the HTML string as the Value
impl StructObject for Edwards {
    fn get_field(&self, name: &str) -> Option<Value> {
        match (name, edwards_ui::wurbo_out::render(&self.into())) {
            ("html", Ok(html)) => Some(Value::from(html)),
            _ => None,
        }
    }
    /// So that debug will show the values
    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["html"])
    }
}

impl From<Option<wurbo_types::EdwardsContext>> for Edwards {
    fn from(context: Option<wurbo_types::EdwardsContext>) -> Self {
        Edwards(context)
    }
}

impl From<&edwards_ui::wurbo_types::Context> for Edwards {
    fn from(context: &edwards_ui::wurbo_types::Context) -> Self {
        Edwards::from(context.clone())
    }
}

impl From<edwards_ui::wurbo_types::Context> for Edwards {
    fn from(context: edwards_ui::wurbo_types::Context) -> Self {
        Edwards(Some(context.into()))
    }
}

impl From<Edwards> for edwards_ui::wurbo_types::Context {
    fn from(context: Edwards) -> Self {
        context.0.unwrap().into()
    }
}

impl From<&Edwards> for edwards_ui::wurbo_types::Context {
    fn from(context: &Edwards) -> Self {
        context.0.clone().unwrap().into()
    }
}

impl Deref for Edwards {
    type Target = wurbo_types::EdwardsContext;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
}