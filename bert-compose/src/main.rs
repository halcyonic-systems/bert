//! bert-compose — compositional creation over the bert-core kernel.
//!
//! The issue #75 spike ("LEGO with proofs"): pick two models, bind outputs to
//! inputs, and compose. The algebra is the proven one — composition is
//! unconditional, bound external pairs become internal bonds, everything
//! unbound provably survives across the new boundary
//! (systems-science-foundations, Systems/Mobus/Composition.lean).
//!
//! Pure kernel surface: depends on bert-core only — no Bevy, no Leptos, no
//! Tauri. The composite saves as ordinary BERT JSON and opens in the editor.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod compose;
mod theme;

use bert_core::validate::{validate, Severity, ValidationResult};
use bert_core::WorldModel;
use compose::{compose, suggest_bindings, Binding, Emergence};
use eframe::egui;
use egui::RichText;
use theme::{
    card, dot, pill, primary_button, section_header, semibold, ACCENT, ACCENT_SOFT, GOLD, GREEN,
    GREEN_SOFT, PRIMARY, RED, SECONDARY,
};

/// Bundled example models, embedded so the .app needs no filesystem layout.
const EXAMPLES: &[(&str, &str)] = &[
    ("bitcoin", include_str!("../../assets/models/examples/bitcoin.json")),
    ("ethereum", include_str!("../../assets/models/examples/ethereum.json")),
    ("cosmos-hub", include_str!("../../assets/models/examples/cosmos-hub.json")),
    ("solana", include_str!("../../assets/models/examples/solana.json")),
    ("llm", include_str!("../../assets/models/examples/llm.json")),
];

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

struct App {
    models: Vec<(String, WorldModel)>,
    a_idx: usize,
    b_idx: usize,
    /// Suggested bindings with an enabled toggle each.
    bindings: Vec<(Binding, bool)>,
    composite: Option<(WorldModel, Emergence, ValidationResult)>,
    composite_name: String,
    status: String,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        theme::apply(&cc.egui_ctx);
        let models: Vec<(String, WorldModel)> = EXAMPLES
            .iter()
            .filter_map(|(name, json)| {
                serde_json::from_str(json).ok().map(|m| (name.to_string(), m))
            })
            .collect();
        let mut app = Self {
            models,
            a_idx: 0,
            b_idx: 1,
            bindings: Vec::new(),
            composite: None,
            composite_name: String::new(),
            status: "pick two models, review the suggested bindings, compose".to_string(),
        };
        app.refresh_suggestions();
        app
    }

    fn a(&self) -> &WorldModel {
        &self.models[self.a_idx].1
    }

    fn b(&self) -> &WorldModel {
        &self.models[self.b_idx].1
    }

    fn refresh_suggestions(&mut self) {
        self.bindings =
            suggest_bindings(self.a(), self.b()).into_iter().map(|b| (b, true)).collect();
        self.composite = None;
        self.composite_name =
            format!("{} × {}", self.models[self.a_idx].0, self.models[self.b_idx].0);
    }

    fn run_compose(&mut self) {
        let enabled: Vec<Binding> =
            self.bindings.iter().filter(|(_, on)| *on).map(|(b, _)| b.clone()).collect();
        let (composite, emergence) = compose(self.a(), self.b(), &enabled, &self.composite_name);
        let result = validate(&composite);
        let errors = result.issues.iter().filter(|i| i.severity == Severity::Error).count();
        self.status = if errors == 0 {
            format!(
                "composed: {} bond(s) emerged, {} externals internalized, {} survive · validates clean",
                emergence.internal_bonds.len(),
                emergence.internalized.len(),
                emergence.surviving.len()
            )
        } else {
            format!("composed with {errors} validation error(s) — see verdict")
        };
        self.composite = Some((composite, emergence, result));
    }

    fn save(&mut self) {
        let Some((composite, ..)) = &self.composite else { return };
        let home = std::env::var("HOME").unwrap_or_default();
        let fname = self.composite_name.replace(" × ", "-x-").replace(' ', "-");
        let path = format!("{home}/Desktop/{fname}.json");
        match serde_json::to_string_pretty(composite)
            .map_err(|e| e.to_string())
            .and_then(|s| std::fs::write(&path, s).map_err(|e| e.to_string()))
        {
            Ok(()) => self.status = format!("saved {path} — open it in BERT"),
            Err(e) => self.status = format!("save failed: {e}"),
        }
    }

    /// Display data for a binding: (sink ⇒ source, direction, substance).
    fn binding_label(&self, b: &Binding) -> (String, String, String) {
        let (out_m, in_m, dir) = if b.a_to_b {
            (
                self.a(),
                self.b(),
                format!("{} → {}", self.models[self.a_idx].0, self.models[self.b_idx].0),
            )
        } else {
            (
                self.b(),
                self.a(),
                format!("{} → {}", self.models[self.b_idx].0, self.models[self.a_idx].0),
            )
        };
        let sink = out_m
            .environment
            .sinks
            .get(b.sink_idx)
            .map(|e| e.info.name.clone())
            .unwrap_or_default();
        let source = in_m
            .environment
            .sources
            .get(b.source_idx)
            .map(|e| e.info.name.clone())
            .unwrap_or_default();
        let substance = out_m
            .environment
            .sinks
            .get(b.sink_idx)
            .and_then(|e| out_m.interactions.iter().find(|f| f.sink == e.info.id))
            .map(|f| format!("{:?}", f.substance.ty))
            .unwrap_or_default();
        (format!("{sink} ⇒ {source}"), dir, substance)
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        top_bar(self, ctx);
        status_bar(self, ctx);
        model_sidebar(self, ctx);
        workbench(self, ctx);
    }
}

