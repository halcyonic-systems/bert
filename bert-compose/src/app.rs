//! Application state and persistence: the App struct, the frame loop, and
//! everything that reads or writes disk (save, export, the latest-run
//! contract, the run digest). UI panels live in `ui/*`.

use crate::circuit::{Circuit, DeclaredSubstance, Node, NodeKind, Wire, SUBSTANCES};
use crate::ladder::Rung;
use crate::{askhal, examples, export, theme, ui};
use bert_core::SubstanceType;
use eframe::egui;
use egui::{pos2, vec2, Pos2, Vec2};

pub struct App {
    pub circuit: Circuit,
    pub name: String,
    pub selected: Option<usize>,
    /// Wire in progress: source node index.
    pub pending_wire: Option<usize>,
    pub running: bool,
    pub ticks_per_sec: f32,
    pub last_tick_at: f64,
    pub next_n: usize,
    pub status: String,
    pub show_charts: bool,
    /// Which metric the plots track: 0 activity, 1 storage, 2 cumulative total.
    pub chart_metric: usize,
    /// Active presentation lens (index into `lens::LENSES`; 0 = Systems).
    pub lens: usize,
    /// The "what is this?" orientation window.
    pub show_about: bool,
    /// Canvas pan offset (drag empty space to move the whole diagram).
    pub pan: Vec2,
    /// Screen-space top-left of the canvas, updated each frame — so a stamped
    /// macro can be placed where it's actually visible.
    pub canvas_origin: Pos2,
    // — substance dictionary —
    /// Substances free-declared this session (pickable on any node).
    pub declared: Vec<DeclaredSubstance>,
    pub declaring: bool,
    pub decl_name: String,
    pub decl_unit: String,
    pub decl_base: SubstanceType,
    // — Ask hal (sovereign in-app analysis) —
    pub hal_model: String,
    pub hal_rx: Option<std::sync::mpsc::Receiver<Result<String, String>>>,
    pub hal_answer: Option<String>,
    pub hal_busy: bool,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
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
            lens: 0,
            show_about: true, // first thing a new user sees
            pan: Vec2::ZERO,
            canvas_origin: pos2(180.0, 90.0),
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

    pub fn ask_hal(&mut self) {
        self.write_latest(); // make sure the digest is current
        self.hal_busy = true;
        self.hal_answer = None;
        self.hal_rx = Some(askhal::ask(self.run_summary(), self.hal_model.clone()));
    }

    /// Named substances the circuit uses become pickable on new nodes.
    fn harvest_declared(&mut self) {
        for node in &self.circuit.nodes {
            let d = &node.out_substance;
            if !d.name.is_empty()
                && !self.declared.contains(d)
                && !SUBSTANCES.iter().any(|(n, b, u)| *n == d.name && *b == d.base && *u == d.unit)
            {
                self.declared.push(d.clone());
            }
        }
    }

    /// A circuit arrived from outside the canvas (example or loaded file) —
    /// reset the interaction state around it.
    fn adopt_circuit(&mut self, circuit: Circuit, name: String, status: String) {
        self.circuit = circuit;
        self.name = name;
        self.harvest_declared();
        self.next_n = self.circuit.nodes.len() + 1;
        self.selected = None;
        self.pending_wire = None;
        self.running = false;
        self.status = status;
    }

    /// Clear the canvas to an empty system — start fresh. (Reset, by
    /// contrast, only rewinds the simulation; this wipes the topology.)
    pub fn new_canvas(&mut self) {
        self.circuit = Circuit::default();
        self.name = "My System".to_string();
        self.selected = None;
        self.pending_wire = None;
        self.running = false;
        self.next_n = 1;
        self.pan = Vec2::ZERO;
        self.status = "cleared — start fresh".to_string();
    }

    pub fn load_example(&mut self, ex: &examples::Example) {
        self.adopt_circuit(
            (ex.build)(),
            ex.name.to_string(),
            format!("loaded \"{}\" — {} · press Run", ex.name, ex.blurb),
        );
        // Domain examples open in their domain's lens; Foundations stay neutral.
        self.lens = ex.lens;
    }

