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

mod circuit;
mod export;
mod theme;

use bert_core::SubstanceType;
use circuit::{Circuit, Node, NodeKind, Wire, PALETTE};
use eframe::egui;
use egui::{pos2, vec2, Color32, Pos2, RichText, Sense, Stroke};
use theme::{
    dot, pill, primary_button, section_header, semibold, ACCENT, ACCENT_SOFT, GOLD, GREEN,
    GREEN_SOFT, HAIRLINE, PAPER, PRIMARY, RED, SECONDARY,
};

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
        }
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
                self.status =
                    format!("wrote {} ticks to {path}", self.circuit.history.len())
            }
            Err(e) => self.status = format!("csv export failed: {e}"),
        }
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

        top_bar(self, ctx);
        status_bar(self, ctx);
        palette_panel(self, ctx);
        inspector_panel(self, ctx);
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
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.add(primary_button("Save as BERT model")).clicked() {
                        app.save();
                    }
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

fn inspector_panel(app: &mut App, ctx: &egui::Context) {
    egui::SidePanel::right("inspector")
        .resizable(false)
        .exact_width(200.0)
        .frame(egui::Frame::new().fill(theme::CREAM).inner_margin(egui::Margin::same(10)))
        .show(ctx, |ui| {
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
            ui.horizontal(|ui| {
                ui.label(RichText::new("emits").color(SECONDARY).small());
                for s in
                    [SubstanceType::Energy, SubstanceType::Material, SubstanceType::Message]
                {
                    let sel = node.out_substance == s;
                    if ui
                        .selectable_label(sel, RichText::new(format!("{s:?}")).size(10.5))
                        .clicked()
                    {
                        node.out_substance = s;
                    }
                }
            });
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

            // Wires touching this node, removable.
            ui.add_space(10.0);
            section_header(ui, "BONDS");
            ui.add_space(2.0);
            let mut remove: Option<usize> = None;
            for (k, w) in app.circuit.wires.iter().enumerate() {
                if w.from != i && w.to != i {
                    continue;
                }
                ui.horizontal(|ui| {
                    if ui.small_button("✕").clicked() {
                        remove = Some(k);
                    }
                    ui.label(
                        RichText::new(format!(
                            "{} → {}",
                            app.circuit.nodes[w.from].name, app.circuit.nodes[w.to].name
                        ))
                        .color(PRIMARY)
                        .size(11.0),
                    );
                });
            }
            if let Some(k) = remove {
                app.circuit.wires.remove(k);
            }
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
                painter.line_segment([a_edge, b_edge], Stroke::new(1.6, color));
                // arrowhead
                let n = vec2(-dir.y, dir.x);
                painter.line_segment([b_edge, b_edge - dir * 7.0 + n * 4.0], Stroke::new(1.6, color));
                painter.line_segment([b_edge, b_edge - dir * 7.0 - n * 4.0], Stroke::new(1.6, color));
                // live amount + moving pulse
                let amount = {
                    let sender = &app.circuit.nodes[wire.from];
                    sender.activity
                };
                let mid = a_edge + (b_edge - a_edge) * 0.5;
                painter.text(
                    mid + vec2(0.0, -10.0),
                    egui::Align2::CENTER_CENTER,
                    format!("{amount:.1}"),
                    egui::FontId::monospace(9.5),
                    if amount > 0.005 { color } else { HAIRLINE },
                );
                if app.running && amount > 0.005 {
                    let t = (time * 0.7 + k as f32 * 0.37).fract();
                    let p = a_edge + (b_edge - a_edge) * t;
                    painter.circle_filled(p, 3.0, color);
                }
            }

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
                } else {
                    (HAIRLINE, 1.2)
                };
                let fill = match node.kind {
                    NodeKind::Source => GREEN_SOFT,
                    NodeKind::Sink => ACCENT_SOFT,
                    _ => PAPER,
                };
                painter.circle(pos, NODE_R, fill, Stroke::new(ring_w, ring));
                // Buffer fill bar: state made visible.
                if matches!(
                    node.kind,
                    NodeKind::Process(bert_core::ProcessPrimitive::Buffering)
                ) {
                    let frac = (node.storage / 10.0).clamp(0.0, 1.0);
                    let bar = egui::Rect::from_min_size(
                        pos + vec2(-16.0, 12.0 - 24.0 * frac),
                        vec2(5.0, 24.0 * frac),
                    );
                    painter.rect_filled(bar, 2.0, GOLD);
                    painter.rect_stroke(
                        egui::Rect::from_min_size(pos + vec2(-16.0, -12.0), vec2(5.0, 24.0)),
                        2.0,
                        Stroke::new(1.0, HAIRLINE),
                        egui::StrokeKind::Inside,
                    );
                }
                let short: String = node.kind.label().chars().take(7).collect();
                painter.text(
                    pos + vec2(2.0, -3.0),
                    egui::Align2::CENTER_CENTER,
                    short,
                    egui::FontId::new(9.5, semibold()),
                    PRIMARY,
                );
                painter.text(
                    pos + vec2(2.0, 8.0),
                    egui::Align2::CENTER_CENTER,
                    format!("{:.1}", node.activity),
                    egui::FontId::monospace(9.0),
                    SECONDARY,
                );
                painter.text(
                    pos + vec2(0.0, NODE_R + 10.0),
                    egui::Align2::CENTER_CENTER,
                    &node.name,
                    egui::FontId::proportional(10.0),
                    SECONDARY,
                );
                // port
                let port_color = if port_resp.hovered() { GOLD } else { substance_color(node.out_substance) };
                painter.circle(port, 5.0, PAPER, Stroke::new(1.6, port_color));
            }

            // Wiring interaction: port starts, body completes.
            if let Some(i) = clicked_port {
                app.pending_wire = Some(i);
                app.status = format!(
                    "wiring from {} — click a target component (esc cancels)",
                    app.circuit.nodes[i].name
                );
            } else if let Some(i) = clicked_body {
                if let Some(from) = app.pending_wire.take() {
                    if from != i && !app.circuit.wires.contains(&Wire { from, to: i }) {
                        app.circuit.wires.push(Wire { from, to: i });
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