fn top_bar(app: &mut App, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top")
        .frame(
            egui::Frame::new()
                .fill(theme::PAPER)
                .inner_margin(egui::Margin { left: 14, right: 14, top: 10, bottom: 10 }),
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("BERT").color(PRIMARY).family(semibold()).size(15.0));
                ui.add_space(-4.0);
                ui.label(RichText::new("COMPOSE").color(SECONDARY).size(15.0));
                ui.add_space(8.0);
                ui.label(
                    RichText::new("LEGO with proofs — composition is unconditional")
                        .color(SECONDARY)
                        .size(11.0)
                        .italics(),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let can_save = app.composite.is_some();
                    if ui
                        .add_enabled(can_save, primary_button("Save composite"))
                        .on_hover_text("write BERT JSON to ~/Desktop — opens in the editor")
                        .clicked()
                    {
                        app.save();
                    }
                    if ui.add(primary_button("Compose")).clicked() {
                        app.run_compose();
                    }
                });
            });
        });
}

fn status_bar(app: &App, ctx: &egui::Context) {
    egui::TopBottomPanel::bottom("status")
        .frame(
            egui::Frame::new()
                .fill(theme::PAPER)
                .inner_margin(egui::Margin { left: 14, right: 14, top: 5, bottom: 5 }),
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                let ok = app
                    .composite
                    .as_ref()
                    .map(|(_, _, r)| !r.issues.iter().any(|i| i.severity == Severity::Error))
                    .unwrap_or(true);
                dot(ui, if ok { GREEN } else { RED });
                ui.label(RichText::new(&app.status).color(SECONDARY).small());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new("algebra: Systems/Mobus/Composition.lean · machine-verified")
                            .color(SECONDARY)
                            .small(),
                    );
                });
            });
        });
}

fn model_sidebar(app: &mut App, ctx: &egui::Context) {
    egui::SidePanel::left("models")
        .resizable(false)
        .exact_width(230.0)
        .frame(egui::Frame::new().fill(theme::CREAM).inner_margin(egui::Margin::same(10)))
        .show(ctx, |ui| {
            let mut changed = false;
            for (label, idx) in [("SYSTEM A", 0usize), ("SYSTEM B", 1usize)] {
                section_header(ui, label);
                ui.add_space(2.0);
                let mut new_sel = if idx == 0 { app.a_idx } else { app.b_idx };
                let current = app.models[new_sel].0.clone();
                egui::ComboBox::from_id_salt(("model", idx))
                    .width(200.0)
                    .selected_text(RichText::new(current).size(12.5))
                    .show_ui(ui, |ui| {
                        for (i, (name, _)) in app.models.iter().enumerate() {
                            if ui.selectable_value(&mut new_sel, i, name).changed() {
                                changed = true;
                            }
                        }
                    });
                if idx == 0 {
                    app.a_idx = new_sel;
                } else {
                    app.b_idx = new_sel;
                }
                let m = &app.models[new_sel].1;
                ui.add_space(2.0);
                ui.label(
                    RichText::new(format!(
                        "{} systems · {} flows · {} in / {} out",
                        m.systems.len(),
                        m.interactions.len(),
                        m.environment.sources.len(),
                        m.environment.sinks.len(),
                    ))
                    .color(SECONDARY)
                    .small(),
                );
                ui.add_space(10.0);
            }
            if changed {
                app.refresh_suggestions();
            }

            section_header(ui, "HOW IT WORKS");
            ui.add_space(2.0);
            ui.label(
                RichText::new(
                    "Bind an output of one system to an input of the other. Each bound \
                     pair becomes an internal bond; both externals are internalized. \
                     Everything unbound survives to the composite's environment — \
                     guaranteed by bipartite_edge_classification.",
                )
                .color(SECONDARY)
                .size(11.0),
            );
        });
}

