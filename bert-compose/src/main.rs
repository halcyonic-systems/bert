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

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod askhal;
mod circuit;
mod docs;
mod examples;
mod export;
mod glyph;
mod theme;

use bert_core::SubstanceType;
use circuit::{Circuit, Node, NodeKind, Wire, PALETTE};
use eframe::egui;
use egui::{pos2, vec2, Color32, Pos2, RichText, Sense, Stroke};
use theme::{
    dot, pill, primary_button, section_header, semibold, ACCENT, ACCENT_SOFT, GOLD, GREEN,
    GREEN_SOFT, HAIRLINE, PAPER, PRIMARY, RED, SECONDARY,
};

/// One-line physics of a base kind — shown under the substance picker so a
/// declared name ("money") carries its conservation law with it.
fn substance_blurb(base: SubstanceType) -> &'static str {
    match base {
        SubstanceType::Energy => "conserved — splits across fanouts, meters the amplifier",
        SubstanceType::Material => "conserved — splits and stores, never copies",
        SubstanceType::Message => "information — copies freely, gates and signals, not conserved",
    }
}

// BERT substance colors (message lifted to accent for visibility on cream).
fn substance_color(s: SubstanceType) -> Color32 {
    match s {
        SubstanceType::Energy => Color32::from_rgb(181, 27, 27),
        SubstanceType::Material => Color32::from_rgb(120, 120, 126),
        SubstanceType::Message => ACCENT,
    }
}

