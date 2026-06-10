//! Application state and persistence: the App struct, the frame loop, and
//! everything that reads or writes disk (save, export, the latest-run
//! contract, the run digest). UI panels live in `ui/*`.

use crate::circuit::{Circuit, DeclaredSubstance, Node, NodeKind, SUBSTANCES};
use crate::{askhal, examples, export, theme, ui};
use bert_core::SubstanceType;
use eframe::egui;
use egui::{vec2, Pos2};

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

    pub fn load_example(&mut self, ex: &examples::Example) {
        self.adopt_circuit(
            (ex.build)(),
            ex.name.to_string(),
            format!("loaded \"{}\" — {} · press Run", ex.name, ex.blurb),
        );
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
        match std::fs::write(&path, self.circuit.csv()) {
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
        let _ = std::fs::write(format!("{dir}/latest.csv"), self.circuit.csv());
        let model = export::to_world_model(&self.circuit, &self.name);
        if let Ok(j) = serde_json::to_string_pretty(&model) {
            let _ = std::fs::write(format!("{dir}/latest.json"), j);
        }
        let _ = std::fs::write(format!("{dir}/latest.md"), self.run_summary());
    }

    /// A compact human/LLM-readable digest of the current run.
    pub fn run_summary(&self) -> String {
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
        ui::hal_window::show(self, ctx);
        ui::status_bar::show(self, ctx);
        ui::palette::show(self, ctx);
        ui::inspector::show(self, ctx);
        ui::charts::show(self, ctx);
        ui::canvas::show(self, ctx);
    }
}
