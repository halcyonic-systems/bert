#![feature(string_remove_matches)]
#![allow(clippy::all)]
#![allow(dead_code)]

pub mod bevy_app;
pub mod leptos_app;

pub use bevy_app::*;
use leptos::mount;
pub use leptos_app::*;

fn main() {
    console_error_panic_hook::set_once();
    mount::mount_to_body(App);
}
