//! bert-compose as a library: the executable dynamical face, callable by
//! another shell. The binary (`main.rs`) is the egui authoring app; this crate
//! root also exposes the minimum a hosting app needs to *run* an authored model
//! and read back the recorded trace — the circuit-building path from an
//! [`OperationalSpec`] and the [`run`] module.
//!
//! Layout: `circuit` is the engine (all physics, no UI), `export` the only JSON
//! path (both directions), `run` the recorder, `app`/`ui`/… the egui shell the
//! binary drives. Only the run-relevant surface is re-exported below; the shell
//! modules stay private to this crate.

mod app;
mod askhal;
mod circuit;
mod docs;
mod examples;
mod export;
mod glyph;
mod ladder;
mod lens;
pub mod run;
#[cfg(test)]
mod sweep;
mod theme;
mod ui;

/// The egui authoring app, for the binary entry point.
pub use app::App;

/// The engine and the node vocabulary a caller reads results against.
pub use circuit::{Circuit, Node, NodeKind};

/// Build a runnable circuit from a validated [`OperationalSpec`] projection.
///
/// [`OperationalSpec`]: bert_core::operational::OperationalSpec
pub use export::from_spec;
