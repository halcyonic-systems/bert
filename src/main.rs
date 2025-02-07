#![feature(string_remove_matches)]

pub mod bevy_app;
pub mod leptos_app;

use leptos::mount;
pub use leptos_app::*;
pub use bevy_app::*;

fn main() {
    console_error_panic_hook::set_once();
    mount::mount_to_body(App);
}
