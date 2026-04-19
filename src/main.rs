#![feature(string_remove_matches)]
#![allow(clippy::all)]
#![allow(dead_code)]

use bert::App;
use leptos::mount;

fn main() {
    console_error_panic_hook::set_once();
    mount::mount_to_body(App);
}
