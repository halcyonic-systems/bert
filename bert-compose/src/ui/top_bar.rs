//! Top bar: run controls, the conservation indicator, examples, load/save,
//! Ask hal.

use crate::app::App;
use crate::theme::{self, primary_button, semibold, GREEN, PAPER, PRIMARY, SECONDARY};
use crate::{askhal, examples, lens};
use egui::RichText;

pub fn show(app: &mut App, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top")
        .frame(
            egui::Frame::new()
                .fill(PAPER)
                .inner_margin(egui::Margin { left: 14, right: 14, top: 10, bottom: 10 }),
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("BERT").color(PRIMARY).family(semibold()).size(15.0));
                ui.add_space(-4.0);
                ui.label(RichText::new("COMPOSE").color(SECONDARY).size(15.0));
                ui.add_space(10.0);
                ui.add(
                    egui::TextEdit::singleline(&mut app.name)
                        .desired_width(180.0)
                        .margin(egui::Margin::symmetric(8, 4)),
                );
                ui.add_space(8.0);
                let run_label = if app.running { "Pause" } else { "Run" };
                if ui.add(primary_button(run_label)).clicked() {
                    app.running = !app.running;
                    if !app.running {
                        app.write_latest(); // pausing publishes the run
                    }
                }
                if ui
                    .add_enabled(!app.running, egui::Button::new("Step"))
                    .on_hover_text("advance exactly one tick")
                    .clicked()
                {
                    app.circuit.step();
                }
                if ui.button("Reset").clicked() {
                    app.circuit.reset();
                }
                ui.add(
                    egui::Slider::new(&mut app.ticks_per_sec, 1.0..=20.0)
                        .text("ticks/s")
                        .fixed_decimals(0),
                );
                ui.label(
                    RichText::new(format!("t = {}", app.circuit.tick))
                        .color(SECONDARY)
                        .monospace(),
                );
                // Conservation ledger — headline placement, next to the clock.
                // Green = every unit of physical mass accounted; amber = leak
                // (or a mid-run stock edit moved the baseline).
                if app.circuit.tick > 0 {
                    let c = &app.circuit;
                    let baseline: f32 = c.nodes.iter().map(|n| n.initial_storage).sum();
                    let residual = c.balance();
                    let ok = residual.abs() <= 0.01 * (c.emitted + baseline).max(1.0);
                    let label = if ok {
                        RichText::new("⚖ conserved").color(GREEN)
                    } else {
                        RichText::new(format!("⚖ off by {residual:+.2}")).color(theme::AMBER)
                    };
                    ui.label(label).on_hover_text(format!(
                        "emitted {:.2} + initial stocks {:.2}  =  stored {:.2} + sunk {:.2} \
                         + in flight {:.2} + dissipated {:.2}  (residual {:+.3})\n\
                         Dissipation = friction, valve shed, amp power, sensing, mismatches, \
                         dead ends — each intended and counted.\n\
                         Edited a stock mid-run? That moves the baseline — Reset re-balances.",
                        c.emitted,
                        baseline,
                        c.stored(),
                        c.sunk,
                        c.in_flight(),
                        c.dissipated,
                        residual,
                    ));
                }
                ui.toggle_value(&mut app.show_charts, "📈 Charts");
                let mut load: Option<usize> = None;
                ui.menu_button("Examples ▾", |ui| {
                    for (i, ex) in examples::EXAMPLES.iter().enumerate() {
                        if ui
                            .add(egui::Button::new(ex.name))
                            .on_hover_text(ex.blurb)
                            .clicked()
                        {
                            load = Some(i);
                            ui.close_menu();
                        }
                    }
                });
                if let Some(i) = load {
                    app.load_example(&examples::EXAMPLES[i]);
                }
                // Lens picker — the SAME circuit read in domain vocabulary.
                egui::ComboBox::from_id_salt("lens")
                    .selected_text(format!("🔍 {}", lens::LENSES[app.lens].name))
                    .show_ui(ui, |ui| {
                        for (i, l) in lens::LENSES.iter().enumerate() {
                            if ui
                                .selectable_label(app.lens == i, l.name)
                                .on_hover_text(l.tagline)
                                .clicked()
                            {
                                app.lens = i;
                            }
                        }
                    });
                if app.lens != 0 {
                    ui.label(
                        RichText::new(lens::LENSES[app.lens].tagline)
                            .color(SECONDARY)
                            .small()
                            .italics(),
                    );
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.add(primary_button("Save as BERT model")).clicked() {
                        app.save();
                    }
                    if ui
                        .button("Load")
                        .on_hover_text(
                            "open a saved BERT model back onto the canvas — \
                             or drag a .json anywhere into the window",
                        )
                        .clicked()
                    {
                        app.load_dialog();
                    }
                    // Ask hal — sovereign in-app analysis of the run.
                    let can_ask = !app.circuit.history.is_empty() && !app.hal_busy;
                    let label = if app.hal_busy { "hal thinking…" } else { "✦ Ask hal" };
                    if ui
                        .add_enabled(can_ask, primary_button(label))
                        .on_hover_text("analyze this run with a local model — nothing leaves your machine")
                        .clicked()
                    {
                        app.ask_hal();
                    }
                    egui::ComboBox::from_id_salt("hal-model")
                        .width(118.0)
                        .selected_text(RichText::new(&app.hal_model).small())
                        .show_ui(ui, |ui| {
                            for m in askhal::MODELS {
                                let tag = if askhal::is_local(m) { "local" } else { "cloud" };
                                ui.selectable_value(
                                    &mut app.hal_model,
                                    m.to_string(),
                                    format!("{m}  ·  {tag}"),
                                );
                            }
                        });
                    if ui
                        .add_enabled(
                            !app.circuit.history.is_empty(),
                            egui::Button::new("Export CSV"),
                        )
                        .on_hover_text("write the recorded run (per-tick data) to ~/Desktop")
                        .clicked()
                    {
                        app.export_csv();
                    }
                });
            });
        });
}
