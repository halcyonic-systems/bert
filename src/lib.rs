#![feature(string_remove_matches)]
// Scoped allows: ECS query tuples routinely exceed the type_complexity threshold;
// dead_code is tolerated while the library API stabilises.
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]

pub mod bevy_app;
#[allow(clippy::redundant_locals)]
pub mod leptos_app;

// Re-export bevy_app at crate root so internal `crate::Foo` references resolve
// across submodules. Intentionally NOT re-exporting leptos_app — UI types stay
// behind `bert::leptos_app::*` to keep the library's public surface minimal for
// native consumers (e.g. the #37 transpiler).
pub use bevy_app::*;