const NODE_R: f32 = 27.0;

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
    circuit: Circuit,
    name: String,
    selected: Option<usize>,
    /// Wire in progress: source node index.
    pending_wire: Option<usize>,
    running: bool,
    ticks_per_sec: f32,
    last_tick_at: f64,
    next_n: usize,
    status: String,
    show_charts: bool,
    /// Which metric the plots track: 0 activity, 1 storage, 2 cumulative total.
    chart_metric: usize,
    // — substance dictionary —
    /// Substances free-declared this session (pickable on any node).
    declared: Vec<circuit::DeclaredSubstance>,
    declaring: bool,
    decl_name: String,
    decl_unit: String,
    decl_base: SubstanceType,
    // — Ask hal (sovereign in-app analysis) —
    hal_model: String,
    hal_rx: Option<std::sync::mpsc::Receiver<Result<String, String>>>,
    hal_answer: Option<String>,
    hal_busy: bool,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        theme::apply(&cc.egui_ctx);
        Self {
            circuit: Circuit::default(),
            name: "My System".to_string(),
            selected: None,
            pending_wire: None,
            running: false,
            ticks_per_sec: 4.0,
            last_tick_at: 0.0,
            next_n: 1,
            status: "add primitives from the palette, wire ◦ → node, press Run".to_string(),
            show_charts: true,
            chart_metric: 0,
            declared: Vec::new(),
            declaring: false,
            decl_name: String::new(),
            decl_unit: String::new(),
            decl_base: SubstanceType::Material,
            hal_model: "llama3".to_string(),
            hal_rx: None,
            hal_answer: None,
            hal_busy: false,
        }
    }

    fn ask_hal(&mut self) {
        self.write_latest(); // make sure the digest is current
        self.hal_busy = true;
        self.hal_answer = None;
        self.hal_rx = Some(askhal::ask(self.run_summary(), self.hal_model.clone()));
    }

    fn load_example(&mut self, ex: &examples::Example) {
        self.circuit = (ex.build)();
        self.name = ex.name.to_string();
        // Named substances the example uses become pickable on new nodes.
        for node in &self.circuit.nodes {
            let d = &node.out_substance;
            if !d.name.is_empty()
                && !self.declared.contains(d)
                && !circuit::SUBSTANCES
                    .iter()
                    .any(|(n, b, u)| *n == d.name && *b == d.base && *u == d.unit)
            {
                self.declared.push(d.clone());
            }
        }
        self.next_n = self.circuit.nodes.len() + 1;
        self.selected = None;
        self.pending_wire = None;
        self.running = false;
        self.status = format!("loaded \"{}\" — {} · press Run", ex.name, ex.blurb);
    }

    fn add_node(&mut self, kind: NodeKind, canvas_center: Pos2) {
        let i = self.circuit.nodes.len();
        let jitter = vec2(((i % 5) as f32 - 2.0) * 70.0, ((i / 5) as f32 - 1.0) * 80.0);
        self.circuit.nodes.push(Node::new(kind, self.next_n, canvas_center + jitter));
        self.next_n += 1;
        self.selected = Some(i);
    }

    fn delete_node(&mut self, i: usize) {
        self.circuit.nodes.remove(i);
        self.circuit.wires.retain(|w| w.from != i && w.to != i);
        for w in &mut self.circuit.wires {
            if w.from > i {
                w.from -= 1;
            }
            if w.to > i {
                w.to -= 1;
            }
        }
        self.selected = None;
        self.pending_wire = None;
    }

    /// First free path: name.ext, name-1.ext, name-2.ext, …
    fn unique_path(dir: &str, stem: &str, ext: &str) -> String {
        let candidate = format!("{dir}/{stem}.{ext}");
        if !std::path::Path::new(&candidate).exists() {
            return candidate;
        }
        (1..)
            .map(|n| format!("{dir}/{stem}-{n}.{ext}"))
            .find(|p| !std::path::Path::new(p).exists())
            .unwrap()
    }

    fn export_csv(&mut self) {
        let home = std::env::var("HOME").unwrap_or_default();
        let path = Self::unique_path(
            &format!("{home}/Desktop"),
            &format!("{}-data", self.name.replace(' ', "-")),
            "csv",
        );
        match std::fs::write(&path, self.circuit.csv()) {
            Ok(()) => {
                self.write_latest();
                self.status =
                    format!("wrote {} ticks to {path} (+ ~/.bert-compose/latest)", self.circuit.history.len())
            }
            Err(e) => self.status = format!("csv export failed: {e}"),
        }
    }

    /// The "latest run" contract: a fixed location Claude Code / any agent can
    /// read without being told a filename. Written on every Run-pause and
    /// export. So "analyze my latest run" always resolves.
    fn write_latest(&self) {
        let home = std::env::var("HOME").unwrap_or_default();
        let dir = format!("{home}/.bert-compose");
        if std::fs::create_dir_all(&dir).is_err() {
            return;
        }
        let _ = std::fs::write(format!("{dir}/latest.csv"), self.circuit.csv());
        let model = export::to_world_model(&self.circuit, &self.name);
        if let Ok(j) = serde_json::to_string_pretty(&model) {
            let _ = std::fs::write(format!("{dir}/latest.json"), j);
        }
        let _ = std::fs::write(format!("{dir}/latest.md"), self.run_summary());
    }

    /// A compact human/LLM-readable digest of the current run.
    fn run_summary(&self) -> String {
        let c = &self.circuit;
        let mut s = format!(
            "# {} — bert-compose run\n\n{} components, {} bonds, {} ticks, diversity {}.\n\n## Components\n",
            self.name,
            c.nodes.len(),
            c.wires.len(),
            c.history.len(),
            c.diversity(),
        );
        for node in &c.nodes {
            s.push_str(&format!(
                "- {} ({}): activity {:.2}{}\n",
                node.name,
                node.kind.label(),
                node.activity,
                if node.storage.abs() > 1e-6 {
                    format!(", stored {:.2}", node.storage)
                } else if node.total.abs() > 1e-6 {
                    format!(", total {:.2}", node.total)
                } else {
                    String::new()
                },
            ));
        }
        s.push_str("\n## Wiring\n");
        for w in &c.wires {
            s.push_str(&format!(
                "- {} → {} ({})\n",
                c.nodes[w.from].name,
                c.nodes[w.to].name,
                c.nodes[w.from].out_substance.label(),
            ));
        }
        if c.tick > 0 {
            let baseline: f32 = c.nodes.iter().map(|n| n.initial_storage).sum();
            s.push_str(&format!(
                "\n## Conservation\nemitted {:.2} + initial stocks {:.2} = stored {:.2} + sunk {:.2} + in-flight {:.2} + dissipated {:.2} (residual {:+.3})\n",
                c.emitted, baseline, c.stored(), c.sunk, c.in_flight(), c.dissipated, c.balance(),
            ));
        }
        let mm = c.substance_mismatches();
        if !mm.is_empty() {
            s.push_str("\n## Warnings\n");
            for (i, wants, got) in mm {
                s.push_str(&format!(
                    "- {} consumes {wants:?} but is fed {} (flow ignored)\n",
                    c.nodes[i].name,
                    got.label(),
                ));
            }
        }
        s.push_str("\nFull per-tick data: latest.csv. Model: latest.json.\n");
        s
    }

    fn save(&mut self) {
        let model = export::to_world_model(&self.circuit, &self.name);
        let home = std::env::var("HOME").unwrap_or_default();
        let path = Self::unique_path(
            &format!("{home}/Desktop"),
            &self.name.replace(' ', "-"),
            "json",
        );
        match serde_json::to_string_pretty(&model)
            .map_err(|e| e.to_string())
            .and_then(|s| std::fs::write(&path, s).map_err(|e| e.to_string()))
        {
            Ok(()) => self.status = format!("saved {path} — open it in BERT"),
            Err(e) => self.status = format!("save failed: {e}"),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Clock.
        if self.running {
            let now = ctx.input(|i| i.time);
            if now - self.last_tick_at >= 1.0 / self.ticks_per_sec as f64 {
                self.circuit.step();
                self.last_tick_at = now;
            }
            ctx.request_repaint();
        }
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.pending_wire = None;
        }

        // Collect hal's answer when it arrives.
        if let Some(rx) = &self.hal_rx {
            if let Ok(res) = rx.try_recv() {
                self.hal_busy = false;
                self.hal_rx = None;
                match res {
                    Ok(a) => self.hal_answer = Some(a),
                    Err(e) => self.hal_answer = Some(e),
                }
            } else {
                ctx.request_repaint_after(std::time::Duration::from_millis(150));
            }
        }

        top_bar(self, ctx);
        hal_window(self, ctx);
        status_bar(self, ctx);
        palette_panel(self, ctx);
        inspector_panel(self, ctx);
        charts_panel(self, ctx);
        canvas(self, ctx);
    }
}

