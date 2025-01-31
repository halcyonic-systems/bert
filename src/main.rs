pub mod bevy_app;
pub mod leptos_app;

pub use leptos_app::*;
pub use bevy_app::*;

use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
