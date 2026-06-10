//! The "what is this?" window — orients a first-time user, names the parts,
//! and states the one promise (everything stays conserved). Opened from the
//! top bar and from the empty-canvas first-run prompt.

use crate::app::App;
use crate::theme::{section_header, ACCENT, GREEN, PRIMARY, SECONDARY};
use egui::RichText;

pub fn show(app: &mut App, ctx: &egui::Context) {
    if !app.show_about {
        return;
    }
    let mut open = true;
    egui::Window::new("What is BERT Compose?")
        .id(egui::Id::new("about-window"))
        .open(&mut open)
        .default_width(440.0)
        .default_pos([280.0, 110.0])
        .collapsible(false)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().max_height(460.0).show(ui, |ui| {
                ui.label(
                    RichText::new("Touch the system.")
                        .color(PRIMARY)
                        .size(15.0)
                        .strong(),
                );
                ui.add_space(4.0);
                ui.label(
                    RichText::new(
                        "Drag a few process primitives onto the canvas, wire them \
                         together, and press Run. Matter, energy, and information \
                         actually flow — and everything stays conserved.",
                    )
                    .color(SECONDARY)
                    .size(12.5),
                );

                ui.add_space(12.0);
                section_header(ui, "THE BRICKS");
                ui.label(
                    RichText::new(
                        "The PRIMITIVES are Mobus's atomic work processes — a tank \
                         (Buffering), a valve (Modulating), a sensor (Sensing), and \
                         so on. They don't decompose further. Click one to add it.",
                    )
                    .color(SECONDARY)
                    .size(12.0),
                );

                ui.add_space(10.0);
                section_header(ui, "THE PROCESSES");
                ui.label(
                    RichText::new(
                        "SYSTEMS PROCESSES are Troncale's patterns — Feedback, \
                         Oscillation, Networks, Potential Fields. They aren't atoms: \
                         each one stamps its primitive circuit onto the canvas, so \
                         you watch the process emerge from the bricks. (A few — \
                         Storage, Flows, Fields — already ARE primitives.)",
                    )
                    .color(SECONDARY)
                    .size(12.0),
                );

                ui.add_space(10.0);
                section_header(ui, "THE PROMISE");
                ui.horizontal(|ui| {
                    ui.label(RichText::new("⚖").color(GREEN).size(14.0));
                    ui.label(
                        RichText::new(
                            "Every run is mass-accounted. The conservation badge by \
                             the clock stays green when nothing leaks — so a curve \
                             here is evidence, not decoration.",
                        )
                        .color(SECONDARY)
                        .size(12.0),
                    );
                });

                ui.add_space(10.0);
                section_header(ui, "THE LENS");
                ui.label(
                    RichText::new(
                        "The same model reads across domains. Switch the 🔍 lens and \
                         a thermostat becomes a quorum, a synapse, a difficulty \
                         adjustment — same dynamics, four readings. The common core, \
                         made visible.",
                    )
                    .color(SECONDARY)
                    .size(12.0),
                );

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(6.0);
                ui.label(
                    RichText::new("Start here:")
                        .color(PRIMARY)
                        .size(12.5)
                        .strong(),
                );
                ui.add_space(4.0);
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("•").color(ACCENT));
                    ui.label(
                        RichText::new("Open the Examples menu and load \"Thermostat\" — press Run.")
                            .color(SECONDARY)
                            .size(12.0),
                    );
                });
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("•").color(ACCENT));
                    ui.label(
                        RichText::new("Or stamp a \"Feedback\" process from the palette and look inside.")
                            .color(SECONDARY)
                            .size(12.0),
                    );
                });
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("•").color(ACCENT));
                    ui.label(
                        RichText::new("Switch the lens while it runs. Save as a BERT model when you like it.")
                            .color(SECONDARY)
                            .size(12.0),
                    );
                });

                ui.add_space(10.0);
                if ui.button("Got it — let me build").clicked() {
                    app.show_about = false;
                }
            });
        });
    if !open {
        app.show_about = false;
    }
}