fn top_bar(app: &mut App, ctx: &egui::Context) {
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
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.add(primary_button("Save as BERT model")).clicked() {
                        app.save();
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

/// hal's reading of the run — a floating, closable card.
fn hal_window(app: &mut App, ctx: &egui::Context) {
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

fn status_bar(app: &App, ctx: &egui::Context) {
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

fn palette_panel(app: &mut App, ctx: &egui::Context) {
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
                if ui
                    .add_sized([146.0, 24.0], egui::Button::new(RichText::new(kind.label()).size(12.0)))
                    .on_hover_text(hover)
                    .clicked()
                {
                    app.add_node(*kind, center);
                }
            }
        });
}

/// A label : value row for the teaching card.
fn learn_row(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.add_space(3.0);
    ui.label(RichText::new(label).color(SECONDARY).size(10.0));
    ui.label(RichText::new(value).color(PRIMARY).size(11.0));
}

fn inspector_panel(app: &mut App, ctx: &egui::Context) {
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
            let is_source = app.circuit.nodes[i].kind == NodeKind::Source;
            let node = &mut app.circuit.nodes[i];
            ui.add(egui::TextEdit::singleline(&mut node.name).desired_width(170.0));
            ui.add_space(4.0);
            let label = if is_source { "rate / tick" } else { "agency 0–1" };
            let max = if is_source { 10.0 } else { 1.0 };
            ui.add(egui::Slider::new(&mut node.param, 0.0..=max).text(label));
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
            }
            ui.add_space(4.0);
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
            if ui.button(RichText::new("✕ delete").color(RED).size(11.5)).clicked() {
                app.delete_node(i);
                return;
            }

            // Teaching card — plain English first, details on demand.
            ui.add_space(10.0);
            let kind = app.circuit.nodes[i].kind;
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

/// Live metrics panel — NetLogo/Mesa-style plots of the recorded run, one line
/// per node, drawn from circuit.history (column layout: [tick, a,s,t per node]).
fn charts_panel(app: &mut App, ctx: &egui::Context) {
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

fn canvas(app: &mut App, ctx: &egui::Context) {
    egui::CentralPanel::default()
        .frame(egui::Frame::new().fill(theme::CREAM))
        .show(ctx, |ui| {
            if app.circuit.nodes.is_empty() {
                theme::empty_state_inline(ui);
                return;
            }
            let painter = ui.painter();
            let time = ui.input(|i| i.time) as f32;

            // Wires first (under nodes).
            for (k, wire) in app.circuit.wires.iter().enumerate() {
                let a = app.circuit.nodes[wire.from].pos;
                let b = app.circuit.nodes[wire.to].pos;
                let substance = app.circuit.wire_substance(wire);
                let color = substance_color(substance);
                let dir = (b - a).normalized();
                let (a_edge, b_edge) = (a + dir * NODE_R, b - dir * (NODE_R + 4.0));
                let gradient = wire.mode == circuit::FlowMode::Gradient;
                // Gradient flows: thickness scales with the LIVE potential
                // difference — the wire visibly thins as the two stocks
                // equalize. You watch the field relax.
                let delta = (app.circuit.level(wire.from) - app.circuit.level(wire.to)).max(0.0);
                let width: f32 = if gradient { (0.8 + 0.5 * delta).min(5.0) } else { 1.6 };
                if gradient {
                    // dashed to read as a field, not a pushed pipe
                    let seg = b_edge - a_edge;
                    let steps = (seg.length() / 9.0).max(1.0) as i32;
                    for s in 0..steps {
                        if s % 2 == 0 {
                            let p0 = a_edge + seg * (s as f32 / steps as f32);
                            let p1 = a_edge + seg * ((s as f32 + 1.0) / steps as f32);
                            painter.line_segment([p0, p1], Stroke::new(width, color));
                        }
                    }
                } else {
                    painter.line_segment([a_edge, b_edge], Stroke::new(width, color));
                }
                // arrowhead
                let n = vec2(-dir.y, dir.x);
                painter.line_segment([b_edge, b_edge - dir * 7.0 + n * 4.0], Stroke::new(1.6, color));
                painter.line_segment([b_edge, b_edge - dir * 7.0 - n * 4.0], Stroke::new(1.6, color));
                // live amount + moving pulse
                let amount = if gradient {
                    wire.conductance * delta
                } else {
                    app.circuit.nodes[wire.from].activity
                };
                let mid = a_edge + (b_edge - a_edge) * 0.5;
                let unit = &app.circuit.nodes[wire.from].out_substance.unit;
                painter.text(
                    mid + vec2(0.0, -10.0),
                    egui::Align2::CENTER_CENTER,
                    format!(
                        "{amount:.1}{}{}",
                        if unit.is_empty() { String::new() } else { format!(" {unit}") },
                        if gradient { " ⤓" } else { "" }
                    ),
                    egui::FontId::monospace(9.5),
                    if amount > 0.005 { color } else { HAIRLINE },
                );
                if app.running && amount > 0.005 {
                    let t = (time * 0.7 + k as f32 * 0.37).fract();
                    let p = a_edge + (b_edge - a_edge) * t;
                    painter.circle_filled(p, 3.0, color);
                }
            }

            // Substance mismatches: nodes silently ignoring a flow they can't use.
            let mut mismatched: std::collections::HashSet<usize> =
                app.circuit.substance_mismatches().iter().map(|(i, _, _)| *i).collect();
            mismatched.extend(app.circuit.underpowered_amplifiers());

            // Nodes.
            let mut clicked_body: Option<usize> = None;
            let mut clicked_port: Option<usize> = None;
            for i in 0..app.circuit.nodes.len() {
                let pos = app.circuit.nodes[i].pos;
                let rect = egui::Rect::from_center_size(pos, vec2(NODE_R * 2.0, NODE_R * 2.0));
                let resp = ui.interact(rect, ui.id().with(("node", i)), Sense::click_and_drag());
                if resp.dragged() {
                    app.circuit.nodes[i].pos += resp.drag_delta();
                }
                if resp.clicked() {
                    clicked_body = Some(i);
                }
                // Out-port handle.
                let port = pos + vec2(NODE_R + 7.0, 0.0);
                let port_rect = egui::Rect::from_center_size(port, vec2(14.0, 14.0));
                let port_resp =
                    ui.interact(port_rect, ui.id().with(("port", i)), Sense::click());
                if port_resp.clicked() {
                    clicked_port = Some(i);
                }

                let node = &app.circuit.nodes[i];
                let painter = ui.painter();
                let (ring, ring_w) = if app.pending_wire == Some(i) {
                    (GOLD, 2.2)
                } else if app.selected == Some(i) {
                    (ACCENT, 2.0)
                } else if mismatched.contains(&i) {
                    (theme::AMBER, 2.0)
                } else {
                    (HAIRLINE, 1.2)
                };
                let fill = match node.kind {
                    NodeKind::Source => GREEN_SOFT,
                    NodeKind::Sink => ACCENT_SOFT,
                    _ => PAPER,
                };
                // The shape IS the semantics — vessel, funnel, valve, eye…
                glyph::draw(painter, node.kind, pos, fill, Stroke::new(ring_w, ring));
                // Buffer fill: the vessel visibly holds its stock.
                if matches!(
                    node.kind,
                    NodeKind::Process(bert_core::ProcessPrimitive::Buffering)
                ) {
                    let frac = (node.storage / node.initial_storage.max(10.0)).clamp(0.0, 1.0);
                    let h = glyph::R * 1.6;
                    let inner = egui::Rect::from_center_size(
                        pos + vec2(0.0, 2.0 + h * 0.5 * (1.0 - frac)),
                        vec2(glyph::R * 1.3, h * frac),
                    );
                    painter.rect_filled(inner, 2.0, GOLD.gamma_multiply(0.55));
                }
                painter.text(
                    pos + vec2(0.0, 8.0),
                    egui::Align2::CENTER_CENTER,
                    format!("{:.1}", node.activity),
                    egui::FontId::monospace(9.0),
                    PRIMARY,
                );
                painter.text(
                    pos + vec2(0.0, glyph::R + 11.0),
                    egui::Align2::CENTER_CENTER,
                    &node.name,
                    egui::FontId::proportional(10.0),
                    SECONDARY,
                );
                if mismatched.contains(&i) {
                    painter.text(
                        pos + vec2(0.0, -NODE_R - 8.0),
                        egui::Align2::CENTER_CENTER,
                        "⚠",
                        egui::FontId::proportional(14.0),
                        theme::AMBER,
                    );
                }
                // port
                let port_color = if port_resp.hovered() { GOLD } else { substance_color(node.out_substance.base) };
                painter.circle(port, 5.0, PAPER, Stroke::new(1.6, port_color));
            }

            // Wiring interaction: port starts, body completes. Boundary
            // discipline: a Sink only absorbs (nothing flows out of it) and a
            // Source only emits (flows can't run back into the environment
            // input) — both would break conservation.
            if let Some(i) = clicked_port {
                if matches!(app.circuit.nodes[i].kind, NodeKind::Sink) {
                    app.status = format!(
                        "{} is a sink — it only absorbs; nothing flows out of it",
                        app.circuit.nodes[i].name
                    );
                } else {
                    app.pending_wire = Some(i);
                    app.status = format!(
                        "wiring from {} — click a target component (esc cancels)",
                        app.circuit.nodes[i].name
                    );
                }
            } else if let Some(i) = clicked_body {
                if let Some(from) = app.pending_wire.take() {
                    if matches!(app.circuit.nodes[i].kind, NodeKind::Source) {
                        app.status = format!(
                            "{} is a source — it only emits; flows can't run back into it",
                            app.circuit.nodes[i].name
                        );
                    } else if from != i
                        && !app.circuit.wires.iter().any(|w| w.from == from && w.to == i)
                    {
                        app.circuit.wires.push(Wire::new(from, i));
                        app.status = format!(
                            "bond: {} → {} (internalized — both endpoints inside)",
                            app.circuit.nodes[from].name, app.circuit.nodes[i].name
                        );
                    }
                } else {
                    app.selected = Some(i);
                }
            }

            // Pending wire follows the pointer.
            if let Some(from) = app.pending_wire {
                if let Some(p) = ui.ctx().pointer_latest_pos() {
                    ui.painter().line_segment(
                        [app.circuit.nodes[from].pos, p],
                        Stroke::new(1.2, GOLD),
                    );
                }
                ui.ctx().request_repaint();
            }

            let _ = pill; // theme helpers kept for parity
        });
}