    /// Load a BERT model (ours or any compose-shaped one) back onto the
    /// canvas — the other half of the Save round-trip.
    pub fn load_model_file(&mut self, path: &std::path::Path) {
        let outcome = std::fs::read_to_string(path)
            .map_err(|e| e.to_string())
            .and_then(|s| {
                serde_json::from_str::<bert_core::WorldModel>(&s).map_err(|e| e.to_string())
            })
            .and_then(|m| export::from_world_model(&m).map(|c| (c, export::model_name(&m))));
        match outcome {
            Ok((circuit, name)) => {
                let n = circuit.nodes.len();
                let b = circuit.wires.len();
                self.adopt_circuit(
                    circuit,
                    name,
                    format!("loaded {} — {n} components, {b} bonds · press Run", path.display()),
                );
            }
            Err(e) => self.status = format!("load failed: {e}"),
        }
    }

    pub fn load_dialog(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("BERT model", &["json"])
            .set_title("Load a BERT model")
            .pick_file()
        {
            self.load_model_file(&path);
        }
    }

    /// Stamp a Troncale process onto the canvas: it drops the rung's PRIMITIVE
    /// circuit (visible, editable) — a macro, not a new atom. Appends below
    /// existing content so a model can be built by composing processes; on an
    /// empty canvas it lands at the rung's native layout. The status line
    /// keeps the honesty ("these are primitives; edit them freely").
    pub fn stamp_macro(&mut self, rung: &Rung) {
        let sub = (rung.build)();
        // Land the macro where it's VISIBLE: translate its top-left corner to
        // the current viewport's top-left (accounting for pan), with a small
        // cascade so repeated stamps don't perfectly overlap. (The old
        // "place below existing content" pushed stamps off the bottom.)
        let sub_min = egui::pos2(
            sub.nodes.iter().map(|n| n.pos.x).fold(f32::MAX, f32::min),
            sub.nodes.iter().map(|n| n.pos.y).fold(f32::MAX, f32::min),
        );
        let cascade = (self.circuit.nodes.len() as f32 / 6.0).floor() * 34.0 % 170.0;
        // world coords for a screen point near the canvas top-left
        let target = self.canvas_origin + vec2(70.0 + cascade, 70.0 + cascade) - self.pan;
        let offset = target - sub_min;
        let base = self.circuit.nodes.len();
        for mut node in sub.nodes {
            node.pos += offset;
            node.process = Some(rung.name); // provenance for the inspector
            self.circuit.nodes.push(node);
        }
        for w in sub.wires {
            self.circuit.wires.push(Wire {
                from: w.from + base,
                to: w.to + base,
                mode: w.mode,
                conductance: w.conductance,
            });
        }
        self.harvest_declared();
        self.next_n = self.circuit.nodes.len() + 1;
        // Select a representative node of the stamp (the first process brick)
        // so the inspector shows a card immediately — same feedback a primitive
        // click gives. Without this, stamping left the inspector empty.
        self.selected = (base..self.circuit.nodes.len())
            .find(|&j| matches!(self.circuit.nodes[j].kind, NodeKind::Process(_)));
        self.pending_wire = None;
        self.running = false;
        self.status = format!(
            "stamped {} — {}. These are primitives; edit them freely.",
            rung.name, rung.composition
        );
    }

    pub fn add_node(&mut self, kind: NodeKind, canvas_center: Pos2) {
        let i = self.circuit.nodes.len();
        let jitter = vec2(((i % 5) as f32 - 2.0) * 70.0, ((i / 5) as f32 - 1.0) * 80.0);
        self.circuit.nodes.push(Node::new(kind, self.next_n, canvas_center + jitter));
        self.next_n += 1;
        self.selected = Some(i);
    }

