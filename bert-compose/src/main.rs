//! bert-compose — touch the system: drag process primitives, wire them, and
//! watch matter, energy, and information actually flow.
//!
//! Issue #75's creation experience at its most minimal: the bricks are
//! Mobus's atomic work processes (transfer functions ported from BERT's
//! verified python/agents.py), the wiring is composition (unconditional, by
//! theorem), the stocks hold state — and Save emits ordinary BERT JSON.
//!
//! No error states exist by construction: every wiring action produces a
//! valid system.
//!
//! Layout: `circuit` is the engine (all physics, no UI), `export` the only
//! JSON path (both directions), `app` the state + persistence, `ui/*` one
//! module per panel.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod askhal;
mod circuit;
mod docs;
mod examples;
mod export;
mod glyph;
mod ladder;
mod lens;
#[cfg(test)]
mod sweep;
mod theme;
mod ui;

use app::App;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1240.0, 800.0])
            .with_min_inner_size([920.0, 600.0])
            .with_title("BERT Compose"),
        ..Default::default()
    };
    eframe::run_native("BERT Compose", options, Box::new(|cc| Ok(Box::new(App::new(cc)))))
}
