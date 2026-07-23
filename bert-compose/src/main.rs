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
//! module per panel. The modules themselves live in `lib.rs` (this package is
//! a lib + bin); this file is only the native entry point.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bert_compose::App;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1240.0, 800.0])
            .with_min_inner_size([920.0, 600.0])
            .with_title("BERT Compose"),
        ..Default::default()
    };
    eframe::run_native(
        "BERT Compose",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}