    pub fn delete_node(&mut self, i: usize) {
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

    /// A node's name as the active lens reads it — so a domain run's exported
    /// data says "Quorum gate / Treasury", not "Modulating 2 / Buffering 3".
    fn node_label(&self, i: usize) -> String {
        let node = &self.circuit.nodes[i];
        crate::lens::display_name(self.lens, node.kind, &node.name)
    }

    /// The recorded run as CSV, labeled by the active lens.
    fn labeled_csv(&self) -> String {
        self.circuit.csv_with(|i| self.node_label(i))
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

    pub fn export_csv(&mut self) {
        let home = std::env::var("HOME").unwrap_or_default();
        let path = Self::unique_path(
            &format!("{home}/Desktop"),
            &format!("{}-data", self.name.replace(' ', "-")),
            "csv",
        );
        match std::fs::write(&path, self.labeled_csv()) {
            Ok(()) => {
                self.write_latest();
                self.status = format!(
                    "wrote {} ticks to {path} (+ ~/.bert-compose/latest)",
                    self.circuit.history.len()
                )
            }
            Err(e) => self.status = format!("csv export failed: {e}"),
        }
    }

    /// The "latest run" contract: a fixed location Claude Code / any agent can
    /// read without being told a filename. Written on every Run-pause and
    /// export. So "analyze my latest run" always resolves.
    pub fn write_latest(&self) {
        let home = std::env::var("HOME").unwrap_or_default();
        let dir = format!("{home}/.bert-compose");
        if std::fs::create_dir_all(&dir).is_err() {
            return;
        }
        let _ = std::fs::write(format!("{dir}/latest.csv"), self.labeled_csv());
        let model = export::to_world_model(&self.circuit, &self.name);
        if let Ok(j) = serde_json::to_string_pretty(&model) {
            let _ = std::fs::write(format!("{dir}/latest.json"), j);
        }
        let _ = std::fs::write(format!("{dir}/latest.md"), self.run_summary());
    }

    /// A compact human/LLM-readable digest of the current run. Names follow
    /// the active lens, so a domain run reads in domain terms.
    pub fn run_summary(&self) -> String {
        let c = &self.circuit;
        let lens_note = if self.lens != 0 {
            format!(" · {} lens", crate::lens::LENSES[self.lens].name)
        } else {
            String::new()
        };
        let mut s = format!(
            "# {} — bert-compose run{}\n\n{} components, {} bonds, {} ticks, diversity {}.\n\n## Components\n",
            self.name,
            lens_note,
            c.nodes.len(),
            c.wires.len(),
            c.history.len(),
            c.diversity(),
        );
        for (i, node) in c.nodes.iter().enumerate() {
            s.push_str(&format!(
                "- {} ({}): activity {:.2}{}\n",
                self.node_label(i),
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
                self.node_label(w.from),
                self.node_label(w.to),
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

    pub fn save(&mut self) {
        let model = export::to_world_model(&self.circuit, &self.name);
        let home = std::env::var("HOME").unwrap_or_default();
        let path =
            Self::unique_path(&format!("{home}/Desktop"), &self.name.replace(' ', "-"), "json");
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
        // Delete / Backspace removes the selected node — unless a text field
        // (a name editor) has focus, where those keys edit text.
        if let Some(i) = self.selected {
            let del = ctx.input(|i| {
                i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::Backspace)
            });
            if del && !ctx.wants_keyboard_input() {
                self.delete_node(i);
            }
        }

        // A .json dragged onto the window loads as a model.
        let dropped: Vec<std::path::PathBuf> = ctx.input(|i| {
            i.raw
                .dropped_files
                .iter()
                .filter_map(|f| f.path.clone())
                .filter(|p| p.extension().is_some_and(|e| e == "json"))
                .collect()
        });
        if let Some(path) = dropped.first() {
            self.load_model_file(path);
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

        ui::top_bar::show(self, ctx);
        ui::about::show(self, ctx);
        ui::hal_window::show(self, ctx);
        ui::status_bar::show(self, ctx);
        ui::palette::show(self, ctx);
        ui::inspector::show(self, ctx);
        ui::charts::show(self, ctx);
        ui::canvas::show(self, ctx);
    }
}
