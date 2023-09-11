mod components;

use leptos::*;

// use components::App;

fn main() {
    mount_to_body(|cx| view! { cx,  <p>"Hello there, world!"</p> })
}
