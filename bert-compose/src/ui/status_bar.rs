//! Bottom status bar: the action log on the left, advisories (mismatch,
//! underpowered amp, inert gradient, dead end) or the validity line on the
//! right.

use crate::app::App;
use crate::theme::{self, dot, GREEN, PAPER, SECONDARY};
use egui::RichText;

pub fn show(app: &App, ctx: &egui::Context) {
    egui::TopBottomPanel::bottom("status")
        .frame(
            egui::Frame::new()
                .fill(PAPER)
                .inner_margin(egui::Margin { left: 14, right: 14, top: 5, bottom: 5 }),
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                dot(ui, GREEN);
                ui.label(RichText::new(&app.status).color(SECONDARY).small());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let mismatches = app.circuit.substance_mismatches();
                    let underpowered = app.circuit.underpowered_amplifiers();
                    let inert = app.circuit.inert_gradient_wires();
                    let dead = app.circuit.dead_ends();
                    if let Some((i, wants, got)) = mismatches.first() {
                        ui.label(
                            RichText::new(format!(
                                "⚠ {} consumes {wants:?} but is fed {} — that flow is ignored. Change the source's substance or insert a Sensing transducer.",
                                app.circuit.nodes[*i].name,
                                got.label(),
                            ))
                            .color(theme::AMBER)
                            .small(),
                        );
                    } else if let Some(i) = underpowered.first() {
                        ui.label(
                            RichText::new(format!(
                                "⚠ {} has a signal but no Energy power — amplification is bounded to 0. Wire an Energy source into it.",
                                app.circuit.nodes[*i].name,
                            ))
                            .color(theme::AMBER)
                            .small(),
                        );
                    } else if let Some(k) = inert.first() {
                        let w = &app.circuit.wires[*k];
                        ui.label(
                            RichText::new(format!(
                                "⚠ gradient {} → {} carries nothing — a field needs a potential to fall from. Only Sources and stocks have levels; switch it to pushed.",
                                app.circuit.nodes[w.from].name, app.circuit.nodes[w.to].name,
                            ))
                            .color(theme::AMBER)
                            .small(),
                        );
                    } else if let Some(i) = dead.first() {
                        ui.label(
                            RichText::new(format!(
                                "⚠ {}'s output goes nowhere — it evaporates each tick. Wire it onward or add a Sink.",
                                app.circuit.nodes[*i].name,
                            ))
                            .color(theme::AMBER)
                            .small(),
                        );
                    } else {
                        ui.label(
                            RichText::new(format!(
                                "{} components · {} bonds · diversity {} · always valid — composition is unconditional",
                                app.circuit.nodes.len(),
                                app.circuit.wires.len(),
                                app.circuit.diversity(),
                            ))
                            .color(SECONDARY)
                            .small(),
                        );
                    }
                });
            });
        });
}
