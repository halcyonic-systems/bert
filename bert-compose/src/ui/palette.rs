//! Left palette: the primitive bricks.

use crate::app::App;
use crate::circuit::{NodeKind, PALETTE};
use crate::theme::{self, section_header, SECONDARY};
use egui::{pos2, RichText};

pub fn show(app: &mut App, ctx: &egui::Context) {
    egui::SidePanel::left("palette")
        .resizable(false)
        .exact_width(168.0)
        .frame(egui::Frame::new().fill(theme::CREAM).inner_margin(egui::Margin::same(10)))
        .show(ctx, |ui| {
            section_header(ui, "PRIMITIVES");
            ui.add_space(2.0);
            ui.label(
                RichText::new("click to add, drag on canvas").color(SECONDARY).small(),
            );
            ui.add_space(6.0);
            let center = pos2(560.0, 380.0);
            for kind in PALETTE {
                let hover = match kind {
                    NodeKind::Source => "environment input — emits its rate every tick",
                    NodeKind::Sink => "environment output — accumulates what arrives",
                    NodeKind::Process(p) => match format!("{p:?}").as_str() {
                        "Buffering" => "conservative stock — the system's state lives here",
                        "Combining" => "merges physical inflows (Σ)",
                        "Splitting" => "fans out, conserving Material/Energy",
                        "Amplifying" => "signal × gain, bounded by metered Energy — no free mass",
                        "Modulating" => "control Message gates a physical flow (valve, ≤1)",
                        "Sensing" => "reads physical flow, emits Message — crosses substance",
                        "Inverting" => "1 − signal (the controller for negative feedback)",
                        "Copying" => "replicates Message — information copies, matter doesn't",
                        "Propelling" => "pushes flow at efficiency η",
                        "Impeding" => "resists flow (1 − impedance)",
                        _ => "",
                    },
                };
                // Brick reads in the active lens's vocabulary; canonical name
                // stays in the tooltip so the mapping is never lost.
                let brick = crate::lens::label(app.lens, *kind);
                let hover = if app.lens == 0 {
                    hover.to_string()
                } else {
                    format!("{} · {hover}", kind.label())
                };
                if ui
                    .add_sized([146.0, 24.0], egui::Button::new(RichText::new(brick).size(12.0)))
                    .on_hover_text(hover)
                    .clicked()
                {
                    app.add_node(*kind, center);
                }
            }
        });
}
