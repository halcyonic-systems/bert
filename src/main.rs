#![feature(string_remove_matches)]
#![allow(clippy::type_complexity)]
#![allow(dead_code)]

use bert::leptos_app::App;
use leptos::mount;

fn main() {
    console_error_panic_hook::set_once();
    mount::mount_to_body(App);
}
