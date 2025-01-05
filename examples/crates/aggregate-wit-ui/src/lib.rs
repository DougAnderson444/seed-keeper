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
#![allow(static_mut_refs)] // bindings always use static mut static_mut_refs
#[allow(warnings)]
mod bindings;

use crate::bindings::exports::wallet::aggregate_wit_ui::wurbo_out::Guest as WurboGuest;
use crate::bindings::wallet::aggregate_wit_ui::wurbo_types::{self, Context};
use bindings::example::edwards_ui;
use bindings::exports::wallet::aggregate_wit_ui::aggregation::Guest as AggregationGuest;
use bindings::seed_keeper::wit_ui;

use wurbo::jinja::{error::RenderError, Entry, Index, Rest, Templates};
use wurbo::prelude::*;

use std::ops::Deref;

struct Component;

bindings::export!(Component with_types_in bindings);

/// We need to provide the templates for the macro to pull in
fn get_templates() -> Templates {
    Templates::new(
        Index::new("page.html", include_str!("templates/page.html")),
        Entry::new("output.html", include_str!("templates/output.html")),
        Rest::new(vec![]),
    )
}

impl WurboGuest for Component {
    /// Render needs to use the Aggregate Template for the initial load, but after that simply call
    /// render on the child component and pass through the HTML
    fn render(context: Context) -> Result<String, String> {
        let html = match context {
            Context::AllContent(ctx) => {
                let templates = get_templates();
                let entry = templates.entry.name.clone();
                let mut env = Environment::new();

                for (name, template) in templates.into_iter() {
                    env.add_template_owned(name.clone(), template.clone())
                        .expect("template should be added");
                }

                let struct_ctx = Value::from_object(AppContext::from(ctx.clone()));

                let prnt_err = |e| {
                    println!("Could not render template: {:#}", e);
                    let mut err = &e as &dyn std::error::Error;
                    while let Some(next_err) = err.source() {
                        println!("caused by: {:#}", next_err);
                        err = next_err;
                    }
                    RenderError::from(e)
                };

                let tmpl = env.get_template(&entry).map_err(prnt_err)?;
                let rendered = tmpl.render(&struct_ctx).map_err(prnt_err)?;
                rendered
            }
            Context::Seed(ctx) => wit_ui::wurbo_out::render(&ctx.into())?,
            Context::Edwards(ctx) => edwards_ui::wurbo_out::render(&ctx.into())?,
        };
        Ok(html)
    }

    /// No-op for activate()
    fn activate(_selectors: Option<Vec<String>>) {}
}

impl AggregationGuest for Component {
    fn activates(selectors: Option<Vec<String>>) {
        // iterate over each of the child components' wurbo_out's and call activate
        edwards_ui::wurbo_out::activate(selectors.as_ref().map_or(None, |s| Some(s.as_slice())));
        wit_ui::wurbo_out::activate(selectors.as_ref().map_or(None, |s| Some(s.as_slice())));
    }
}

/// AppContent is all the content for the entire app. It's comprised of the content of all the
/// components.
#[derive(Debug, Clone)]
pub(crate) struct AppContext {
    app: App,
    seed_ui: SeedUI,
    edwards_ui: Edwards,
}

impl Object for AppContext {
    fn get_value(self: &std::sync::Arc<Self>, key: &Value) -> Option<Value> {
        match key.as_str()? {
            "app" => Some(Value::from_object(self.app.clone())),
            "seed_ui" => Some(Value::from_object(self.seed_ui.clone())),
            "edwards_ui" => Some(Value::from_object(self.edwards_ui.clone())),
            _ => None,
        }
    }
}

/// We have all the content, convert it to AppContext
impl From<wurbo_types::Content> for AppContext {
    fn from(context: wurbo_types::Content) -> Self {
        AppContext {
            app: App::from(context.app),
            // We pass props since initial content could have the encrypted seed for the seed keeper
            seed_ui: SeedUI::from(context.seed_ui),
            // We could have an initial message to sign or verify too...
            edwards_ui: Edwards::from(context.edwards_ui),
        }
    }
}

// Some App properties
#[derive(Debug, Clone)]
pub(crate) struct App(wurbo_types::App);

impl Object for App {
    fn get_value(self: &std::sync::Arc<Self>, key: &Value) -> Option<Value> {
        match key.as_str()? {
            "title" => Some(Value::from(self.title.clone())),
            "id" => Some(Value::from(rand_id())),
            _ => None,
        }
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

/// Wrapper around the seed keeper context so we can implement [Object] on top of it
#[derive(Debug, Clone)]
struct SeedUI(wurbo_types::SeedContext);

/// Implement [Object] for SeedKeeper so that we can use it in the template
/// The main point of this impl is to call render(ctx) on the SeedKeeperUIContext
/// and return the HTML string as the Value
impl Object for SeedUI {
    /// Simply passes through the seed context to the component for rendering
    /// outputs to .html
    fn get_value(self: &std::sync::Arc<Self>, key: &Value) -> Option<Value> {
        let render_result = wit_ui::wurbo_out::render(&self);
        match (key.as_str()?, render_result) {
            ("html", Ok(html)) => Some(Value::from(html)),
            _ => None,
        }
    }
}

impl From<wit_ui::wurbo_types::Context> for SeedUI {
    fn from(context: wit_ui::wurbo_types::Context) -> Self {
        SeedUI(context)
    }
}

impl Deref for SeedUI {
    type Target = wurbo_types::SeedContext;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
struct Edwards(wurbo_types::EdwardsContext);

/// Implement [Object] for Edwards so that we can use it in the template
/// The main point of this impl is to call render(ctx) on the EdwardsUIContext
/// and return the HTML string as the Value
impl Object for Edwards {
    fn get_value(self: &std::sync::Arc<Self>, key: &Value) -> Option<Value> {
        let render_result = edwards_ui::wurbo_out::render(&self);
        match (key.as_str()?, render_result) {
            ("html", Ok(html)) => Some(Value::from(html)),
            _ => None,
        }
    }
}

impl From<edwards_ui::wurbo_types::Context> for Edwards {
    fn from(context: edwards_ui::wurbo_types::Context) -> Self {
        Self(context)
    }
}

impl Deref for Edwards {
    type Target = wurbo_types::EdwardsContext;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
