//! Live metrics panel — NetLogo/Mesa-style plots of the recorded run, one line
//! per node, drawn from circuit.history (column layout: [tick, a,s,t per node]).

use crate::app::App;
use crate::theme::{self, section_header, ACCENT, GREEN, PAPER, RED, SECONDARY};
use egui::{Color32, RichText};

pub fn show(app: &mut App, ctx: &egui::Context) {
    if !app.show_charts {
        return;
    }
    egui::TopBottomPanel::bottom("charts")
        .resizable(true)
        .default_height(210.0)
        .frame(egui::Frame::new().fill(PAPER).inner_margin(egui::Margin::same(10)))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                section_header(ui, "METRICS");
                for (i, name) in ["activity", "storage", "cumulative"].iter().enumerate() {
                    if ui.selectable_label(app.chart_metric == i, *name).clicked() {
                        app.chart_metric = i;
                    }
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(format!("{} ticks recorded", app.circuit.history.len()))
                            .color(SECONDARY)
                            .small(),
                    );
                });
            });
            ui.add_space(2.0);

            if app.circuit.history.is_empty() {
                ui.add_space(40.0);
                ui.vertical_centered(|ui| {
                    ui.label(
                        RichText::new("press Run or Step — lines plot here as the system flows")
                            .color(SECONDARY)
                            .italics(),
                    );
                });
                return;
            }

            let palette = [
                ACCENT,
                GREEN,
                theme::AMBER,
                Color32::from_rgb(146, 100, 156),
                RED,
                Color32::from_rgb(90, 140, 160),
                Color32::from_rgb(150, 110, 70),
            ];
            let offset = 1 + app.chart_metric; // col within each node's triple
            egui_plot::Plot::new("metrics")
                .height(ui.available_height())
                .legend(egui_plot::Legend::default())
                .show_axes([true, true])
                .show_grid(true)
                .show(ui, |plot_ui| {
                    for (n, node) in app.circuit.nodes.iter().enumerate() {
                        // Skip flat-zero series to keep the legend meaningful.
                        let col = 1 + n * 3 + offset;
                        let pts: Vec<[f64; 2]> = app
                            .circuit
                            .history
                            .iter()
                            .filter(|r| col < r.len())
                            .map(|r| [r[0] as f64, r[col] as f64])
                            .collect();
                        if pts.iter().all(|p| p[1].abs() < 1e-6) {
                            continue;
                        }
                        plot_ui.line(
                            egui_plot::Line::new(pts)
                                .name(node.name.clone())
                                .color(palette[n % palette.len()])
                                .width(1.8),
                        );
                    }
                });
        });
}
