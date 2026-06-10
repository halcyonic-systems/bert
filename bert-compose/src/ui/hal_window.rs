//! hal's reading of the run — a floating, closable card.

use crate::app::App;
use crate::askhal;
use crate::theme::{self, dot, GREEN, PRIMARY, SECONDARY};
use egui::RichText;

pub fn show(app: &mut App, ctx: &egui::Context) {
    if app.hal_answer.is_none() && !app.hal_busy {
        return;
    }
    let mut open = true;
    egui::Window::new(format!("✦ hal reads your system  ·  {}", app.hal_model))
        .id(egui::Id::new("hal-window"))
        .open(&mut open)
        .default_width(420.0)
        .default_pos([300.0, 130.0])
        .show(ctx, |ui| {
            if app.hal_busy {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label(
                        RichText::new(format!("{} is reading the run…", app.hal_model))
                            .color(SECONDARY),
                    );
                });
                return;
            }
            if let Some(answer) = &app.hal_answer {
                egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                    ui.label(RichText::new(answer).color(PRIMARY).size(13.0));
                });
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    let local = askhal::is_local(&app.hal_model);
                    dot(ui, if local { GREEN } else { theme::AMBER });
                    ui.label(
                        RichText::new(if local {
                            "answered locally — nothing left your machine"
                        } else {
                            "answered by a cloud model"
                        })
                        .color(SECONDARY)
                        .small(),
                    );
                    if ui.small_button("ask again").clicked() {
                        app.ask_hal();
                    }
                });
            }
        });
    if !open {
        app.hal_answer = None;
    }
}