fn workbench(app: &mut App, ctx: &egui::Context) {
    egui::CentralPanel::default()
        .frame(egui::Frame::new().fill(theme::CREAM).inner_margin(egui::Margin::same(12)))
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                ui.horizontal(|ui| {
                    section_header(ui, "BINDINGS");
                    ui.label(
                        RichText::new("outputs wired to inputs — substance types must agree")
                            .color(SECONDARY)
                            .small(),
                    );
                });
                ui.add_space(4.0);
                if app.bindings.is_empty() {
                    ui.label(
                        RichText::new(
                            "no substance-compatible sink/source pairs between these two — \
                             composing yields a disjoint union under a new root (still valid: \
                             composition is unconditional)",
                        )
                        .color(SECONDARY)
                        .italics()
                        .size(12.0),
                    );
                }
                let labels: Vec<_> =
                    app.bindings.iter().map(|(b, _)| app.binding_label(b)).collect();
                for (i, (label, dir, substance)) in labels.iter().enumerate() {
                    let on = &mut app.bindings[i].1;
                    ui.horizontal(|ui| {
                        ui.checkbox(on, "");
                        ui.label(
                            RichText::new(label)
                                .color(if *on { PRIMARY } else { SECONDARY })
                                .size(13.0),
                        );
                        pill(ui, substance, ACCENT, ACCENT_SOFT);
                        ui.label(RichText::new(dir).color(SECONDARY).small());
                    });
                }

                if let Some((composite, emergence, result)) = &app.composite {
                    ui.add_space(14.0);
                    render_verdict(ui, composite, emergence, result);
                }
            });
        });
}

fn render_verdict(
    ui: &mut egui::Ui,
    composite: &WorldModel,
    emergence: &Emergence,
    result: &ValidationResult,
) {
    let errors: Vec<_> = result.issues.iter().filter(|i| i.severity == Severity::Error).collect();
    card()
        .stroke(egui::Stroke::new(1.2, if errors.is_empty() { GOLD } else { RED }))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                section_header(ui, "WHAT EMERGED");
                ui.label(
                    RichText::new(&composite.systems[0].info.name)
                        .color(PRIMARY)
                        .family(semibold())
                        .size(14.0),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if errors.is_empty() {
                        pill(ui, "validates clean", GREEN, GREEN_SOFT);
                    } else {
                        pill(ui, &format!("{} errors", errors.len()), egui::Color32::WHITE, RED);
                    }
                    pill(
                        ui,
                        &format!("{} systems", composite.systems.len()),
                        SECONDARY,
                        theme::HOVER,
                    );
                });
            });
            ui.add_space(6.0);

            for bond in &emergence.internal_bonds {
                ui.horizontal(|ui| {
                    dot(ui, GREEN);
                    ui.label(RichText::new(bond).color(PRIMARY).size(12.5));
                });
            }
            if emergence.internal_bonds.is_empty() {
                ui.label(RichText::new("no new bonds — disjoint union").color(SECONDARY).small());
            }
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                pill(
                    ui,
                    &format!("{} externals internalized", emergence.internalized.len()),
                    ACCENT,
                    ACCENT_SOFT,
                );
                pill(
                    ui,
                    &format!("{} boundary flows survive", emergence.surviving.len()),
                    SECONDARY,
                    theme::HOVER,
                );
            });
            if !emergence.surviving.is_empty() {
                ui.add_space(2.0);
                ui.label(
                    RichText::new(format!("surviving: {}", emergence.surviving.join(" · ")))
                        .color(SECONDARY)
                        .size(11.0),
                );
            }
            for issue in errors.iter().take(6) {
                ui.label(
                    RichText::new(format!("✗ {} — {}", issue.location, issue.message))
                        .color(RED)
                        .size(11.5),
                );
            }
            ui.add_space(4.0);
            ui.separator();
            ui.label(
                RichText::new(
                    "every move above is licensed: bound pairs reclassified internal, survivors \
                     cross the new boundary — bipartite_edge_classification, \
                     Systems/Mobus/Composition.lean (machine-verified; composition unconditional)",
                )
                .color(SECONDARY)
                .size(10.5)
                .italics(),
            );
        });

    ui.add_space(10.0);
    section_header(ui, "COMPOSITE STRUCTURE");
    ui.add_space(2.0);
    for sys in &composite.systems {
        let depth = sys.info.id.indices.len().saturating_sub(1);
        ui.horizontal(|ui| {
            ui.add_space(12.0 * depth as f32);
            ui.label(
                RichText::new(if depth == 0 { "▣" } else { "▢" })
                    .color(if depth == 0 { GOLD } else { ACCENT })
                    .small(),
            );
            ui.label(RichText::new(&sys.info.name).color(PRIMARY).size(12.0));
        });
    }
}
