//! Right inspector: parameters, the substance dictionary, the teaching card,
//! and per-bond flow modes.

use crate::app::App;
use crate::circuit::{self, NodeKind};
use crate::docs;
use crate::theme::{self, section_header, HAIRLINE, PRIMARY, RED, SECONDARY};
use crate::ui::substance_blurb;
use bert_core::SubstanceType;
use egui::RichText;

/// A label : value row for the teaching card.
fn learn_row(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.add_space(3.0);
    ui.label(RichText::new(label).color(SECONDARY).size(10.0));
    ui.label(RichText::new(value).color(PRIMARY).size(11.0));
}

pub fn show(app: &mut App, ctx: &egui::Context) {
    egui::SidePanel::right("inspector")
        .resizable(true)
        .default_width(248.0)
        .width_range(220.0..=360.0)
        .frame(egui::Frame::new().fill(theme::CREAM).inner_margin(egui::Margin::same(10)))
        .show(ctx, |ui| {
            // Everything wraps to the panel width — no clipped text.
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
            egui::ScrollArea::vertical().show(ui, |ui| {
            ui.set_width(ui.available_width());
            section_header(ui, "INSPECTOR");
            ui.add_space(4.0);
            let Some(i) = app.selected else {
                ui.label(
                    RichText::new("select a component").color(SECONDARY).small().italics(),
                );
                return;
            };
            if i >= app.circuit.nodes.len() {
                app.selected = None;
                return;
            }
            let is_buffer = matches!(
                app.circuit.nodes[i].kind,
                NodeKind::Process(bert_core::ProcessPrimitive::Buffering)
            );
            let is_inverting = matches!(
                app.circuit.nodes[i].kind,
                NodeKind::Process(bert_core::ProcessPrimitive::Inverting)
            );
            let param_spec = app.circuit.nodes[i].kind.param_spec();
            let node = &mut app.circuit.nodes[i];
            ui.add(egui::TextEdit::singleline(&mut node.name).desired_width(170.0));
            ui.add_space(4.0);
            // Process parameter (gain / efficiency / rate) — only where the
            // primitive actually has one. Not "agency": that belongs to agents.
            if let Some((label, max)) = param_spec {
                ui.add(egui::Slider::new(&mut node.param, 0.0..=max).text(label));
            }
            // Setpoint: the controller's reference (Mobus Fig 4.12). Raise it
            // to hold the regulated stock at a higher level.
            if is_inverting {
                ui.add(egui::Slider::new(&mut node.setpoint, 0.0..=10.0).text("setpoint"))
                    .on_hover_text(
                        "the reference the controller aims for: output = (setpoint − signal). \
                         Raise it to hold a higher regulated level. Mobus Fig 4.12 (reference − measured).",
                    );
            }
            if is_buffer {
                ui.add(
                    egui::Slider::new(&mut node.release_rate, 0.0..=10.0).text("release / tick"),
                );
                let resp = ui.add(
                    egui::Slider::new(&mut node.initial_storage, 0.0..=50.0)
                        .text("initial stock"),
                );
                if resp.changed() {
                    node.storage = node.initial_storage; // immediate, touchable
                }
                // Capacity: the tank's ceiling. 0 = unbounded (∞); above it,
                // the stock overflows and the excess is dissipated.
                ui.add(
                    egui::Slider::new(&mut node.capacity, 0.0..=50.0)
                        .text("capacity")
                        .custom_formatter(|v, _| {
                            if v < 0.5 { "∞".to_owned() } else { format!("{v:.0}") }
                        }),
                )
                .on_hover_text(
                    "the tank's ceiling — above it the stock overflows (excess \
                     dissipated). 0 = unbounded. Mobus: containers have a capacity.",
                );
            }
            ui.add_space(4.0);
            // The substance choice is only a CHOICE where the substance is a
            // degree of freedom. A Sink absorbs (no output). A signal primitive
            // (Sensing/Inverting/Copying/Amplifying) emits a Message by its
            // definition — locked, not a trichotomy. Everything else carries a
            // domain substance the user picks.
            if matches!(node.kind, NodeKind::Sink) {
                ui.label(RichText::new("absorbs everything").color(SECONDARY).small().italics());
            } else if node.kind.emits_signal() {
                node.out_substance = circuit::DeclaredSubstance::bare(SubstanceType::Message);
                ui.label(RichText::new("emits").color(SECONDARY).small());
                ui.label(RichText::new("signal (Message)").color(PRIMARY).size(11.5));
                ui.label(
                    RichText::new("fixed by this process — a control signal")
                        .color(SECONDARY)
                        .size(10.0)
                        .italics(),
                );
            } else {
            ui.label(RichText::new("emits").color(SECONDARY).small());
            // Substance dictionary: bare kinds, the curated palette, anything
            // declared this session, or declare a new one. Names are for
            // humans; the dynamics run on the conserved base kind.
            let mut choose: Option<circuit::DeclaredSubstance> = None;
            egui::ComboBox::from_id_salt("substance")
                .width(176.0)
                .selected_text(RichText::new(node.out_substance.label()).size(11.0))
                .show_ui(ui, |ui| {
                    for base in
                        [SubstanceType::Energy, SubstanceType::Material, SubstanceType::Message]
                    {
                        let d = circuit::DeclaredSubstance::bare(base);
                        if ui.selectable_label(node.out_substance == d, d.label()).clicked() {
                            choose = Some(d);
                        }
                    }
                    ui.separator();
                    for (name, base, unit) in circuit::SUBSTANCES {
                        let d = circuit::DeclaredSubstance::named(name, *base, unit);
                        if ui
                            .selectable_label(
                                node.out_substance == d,
                                format!("{name}  ·  {base:?}"),
                            )
                            .clicked()
                        {
                            choose = Some(d);
                        }
                    }
                    if !app.declared.is_empty() {
                        ui.separator();
                        for d in &app.declared {
                            if ui
                                .selectable_label(
                                    node.out_substance == *d,
                                    format!("{}  ·  {:?}", d.name, d.base),
                                )
                                .clicked()
                            {
                                choose = Some(d.clone());
                            }
                        }
                    }
                    ui.separator();
                    if ui.selectable_label(app.declaring, "+ declare a substance…").clicked() {
                        app.declaring = true;
                    }
                });
            if let Some(d) = choose {
                node.out_substance = d;
                app.declaring = false;
            }
            if app.declaring {
                ui.add(
                    egui::TextEdit::singleline(&mut app.decl_name)
                        .hint_text("name — e.g. trust, grain, memes")
                        .desired_width(170.0),
                );
                ui.add(
                    egui::TextEdit::singleline(&mut app.decl_unit)
                        .hint_text("unit (optional)")
                        .desired_width(170.0),
                );
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("flows as").color(SECONDARY).size(10.0));
                    for base in
                        [SubstanceType::Energy, SubstanceType::Material, SubstanceType::Message]
                    {
                        if ui
                            .selectable_label(
                                app.decl_base == base,
                                RichText::new(format!("{base:?}")).size(10.5),
                            )
                            .clicked()
                        {
                            app.decl_base = base;
                        }
                    }
                });
                ui.horizontal(|ui| {
                    let ok = !app.decl_name.trim().is_empty();
                    if ui.add_enabled(ok, egui::Button::new("declare")).clicked() {
                        let d = circuit::DeclaredSubstance::named(
                            app.decl_name.trim(),
                            app.decl_base,
                            app.decl_unit.trim(),
                        );
                        if !app.declared.contains(&d) {
                            app.declared.push(d.clone());
                        }
                        node.out_substance = d;
                        app.declaring = false;
                        app.decl_name.clear();
                        app.decl_unit.clear();
                    }
                    if ui.small_button("cancel").clicked() {
                        app.declaring = false;
                    }
                });
            }
            ui.label(
                RichText::new(substance_blurb(node.out_substance.base))
                    .color(SECONDARY)
                    .size(10.0)
                    .italics(),
            );
            } // end else (substance is a free choice)
            ui.add_space(6.0);
            ui.label(
                RichText::new(format!(
                    "activity {:.2}{}",
                    node.activity,
                    if is_buffer { format!(" · stored {:.2}", node.storage) } else { String::new() }
                ))
                .color(SECONDARY)
                .monospace(),
            );
            ui.add_space(8.0);
            // Keyboard ⌫ is the primary delete now (see app.rs); this stays as
            // the discoverable affordance, but quiet — a small link, not a
            // red button — with the shortcut taught beside it.
            ui.horizontal(|ui| {
                if ui
                    .add(egui::Label::new(RichText::new("✕ delete").color(RED).size(11.0))
                        .sense(egui::Sense::click()))
                    .clicked()
                {
                    app.delete_node(i);
                }
                ui.label(RichText::new("or ⌫").color(SECONDARY).size(10.0));
            });
            if app.selected.is_none() {
                return; // node was deleted
            }

            // Teaching card — plain English first, details on demand.
            ui.add_space(10.0);
            let kind = app.circuit.nodes[i].kind;

            // Process provenance — if this node was stamped as part of a
            // Troncale process, describe the PROCESS (not just the brick), so
            // clicking any node of a stamped Feedback loop teaches Feedback.
            if let Some(rung) = app.circuit.nodes[i].process.and_then(crate::ladder::by_name) {
                egui::Frame::new()
                    .fill(theme::GOLD.gamma_multiply(0.10))
                    .stroke(egui::Stroke::new(1.0, theme::GOLD.gamma_multiply(0.5)))
                    .corner_radius(egui::CornerRadius::same(6))
                    .inner_margin(egui::Margin::same(8))
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new(format!("⬡ {} — a systems process", rung.name))
                                .color(theme::GOLD)
                                .size(11.5)
                                .family(theme::semibold()),
                        );
                        ui.add_space(2.0);
                        ui.label(RichText::new(rung.blurb).color(PRIMARY).size(11.5));
                        ui.add_space(3.0);
                        ui.label(
                            RichText::new(format!("built from: {}", rung.composition))
                                .color(SECONDARY)
                                .size(10.5)
                                .italics(),
                        );
                        ui.label(
                            RichText::new(format!("Troncale: {}", rung.provenance))
                                .color(SECONDARY)
                                .size(10.0),
                        );
                    });
                ui.add_space(4.0);
                ui.label(
                    RichText::new("This node's role in it:")
                        .color(SECONDARY)
                        .size(10.5)
                        .italics(),
                );
                ui.add_space(2.0);
            }

            // Under a domain lens, name the reading: "Receptor · a Sensing
            // primitive" — the renaming and what it really is, side by side.
            if app.lens != 0 {
                ui.label(
                    RichText::new(format!(
                        "{} · a {} primitive",
                        crate::lens::label(app.lens, kind),
                        kind.label()
                    ))
                    .color(theme::ACCENT)
                    .size(11.0),
                );
                if let Some(g) = crate::lens::gloss(app.lens, kind) {
                    ui.label(RichText::new(g).color(SECONDARY).size(11.0).italics());
                }
                ui.add_space(2.0);
            }
            let d = docs::doc(kind);
            ui.label(RichText::new(d.plain).color(PRIMARY).size(12.0));
            ui.add_space(2.0);
            ui.label(RichText::new(format!("e.g. {}", d.everyday)).color(SECONDARY).size(11.0).italics());
            egui::CollapsingHeader::new(RichText::new("how it works").color(SECONDARY).size(11.0))
                .id_salt("learn")
                .show(ui, |ui| {
                    learn_row(ui, "rule", d.math);
                    learn_row(ui, "substance", d.substance);
                    learn_row(ui, "theory", d.theory);
                    ui.add_space(2.0);
                    ui.label(RichText::new("transfer function").color(SECONDARY).size(10.0));
                    egui::Frame::new()
                        .fill(theme::INPUT_BG)
                        .stroke(egui::Stroke::new(1.0, HAIRLINE))
                        .corner_radius(egui::CornerRadius::same(6))
                        .inner_margin(egui::Margin::same(7))
                        .show(ui, |ui| {
                            ui.label(RichText::new(d.code).monospace().size(10.5).color(PRIMARY));
                        });
                });

            // Wires touching this node: remove, and set flow mode (pushed vs
            // gradient = Potential Field) + conductance per outgoing wire.
            ui.add_space(10.0);
            section_header(ui, "BONDS");
            ui.add_space(2.0);
            let mut remove: Option<usize> = None;
            let names: Vec<(usize, usize, String, String)> = app
                .circuit
                .wires
                .iter()
                .enumerate()
                .filter(|(_, w)| w.from == i || w.to == i)
                .map(|(k, w)| {
                    (k, w.from, app.circuit.nodes[w.from].name.clone(), app.circuit.nodes[w.to].name.clone())
                })
                .collect();
            for (k, from, fname, tname) in names {
                ui.horizontal(|ui| {
                    if ui.small_button("✕").clicked() {
                        remove = Some(k);
                    }
                    ui.label(RichText::new(format!("{fname} → {tname}")).color(PRIMARY).size(11.0));
                });
                // Mode toggle only for outgoing wires (the flow's rate law),
                // and only where a gradient is meaningful: a field needs a
                // potential to fall from, which only Sources and stocks have.
                if from == i && app.circuit.has_potential(i) {
                    ui.horizontal(|ui| {
                        ui.add_space(18.0);
                        let w = &mut app.circuit.wires[k];
                        let mut gradient = w.mode == circuit::FlowMode::Gradient;
                        if ui
                            .selectable_label(
                                gradient,
                                RichText::new("gradient (field)").size(10.0),
                            )
                            .on_hover_text(
                                "flow runs down a potential difference: rate = k·(Δlevel). \
                                 Self-regulating — slows as the two stocks equalize.",
                            )
                            .clicked()
                        {
                            gradient = !gradient;
                            w.mode = if gradient {
                                circuit::FlowMode::Gradient
                            } else {
                                circuit::FlowMode::Pushed
                            };
                        }
                        if gradient {
                            ui.add(
                                egui::Slider::new(&mut w.conductance, 0.0..=1.0)
                                    .text("k")
                                    .fixed_decimals(2),
                            );
                        }
                    });
                }
            }
            if let Some(k) = remove {
                app.circuit.wires.remove(k);
            }
            }); // ScrollArea
        });
}
