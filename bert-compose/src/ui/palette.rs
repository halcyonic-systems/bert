//! Left palette: the primitive bricks + the systems-process macros.
//!
//! Color = meaning (the shell's discipline). Each brick is tinted by its
//! ROLE family, so the palette reads as a legend: green = environment,
//! gold = store, blue = signal, violet = regulation, amber = transport,
//! slate = matter transform. The process macros are gold (composites).

use crate::app::App;
use crate::circuit::{NodeKind, PALETTE};
use crate::theme::{self, section_header, ACCENT, GOLD, GREEN, PRIMARY, SECONDARY};
use bert_core::ProcessPrimitive::*;
use egui::{pos2, Color32, CornerRadius, RichText, Stroke};

/// A primitive's role-family color — the legend the palette teaches.
fn family_color(kind: &NodeKind) -> Color32 {
    match kind {
        NodeKind::Source | NodeKind::Sink => GREEN, // environment
        NodeKind::Process(Buffering) => GOLD,       // store
        NodeKind::Process(Combining | Splitting) => Color32::from_rgb(120, 120, 126), // matter
        NodeKind::Process(Sensing | Inverting | Copying | Amplifying) => ACCENT, // signal
        NodeKind::Process(Modulating) => Color32::from_rgb(146, 100, 156), // regulate
        NodeKind::Process(Propelling | Impeding) => Color32::from_rgb(163, 121, 47), // transport
    }
}

/// A soft tinted "chip" button in a family color. Resting fill is a light
/// wash; the stroke carries the hue. Hover is deepened by the caller.
fn chip(label: &str, fam: Color32) -> egui::Button<'static> {
    egui::Button::new(RichText::new(label.to_owned()).size(12.0).color(PRIMARY))
        .fill(fam.gamma_multiply(0.15))
        .stroke(Stroke::new(1.0, fam.gamma_multiply(0.55)))
        .corner_radius(CornerRadius::same(6))
}

pub fn show(app: &mut App, ctx: &egui::Context) {
    egui::SidePanel::left("palette")
        .resizable(false)
        .exact_width(168.0)
        .frame(egui::Frame::new().fill(theme::CREAM).inner_margin(egui::Margin::same(10)))
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
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
                let w = ui.available_width();
                let fam = family_color(kind);
                let resp = ui.add_sized([w, 24.0], chip(&brick, fam)).on_hover_text(hover);
                if resp.hovered() {
                    ui.painter().rect_filled(
                        resp.rect,
                        CornerRadius::same(6),
                        fam.gamma_multiply(0.12),
                    );
                }
                if resp.clicked() {
                    app.add_node(*kind, center);
                }
            }

            // ── Troncale's systems processes, as stampable macros ──────────
            // These are NOT atoms — each stamps its primitive circuit onto the
            // canvas. The honesty is the point: you watch the process emerge
            // from the bricks above.
            ui.add_space(12.0);
            ui.separator();
            // Gold header — the composites stand apart from the atoms above.
            ui.label(
                RichText::new("PROCESSES")
                    .color(GOLD)
                    .size(10.0)
                    .family(theme::semibold())
                    .extra_letter_spacing(1.4),
            );
            ui.add_space(1.0);
            ui.label(
                RichText::new("Troncale's processes — each stamps its primitive circuit, not an atom")
                    .color(SECONDARY)
                    .size(9.5),
            );
            ui.add_space(5.0);
            let mut stamp: Option<&crate::ladder::Rung> = None;
            for rung in crate::ladder::palette_macros() {
                let w = ui.available_width();
                let resp = ui
                    .add_sized([w, 24.0], chip(rung.name, GOLD))
                    .on_hover_text(format!("{}\n\n↳ {}", rung.blurb, rung.composition));
                if resp.hovered() {
                    ui.painter().rect_filled(resp.rect, CornerRadius::same(6), GOLD.gamma_multiply(0.12));
                }
                if resp.clicked() {
                    stamp = Some(rung);
                }
            }
            if let Some(rung) = stamp {
                app.stamp_macro(rung);
            }
            }); // ScrollArea
        });
}
