//! The live circuit: process primitives wired into a flowing, stateful system.
//!
//! Transfer functions are ports of BERT's verified implementations
//! (python/agents.py PRIMITIVE_T, 39 tests) — Mobus's atomic work processes,
//! substance-aware: Energy/Material conserve, Message copies; Amplifying draws
//! its gain from a metered Energy input (gain never manufactures mass);
//! Sensing crosses substance (physical in → signal out); Buffering is a
//! conservative stock — the system's state/memory lives there.
//!
//! Update rule: synchronous discrete time. Each tick reads the previous
//! tick's wire amounts and writes the next — so feedback loops are ordinary
//! dynamics, no special cases. (Divergence from the Python, noted: the
//! buffer's release is a knob here rather than demand-tracking — same
//! conservative stock, simpler to touch.)
//!
//! # Conservation ledger
//!
//! Physical mass (Energy/Material) is fully accounted every tick:
//!
//! ```text
//! emitted + initial stocks == stored + sunk + in-flight + dissipated
//! ```
//!
//! `dissipated` is not a fudge factor — it is computed per node as
//! `physical in − physical out − Δstorage`, so the equation holds by
//! construction and any *unintended* leak shows up as a nonzero residual
//! (`balance()`), which the property tests assert over random circuits.
//! The intended dissipation channels, each a deliberate modeling decision:
//!
//! - **Propelling/Impeding friction** — the `(1 − agency)` share is lost in
//!   transport, the classic transport cost.
//! - **Amplifying power draw** — the amp consumes its entire metered Energy
//!   feed (signal out + heat); output is Message, which is never ledgered.
//! - **Modulating shed** — flow blocked by a throttled gate is shed at the
//!   valve (this push model has no backpressure).
//! - **Sensing consumption** — a pushed feed into a sensor is consumed by
//!   measurement. Observation taps (Buffer → Sensing) read the level and
//!   consume nothing.
//! - **Substance-mismatch shed** — flow a node can't use vanishes; surfaced
//!   by the amber ⚠ and counted here.
//! - **Dead ends** — activity with no pushed outwire evaporates next tick;
//!   surfaced by `dead_ends()` and counted.
//!
//! Message is information: copied, gated, manufactured (Inverting) — never
//! conserved, never in the ledger.

use bert_core::{ProcessPrimitive, SubstanceType};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NodeKind {
    /// Environment input: emits `rate` per tick.
    Source,
    /// Environment output: accumulates what arrives.
    Sink,
    Process(ProcessPrimitive),
}

impl NodeKind {
    pub fn label(&self) -> String {
        match self {
            NodeKind::Source => "Source".into(),
            NodeKind::Sink => "Sink".into(),
            NodeKind::Process(p) => format!("{p:?}"),
        }
    }

    /// The tunable scalar coefficient of this work process, for the inspector:
    /// `(label, max)`. `None` means the primitive has no scalar knob — its
    /// behavior is structural (Buffering's knobs are stock + release; a
    /// Splitter just divides; a valve is driven by its control wire).
    ///
    /// NB: these are PROCESS PARAMETERS — a gain, an efficiency, a rate — NOT
    /// "agency". In Mobus, agency is a property of *agents* (Reactive/
    /// Anticipatory/Intentional), not of atomic work processes. The earlier
    /// "agency 0–1" label was a category error; a Sensing process has a gain,
    /// not agency.
    pub fn param_spec(&self) -> Option<(&'static str, f32)> {
        use ProcessPrimitive::*;
        match self {
            NodeKind::Source => Some(("rate / tick", 10.0)),
            NodeKind::Process(Sensing) => Some(("sensor gain  k", 1.0)),
            NodeKind::Process(Amplifying) => Some(("gain  (→ ×1–10)", 1.0)),
            NodeKind::Process(Propelling) => Some(("efficiency  η", 1.0)),
            NodeKind::Process(Impeding) => Some(("throughput  (1 − impedance)", 1.0)),
            _ => None,
        }
    }

    /// Default output substance — signal-class primitives emit Message.
    pub fn default_out(&self) -> SubstanceType {
        match self {
            NodeKind::Process(
                ProcessPrimitive::Sensing
                | ProcessPrimitive::Inverting
                | ProcessPrimitive::Copying
                | ProcessPrimitive::Amplifying,
            ) => SubstanceType::Message,
            _ => SubstanceType::Material,
        }
    }

    /// Can this primitive turn an incoming flow of `s` into output? Feeding it
    /// a substance it can't use is a silent no-op (Copying ignores Material,
    /// Amplifying ignores Material) — the UI surfaces the mismatch so no flow
    /// vanishes without explanation.
    pub fn consumes(&self, s: SubstanceType) -> bool {
        use ProcessPrimitive::*;
        let physical = s != SubstanceType::Message;
        match self {
            // Message-only signal processors: copying/inverting matter would
            // counterfeit it.
            NodeKind::Process(Copying | Inverting) => s == SubstanceType::Message,
            // Sensing reads physical flow (Energy/Material), crosses to Message.
            NodeKind::Process(Sensing) => physical,
            // Amplifying needs a Message signal and Energy power — Material is
            // dead weight to it.
            NodeKind::Process(Amplifying) => s != SubstanceType::Material,
            // You SPLIT matter (divide a conserved quantity) and COMBINE matter;
            // information isn't divided, it's copied — so these are physical-only.
            NodeKind::Process(Splitting | Combining) => physical,
            // Buffering (stock + optional Message gate), Modulating (physical
            // primary + Message control), Propelling/Impeding (move anything),
            // Source/Sink: take what they're given.
            _ => true,
        }
    }
}

/// A declared substance: a human name ("money", "water", "votes") that
/// factors through one of the three conserved kinds. Neutrality is the
/// trichotomy (Energy/Material/Message); reality is a refinement that maps
/// onto it — the dynamics only ever read `base`, so money conserves exactly
/// like Material and votes copy exactly like Message. The name and unit ride
/// along into the BERT JSON (`Substance.sub_type`, `Interaction.unit`).
#[derive(Clone, PartialEq, Debug)]
pub struct DeclaredSubstance {
    /// Plain name ("money"); empty = the bare base kind.
    pub name: String,
    /// The conserved kind whose physics this substance inherits.
    pub base: SubstanceType,
    /// Display unit ("$", "L", "votes"); empty = unitless.
    pub unit: String,
}

impl DeclaredSubstance {
    pub fn bare(base: SubstanceType) -> Self {
        Self { name: String::new(), base, unit: String::new() }
    }
    pub fn named(name: &str, base: SubstanceType, unit: &str) -> Self {
        Self { name: name.to_string(), base, unit: unit.to_string() }
    }
    /// "money (Material)" — or just "Material" when unnamed.
    pub fn label(&self) -> String {
        if self.name.is_empty() {
            format!("{:?}", self.base)
        } else {
            format!("{} ({:?})", self.name, self.base)
        }
    }
}

impl From<SubstanceType> for DeclaredSubstance {
    fn from(base: SubstanceType) -> Self {
        Self::bare(base)
    }
}

/// The curated substance palette — relatable names first (this tool is for
/// social scientists and systems theorists, not just engineers). Each maps
/// to the conserved kind whose physics it inherits; anything not here can be
/// free-declared in the inspector.
pub const SUBSTANCES: &[(&str, SubstanceType, &str)] = &[
    ("money", SubstanceType::Material, "$"),
    ("water", SubstanceType::Material, "L"),
    ("people", SubstanceType::Material, "people"),
    ("food", SubstanceType::Material, "kg"),
    ("goods", SubstanceType::Material, "units"),
    ("sunlight", SubstanceType::Energy, "W"),
    ("electricity", SubstanceType::Energy, "kWh"),
    ("fuel", SubstanceType::Energy, "J"),
    ("effort", SubstanceType::Energy, "hours"),
    ("votes", SubstanceType::Message, "votes"),
    ("news", SubstanceType::Message, "stories"),
    ("data", SubstanceType::Message, "bits"),
    ("orders", SubstanceType::Message, "orders"),
];

pub const PALETTE: &[NodeKind] = &[
    NodeKind::Source,
    NodeKind::Sink,
    NodeKind::Process(ProcessPrimitive::Buffering),
    NodeKind::Process(ProcessPrimitive::Combining),
    NodeKind::Process(ProcessPrimitive::Splitting),
    NodeKind::Process(ProcessPrimitive::Amplifying),
    NodeKind::Process(ProcessPrimitive::Modulating),
    NodeKind::Process(ProcessPrimitive::Sensing),
    NodeKind::Process(ProcessPrimitive::Inverting),
    NodeKind::Process(ProcessPrimitive::Copying),
    NodeKind::Process(ProcessPrimitive::Propelling),
    NodeKind::Process(ProcessPrimitive::Impeding),
];

pub struct Node {
    pub kind: NodeKind,
    pub name: String,
    pub pos: egui::Pos2,
    /// Output substance (wires created from this node inherit it). A
    /// declared name + unit over a conserved base kind; dynamics read
    /// `.base` only.
    pub out_substance: DeclaredSubstance,
    /// Source rate / agency capacity (gain, efficiency, k…) depending on kind.
    pub param: f32,
    /// Buffer release rate per tick.
    pub release_rate: f32,
    /// Buffer starting stock — the "this system HAS a quantity" assertion.
    /// Exported as AgentModel.initial_state{"storage"} (what Mesa seeds).
    pub initial_storage: f32,
    /// Provenance: the Troncale process this node was stamped from (a
    /// `ladder::Rung` name), or `None` if hand-placed. Pure UI hint — lets the
    /// inspector show "this is part of a Feedback process" alongside the
    /// node's own primitive card. Not serialized.
    pub process: Option<&'static str>,
    // — live state —
    pub storage: f32,
    pub activity: f32,
    pub total: f32,
}

impl Node {
    pub fn new(kind: NodeKind, n: usize, pos: egui::Pos2) -> Self {
        Self {
            kind,
            name: format!("{} {}", kind.label(), n),
            pos,
            out_substance: kind.default_out().into(),
            param: if kind == NodeKind::Source { 1.0 } else { 0.5 },
            release_rate: 1.0,
            initial_storage: 0.0,
            process: None,
            storage: 0.0,
            activity: 0.0,
            total: 0.0,
        }
    }
}

/// How a flow's rate is set. Pushed = a node emits at its own rate (the
/// default). Gradient = the flow is a *generalized flow* down a potential
/// difference (Mobus Ch.4: forces/fields/diffusion are flows with a gradient
/// rate-law) — `rate = conductance · (level_from − level_to)`. Gradient is how
/// Potential Fields enter bert-compose: a field is a flow MODE, not a node.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum FlowMode {
    #[default]
    Pushed,
    Gradient,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Wire {
    pub from: usize,
    pub to: usize,
    pub mode: FlowMode,
    /// Gradient-mode conductance (k in rate = k·Δlevel). Ignored when pushed.
    pub conductance: f32,
}

impl Wire {
    pub fn new(from: usize, to: usize) -> Self {
        Self { from, to, mode: FlowMode::Pushed, conductance: 0.3 }
    }
    pub fn gradient(from: usize, to: usize, conductance: f32) -> Self {
        Self { from, to, mode: FlowMode::Gradient, conductance }
    }
}

#[derive(Default)]
pub struct Circuit {
    pub nodes: Vec<Node>,
    pub wires: Vec<Wire>,
    pub tick: u64,
    /// Per-tick data rows: [tick, n0.activity, n0.storage, n0.total, n1…].
    /// Cleared on Reset or when the topology changes mid-recording.
    pub history: Vec<Vec<f32>>,
    // — conservation ledger (physical mass only; see module docs) —
    /// Cumulative physical mass delivered out of Sources.
    pub emitted: f32,
    /// Cumulative physical mass absorbed by Sinks.
    pub sunk: f32,
    /// Cumulative physical mass shed through the intended channels
    /// (friction, valve shed, amp power, sensing, mismatches, dead ends).
    pub dissipated: f32,
}

impl Circuit {
    pub fn reset(&mut self) {
        for n in &mut self.nodes {
            n.storage = n.initial_storage;
            n.activity = 0.0;
            n.total = 0.0;
        }
        self.tick = 0;
        self.history.clear();
        self.emitted = 0.0;
        self.sunk = 0.0;
        self.dissipated = 0.0;
    }

    /// Σ stock across all nodes.
    pub fn stored(&self) -> f32 {
        self.nodes.iter().map(|n| n.storage).sum()
    }

    /// Physical mass in transit: activity of process nodes that emit a
    /// conserved substance — emitted last tick, delivered next.
    pub fn in_flight(&self) -> f32 {
        self.nodes
            .iter()
            .filter(|n| {
                matches!(n.kind, NodeKind::Process(_))
                    && n.out_substance.base != SubstanceType::Message
            })
            .map(|n| n.activity)
            .sum()
    }

    /// Conservation residual. ≈0 (float noise) means every unit of physical
    /// mass is accounted: emissions plus starting stocks equal what's stored,
    /// sunk, in flight, or dissipated through declared channels. Anything
    /// else is a leak — a bug by definition. (Editing a stock mid-run moves
    /// the baseline; Reset re-baselines.)
    pub fn balance(&self) -> f32 {
        let baseline: f32 = self.nodes.iter().map(|n| n.initial_storage).sum();
        self.emitted + baseline - (self.stored() + self.sunk + self.in_flight() + self.dissipated)
    }

    /// Conserved kind carried by a wire = the base of the sender's declared
    /// output substance (what the dynamics run on).
    pub fn wire_substance(&self, w: &Wire) -> SubstanceType {
        self.nodes[w.from].out_substance.base
    }

    /// One synchronous step: all transfer functions read the previous tick's
    /// wire amounts (sender activity at t-1) and write activity for t.
    /// A node's "level" (potential) for gradient-flow rate laws (Mobus Ch.4):
    /// a buffer's stock, a source's fixed potential (its rate), a sink's ground
    /// (0), else a node's current activity.
    pub fn level(&self, i: usize) -> f32 {
        match self.nodes[i].kind {
            NodeKind::Source => self.nodes[i].param,
            NodeKind::Sink => 0.0,
            NodeKind::Process(ProcessPrimitive::Buffering) => self.nodes[i].storage,
            _ => self.nodes[i].activity,
        }
    }

    /// A node has a potential — something a gradient flow can fall from —
    /// only if it's a Source (fixed potential) or a Buffering stock. A
    /// gradient wire from anything else would read mass off a transient
    /// activity that nothing drains (creating matter), so those wires are
    /// inert; `inert_gradient_wires()` surfaces them.
    pub fn has_potential(&self, i: usize) -> bool {
        matches!(
            self.nodes[i].kind,
            NodeKind::Source | NodeKind::Process(ProcessPrimitive::Buffering)
        )
    }

    pub fn step(&mut self) {
        let n = self.nodes.len();
        let nw = self.wires.len();

        // Ledger deltas for this tick, committed at the end.
        let mut emitted_now = 0.0f32;
        let mut sunk_now = 0.0f32;
        let mut dissipated_now = 0.0f32;

        // Dead ends: an activity with no pushed outwire is read by nothing —
        // it evaporates this tick. Count it so the ledger stays exact.
        // (Buffers never dangle: release is 0 without a pushed outlet.)
        for i in 0..n {
            if matches!(self.nodes[i].kind, NodeKind::Process(_))
                && self.nodes[i].out_substance.base != SubstanceType::Message
                && !self.wires.iter().any(|w| w.from == i && w.mode == FlowMode::Pushed)
            {
                dissipated_now += self.nodes[i].activity;
            }
        }

        // ── Gradient flows (Potential Fields): rate = conductance·(Δlevel),
        // forward-only, read from pre-tick levels (synchronous). Capped so a
        // buffer source can't drain below zero in one tick. ──
        let mut grad: Vec<f32> = self
            .wires
            .iter()
            .map(|w| {
                if w.mode == FlowMode::Gradient && self.has_potential(w.from) {
                    (w.conductance * (self.level(w.from) - self.level(w.to))).max(0.0)
                } else {
                    0.0
                }
            })
            .collect();
        for i in 0..n {
            if !matches!(self.nodes[i].kind, NodeKind::Process(ProcessPrimitive::Buffering)) {
                continue; // only buffers can over-drain; sources are fixed potentials
            }
            let idxs: Vec<usize> = (0..nw)
                .filter(|&k| self.wires[k].from == i && self.wires[k].mode == FlowMode::Gradient)
                .collect();
            let total: f32 = idxs.iter().map(|&k| grad[k]).sum();
            if total > self.nodes[i].storage && total > 0.0 {
                let scale = self.nodes[i].storage / total;
                for k in idxs {
                    grad[k] *= scale;
                }
            }
        }
        // Gradient outflow leaving each node (drains buffers).
        let gradient_out: Vec<f32> = (0..n)
            .map(|i| (0..nw).filter(|&k| self.wires[k].from == i).map(|k| grad[k]).sum())
            .collect();

        // Previous-tick amount arriving over each PUSHED wire, split when the
        // sender fans out a conservative substance (Splitting/Copying handle
        // their own fanout semantics; everything else divides physical flow).
        // A PUSHED wire Buffer → Sensing is an observation tap: the sensor
        // reads the stock's LEVEL without draining it ("sensing is very low
        // power"). A gradient wire into a sensor is a real drain, not a tap.
        let is_observation = |w: &Wire| -> bool {
            w.mode == FlowMode::Pushed
                && matches!(self.nodes[w.from].kind, NodeKind::Process(ProcessPrimitive::Buffering))
                && matches!(self.nodes[w.to].kind, NodeKind::Process(ProcessPrimitive::Sensing))
        };
        // amount delivered over wire index k (gradient or pushed).
        let amount_on = |k: usize| -> f32 {
            let w = &self.wires[k];
            if w.mode == FlowMode::Gradient {
                return grad[k];
            }
            if is_observation(w) {
                return self.nodes[w.from].storage; // non-draining level read
            }
            let sender = &self.nodes[w.from];
            if matches!(sender.kind, NodeKind::Sink) {
                return 0.0; // a sink is terminal — absorbed mass never re-emits
            }
            // Pushed fanout splits the sender's activity across pushed,
            // non-observation outwires only (gradient/observation excluded).
            let outs = (0..nw)
                .filter(|&x| {
                    self.wires[x].from == w.from
                        && self.wires[x].mode == FlowMode::Pushed
                        && !is_observation(&self.wires[x])
                })
                .count()
                .max(1) as f32;
            // Message replicates to every receiver (information copies);
            // Energy/Material split across the fanout (matter doesn't) —
            // which is also why Copying relabeled to a physical substance
            // splits rather than duplicating.
            if sender.out_substance.base == SubstanceType::Message {
                sender.activity
            } else {
                sender.activity / outs
            }
        };

        // Emissions: physical mass actually delivered out of Sources this
        // tick, over pushed and gradient wires alike.
        for k in 0..nw {
            let w = &self.wires[k];
            if matches!(self.nodes[w.from].kind, NodeKind::Source)
                && self.wire_substance(w) != SubstanceType::Message
            {
                emitted_now += amount_on(k);
            }
        }

        let mut next_activity = vec![0.0f32; n];
        let mut next_storage: Vec<f32> = self.nodes.iter().map(|x| x.storage).collect();
        let mut sink_add = vec![0.0f32; n];

        for (i, node) in self.nodes.iter().enumerate() {
            let incoming: Vec<(SubstanceType, f32, bool)> = (0..nw)
                .filter(|&k| self.wires[k].to == i)
                .map(|k| {
                    (
                        self.wire_substance(&self.wires[k]),
                        amount_on(k),
                        is_observation(&self.wires[k]),
                    )
                })
                .collect();
            // What the transfer function sees (observation level-reads count:
            // a sensor reads the stock).
            let physical: f32 = incoming
                .iter()
                .filter(|(s, _, _)| *s != SubstanceType::Message)
                .map(|(_, a, _)| a)
                .sum();
            // What was actually DELIVERED — observation reads excluded; this
            // is the mass the ledger holds the node accountable for.
            let delivered_phys: f32 = incoming
                .iter()
                .filter(|(s, _, obs)| *s != SubstanceType::Message && !obs)
                .map(|(_, a, _)| a)
                .sum();
            let message: f32 = incoming
                .iter()
                .filter(|(s, _, _)| *s == SubstanceType::Message)
                .map(|(_, a, _)| a)
                .sum();
            let a = node.param; // agency capacity 0..1

            next_activity[i] = match node.kind {
                NodeKind::Source => node.param, // emits its rate
                NodeKind::Sink => {
                    sink_add[i] = physical + message;
                    physical + message
                }
                NodeKind::Process(p) => match p {
                    // storage += inflow (incl. gradient in); −release (pushed
                    // out) −gradient_out (field-driven out). The gradient drain
                    // already left via its wires; subtract it from the stock.
                    ProcessPrimitive::Buffering => {
                        let mut storage = next_storage[i] + physical - gradient_out[i];
                        let gate = if self.wires.iter().any(|w| {
                            w.to == i
                                && w.mode == FlowMode::Pushed
                                && self.wire_substance(w) == SubstanceType::Message
                        }) {
                            message.clamp(0.0, 1.0)
                        } else {
                            1.0
                        };
                        // Release is the PUSHED outflow. It can only leave through
                        // a pushed, mass-carrying outwire — you can't pour out of a
                        // tank with no spout. Without one, the release would drain
                        // the stock into nowhere (mass destroyed). Gradient outwires
                        // are NOT spouts for release; they carry gradient_out above.
                        let has_pushed_outlet = (0..nw).any(|k| {
                            self.wires[k].from == i
                                && self.wires[k].mode == FlowMode::Pushed
                                && !is_observation(&self.wires[k])
                        });
                        let released = if has_pushed_outlet {
                            (node.release_rate * gate).min(storage.max(0.0))
                        } else {
                            0.0
                        };
                        storage -= released;
                        next_storage[i] = storage;
                        released
                    }
                    ProcessPrimitive::Combining => physical,
                    ProcessPrimitive::Splitting => physical, // fanout divides on wires
                    ProcessPrimitive::Propelling => (physical + message) * a,
                    ProcessPrimitive::Impeding => (physical + message) * a,
                    // signal · gain, bounded by metered Energy — no free mass
                    ProcessPrimitive::Amplifying => {
                        let power: f32 = incoming
                            .iter()
                            .filter(|(s, _, _)| *s == SubstanceType::Energy)
                            .map(|(_, x, _)| x)
                            .sum();
                        let gain = 1.0 + 9.0 * a;
                        (message * gain).min(power)
                    }
                    // physical → signal (crosses substance, never drains)
                    ProcessPrimitive::Sensing => physical * a,
                    // primary gated by control in [0,1]; with no control wire
                    // the valve sits OPEN (gate 1) — same convention as the
                    // buffer's gate. A closed-by-default valve silently
                    // destroyed every physical inflow.
                    ProcessPrimitive::Modulating => {
                        let has_control = self.wires.iter().any(|w| {
                            w.to == i
                                && w.mode == FlowMode::Pushed
                                && self.wire_substance(w) == SubstanceType::Message
                        });
                        let gate = if has_control { message.clamp(0.0, 1.0) } else { 1.0 };
                        physical * gate
                    }
                    ProcessPrimitive::Inverting => (1.0 - message).max(0.0),
                    ProcessPrimitive::Copying => message,
                },
            };

            // The ledger rule (one rule, every arm): whatever physical mass a
            // node was delivered and neither re-emits, passes down a gradient,
            // nor stores, it dissipated. Exact by construction — see module
            // docs for why each channel is intended.
            match node.kind {
                // Inflow to a source has nowhere to go (the UI refuses these
                // wires; ledgered defensively).
                NodeKind::Source => dissipated_now += delivered_phys,
                NodeKind::Sink => sunk_now += delivered_phys,
                NodeKind::Process(_) => {
                    let out_phys = if node.out_substance.base == SubstanceType::Message {
                        0.0
                    } else {
                        next_activity[i]
                    };
                    dissipated_now += delivered_phys
                        - out_phys
                        - gradient_out[i]
                        - (next_storage[i] - node.storage);
                }
            }
        }

        for (i, node) in self.nodes.iter_mut().enumerate() {
            node.activity = next_activity[i];
            node.storage = next_storage[i];
            node.total += sink_add[i];
        }
        self.emitted += emitted_now;
        self.sunk += sunk_now;
        self.dissipated += dissipated_now;
        self.tick += 1;

        // Record the tick. A topology change invalidates prior columns.
        let width = 1 + self.nodes.len() * 3;
        if self.history.last().map(|r| r.len()) != Some(width) {
            self.history.clear();
        }
        let mut row = Vec::with_capacity(width);
        row.push(self.tick as f32);
        for node in &self.nodes {
            row.push(node.activity);
            row.push(node.storage);
            row.push(node.total);
        }
        self.history.push(row);
    }

    /// The recorded run as CSV with raw node names. (The app exports via
    /// `csv_with` to carry lens names; this is the raw form used by tests and
    /// the sweep emitter.)
    #[allow(dead_code)]
    pub fn csv(&self) -> String {
        self.csv_with(|i| self.nodes[i].name.clone())
    }

    /// The recorded run as CSV: tick, then activity/storage/total per node.
    /// `label(i)` names column-group `i` — the app passes the lens reading so
    /// a domain run exports as "Quorum gate", "Treasury", not "Modulating 2".
    pub fn csv_with(&self, label: impl Fn(usize) -> String) -> String {
        let mut out = String::from("tick");
        for i in 0..self.nodes.len() {
            let name = label(i).replace(',', " ");
            out.push_str(&format!(",{name}.activity,{name}.storage,{name}.total"));
        }
        out.push('\n');
        for row in &self.history {
            let cells: Vec<String> = row.iter().map(|v| format!("{v}")).collect();
            out.push_str(&cells.join(","));
            out.push('\n');
        }
        out
    }

    /// Nodes wired to receive a substance they can't consume — the flow is
    /// silently ignored. Returns (node index, what it wants, what it's fed —
    /// as declared, so warnings can say "fed money (Material)").
    pub fn substance_mismatches(&self) -> Vec<(usize, SubstanceType, DeclaredSubstance)> {
        let mut out = Vec::new();
        for (i, node) in self.nodes.iter().enumerate() {
            for w in self.wires.iter().filter(|w| w.to == i) {
                let got = self.nodes[w.from].out_substance.clone();
                if !node.kind.consumes(got.base) {
                    // Report what it wants: the first substance it does consume.
                    let wants = [
                        SubstanceType::Message,
                        SubstanceType::Energy,
                        SubstanceType::Material,
                    ]
                    .into_iter()
                    .find(|s| node.kind.consumes(*s))
                    .unwrap_or(SubstanceType::Message);
                    out.push((i, wants, got));
                    break;
                }
            }
        }
        out
    }

    /// Amplifying with a signal but no Energy power: output is bounded to 0.
    /// A second, softer advisory (the node IS wired right, just underpowered).
    pub fn underpowered_amplifiers(&self) -> Vec<usize> {
        self.nodes
            .iter()
            .enumerate()
            .filter(|(i, n)| {
                matches!(n.kind, NodeKind::Process(ProcessPrimitive::Amplifying))
                    && self.wires.iter().any(|w| {
                        w.to == *i && self.wire_substance(w) == SubstanceType::Message
                    })
                    && !self.wires.iter().any(|w| {
                        w.to == *i && self.wire_substance(w) == SubstanceType::Energy
                    })
            })
            .map(|(i, _)| i)
            .collect()
    }

    /// Gradient wires drawn from a node with no potential (not a Source or a
    /// stock): a field needs a level to fall from, so these carry nothing.
    /// Surfaced so the wire doesn't read as mysteriously dead.
    pub fn inert_gradient_wires(&self) -> Vec<usize> {
        (0..self.wires.len())
            .filter(|&k| {
                self.wires[k].mode == FlowMode::Gradient && !self.has_potential(self.wires[k].from)
            })
            .collect()
    }

    /// Process nodes that receive flow but send it nowhere (no pushed
    /// outwire): their output evaporates each tick (ledgered as dissipated).
    /// Usually the model wants a Sink there. Buffers are exempt — a terminal
    /// stock legitimately accumulates.
    pub fn dead_ends(&self) -> Vec<usize> {
        self.nodes
            .iter()
            .enumerate()
            .filter(|(i, n)| {
                matches!(
                    n.kind,
                    NodeKind::Process(p) if p != ProcessPrimitive::Buffering
                ) && self.wires.iter().any(|w| w.to == *i)
                    && !self.wires.iter().any(|w| w.from == *i && w.mode == FlowMode::Pushed)
            })
            .map(|(i, _)| i)
            .collect()
    }

    /// SameKind (Systems/Core/Complexity.lean): two components are the same
    /// kind iff they act on exactly the same things and exactly the same
    /// things act on them. Returns the number of equivalence classes —
    /// component-kind diversity, derived from wiring alone.
    pub fn diversity(&self) -> usize {
        let profile = |i: usize| {
            let mut outs: Vec<usize> =
                self.wires.iter().filter(|w| w.from == i).map(|w| w.to).collect();
            let mut ins: Vec<usize> =
                self.wires.iter().filter(|w| w.to == i).map(|w| w.from).collect();
            outs.sort_unstable();
            ins.sort_unstable();
            (outs, ins)
        };
        let mut classes: Vec<(Vec<usize>, Vec<usize>)> = Vec::new();
        for i in 0..self.nodes.len() {
            let p = profile(i);
            if !classes.contains(&p) {
                classes.push(p);
            }
        }
        classes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::pos2;

    fn node(kind: NodeKind) -> Node {
        Node::new(kind, 0, pos2(0.0, 0.0))
    }

    /// Source → Buffer → Sink: the stock fills faster than it drains, state
    /// accumulates, and mass is conserved end to end.
    #[test]
    fn buffer_stores_state_and_conserves() {
        let mut c = Circuit::default();
        c.nodes.push(node(NodeKind::Source)); // rate 1.0
        c.nodes.push(node(NodeKind::Process(ProcessPrimitive::Buffering)));
        c.nodes.push(node(NodeKind::Sink));
        c.nodes[0].param = 2.0; // inflow 2/tick
        c.nodes[1].release_rate = 1.0; // outflow 1/tick
        c.wires.push(Wire::new(0, 1));
        c.wires.push(Wire::new(1, 2));
        for _ in 0..10 {
            c.step();
        }
        assert!(c.nodes[1].storage > 5.0, "stock accumulates: {}", c.nodes[1].storage);
        // Conservation: everything emitted is in the stock, in transit, or in the sink.
        let emitted = 2.0 * (c.tick as f32 - 1.0); // first tick's emission lands at t=2
        let accounted = c.nodes[1].storage + c.nodes[1].activity + c.nodes[2].total;
        assert!(
            (emitted - accounted).abs() <= 2.0 + f32::EPSILON,
            "mass conserved: emitted {emitted}, accounted {accounted}"
        );
    }

    /// Sensing + Inverting + Modulating around a Buffer = a homeostat: the
    /// control loop throttles inflow as the sensed level rises. The loop must
    /// regulate (bounded storage), not run away.
    #[test]
    fn negative_feedback_regulates() {
        let mut c = Circuit::default();
        c.nodes.push(node(NodeKind::Source)); // 0: supply
        c.nodes.push(node(NodeKind::Process(ProcessPrimitive::Modulating))); // 1: valve
        c.nodes.push(node(NodeKind::Process(ProcessPrimitive::Buffering))); // 2: stock
        c.nodes.push(node(NodeKind::Process(ProcessPrimitive::Sensing))); // 3: sensor
        c.nodes.push(node(NodeKind::Process(ProcessPrimitive::Inverting))); // 4: controller
        c.nodes.push(node(NodeKind::Sink)); // 5
        c.nodes[0].param = 3.0;
        c.nodes[2].release_rate = 1.0;
        c.nodes[3].param = 0.2; // sensor gain k
        c.wires.push(Wire::new(0, 1)); // supply → valve
        c.wires.push(Wire::new(1, 2)); // valve → stock
        c.wires.push(Wire::new(2, 3)); // stock outflow sensed
        c.wires.push(Wire::new(3, 4)); // sensor → inverter
        c.wires.push(Wire::new(4, 1)); // control closes the loop (gate)
        c.wires.push(Wire::new(2, 5)); // stock → sink
        let mut peak: f32 = 0.0;
        for _ in 0..200 {
            c.step();
            peak = peak.max(c.nodes[2].storage);
        }
        assert!(
            c.nodes[2].storage < 100.0,
            "feedback keeps the stock bounded, got {}",
            c.nodes[2].storage
        );
        assert!(c.nodes[5].total > 0.0, "flow still reaches the sink");
    }

    /// Amplifying cannot manufacture mass: output is capped by metered power.
    #[test]
    fn amplifier_bounded_by_power() {
        let mut c = Circuit::default();
        c.nodes.push(node(NodeKind::Source)); // 0: signal source
        c.nodes.push(node(NodeKind::Source)); // 1: power source
        c.nodes.push(node(NodeKind::Process(ProcessPrimitive::Amplifying))); // 2
        c.nodes.push(node(NodeKind::Sink)); // 3
        c.nodes[0].param = 1.0;
        c.nodes[0].out_substance = SubstanceType::Message.into();
        c.nodes[1].param = 2.5;
        c.nodes[1].out_substance = SubstanceType::Energy.into();
        c.nodes[2].param = 1.0; // gain 10
        c.wires.push(Wire::new(0, 2));
        c.wires.push(Wire::new(1, 2));
        c.wires.push(Wire::new(2, 3));
        for _ in 0..5 {
            c.step();
        }
        // desired = 1.0 * 10 = 10, but only 2.5 energy available
        assert!((c.nodes[2].activity - 2.5).abs() < f32::EPSILON);
    }

    #[test]
    fn copying_material_is_flagged_not_swallowed() {
        let mut c = Circuit::default();
        c.nodes.push(node(NodeKind::Source)); // emits Material by default
        c.nodes.push(node(NodeKind::Process(ProcessPrimitive::Copying)));
        c.wires.push(Wire::new(0, 1));
        let m = c.substance_mismatches();
        assert_eq!(m.len(), 1);
        assert_eq!((m[0].0, m[0].1, m[0].2.base), (1, SubstanceType::Message, SubstanceType::Material));
        // Amplifying fed Material is also flagged now (was silently zeroing).
        let mut amp = Circuit::default();
        amp.nodes.push(node(NodeKind::Source)); // Material
        amp.nodes.push(node(NodeKind::Process(ProcessPrimitive::Amplifying)));
        amp.wires.push(Wire::new(0, 1));
        assert_eq!(amp.substance_mismatches().len(), 1, "Material -> Amplifying flagged");
        // Splitting fed Message is flagged (you split matter, you copy info).
        let mut sp = Circuit::default();
        sp.nodes.push(node(NodeKind::Process(ProcessPrimitive::Copying)));
        sp.nodes[0].out_substance = SubstanceType::Message.into();
        sp.nodes.push(node(NodeKind::Process(ProcessPrimitive::Splitting)));
        sp.wires.push(Wire::new(0, 1));
        assert_eq!(sp.substance_mismatches().len(), 1, "Message -> Splitting flagged");
        // Setting the source to emit Message clears it.
        c.nodes[0].out_substance = SubstanceType::Message.into();
        assert!(c.substance_mismatches().is_empty());
    }

    #[test]
    fn csv_records_every_tick() {
        let mut c = Circuit::default();
        c.nodes.push(node(NodeKind::Source));
        c.nodes.push(node(NodeKind::Sink));
        c.nodes[0].param = 2.0;
        c.wires.push(Wire::new(0, 1));
        for _ in 0..3 {
            c.step();
        }
        let csv = c.csv();
        let lines: Vec<&str> = csv.lines().collect();
        assert_eq!(lines.len(), 4, "header + 3 ticks");
        assert!(lines[0].starts_with("tick,"));
        assert!(lines[0].contains(".activity"));
        c.reset();
        assert!(c.history.is_empty(), "reset clears the recording");
    }

    /// Gradient flow = Potential Fields. A full buffer wired by a gradient flow
    /// to an empty buffer equalizes (a battery discharging / two tanks), and
    /// total stock is conserved — no controller needed (passive homeostasis).
    #[test]
    fn gradient_flow_equalizes_and_conserves() {
        let mut c = Circuit::default();
        c.nodes.push(node(NodeKind::Process(ProcessPrimitive::Buffering))); // 0 full
        c.nodes.push(node(NodeKind::Process(ProcessPrimitive::Buffering))); // 1 empty
        c.nodes[0].initial_storage = 10.0;
        c.nodes[0].storage = 10.0;
        c.nodes[0].release_rate = 0.0; // no pushed release — gradient only
        c.nodes[1].release_rate = 0.0;
        c.wires.push(Wire::gradient(0, 1, 0.3));
        let total0 = c.nodes[0].storage + c.nodes[1].storage;
        for _ in 0..200 {
            c.step();
        }
        let (a, b) = (c.nodes[0].storage, c.nodes[1].storage);
        assert!((a - b).abs() < 0.1, "two tanks equalize: {a} vs {b}");
        assert!((a + b - total0).abs() < 1e-3, "stock conserved: {} vs {total0}", a + b);
    }

    /// A buffer with release but NO pushed outlet must not destroy mass —
    /// you can't pour out of a tank with no spout (a gradient outwire is a
    /// field, not a spout). Regression for the conservation bug Shingai found
    /// by raising release on a gradient-only buffer mid-run.
    #[test]
    fn release_without_pushed_outlet_conserves() {
        let mut c = Circuit::default();
        c.nodes.push(node(NodeKind::Process(ProcessPrimitive::Buffering))); // 0
        c.nodes.push(node(NodeKind::Process(ProcessPrimitive::Buffering))); // 1
        c.nodes[0].initial_storage = 20.0;
        c.nodes[0].storage = 20.0;
        c.nodes[0].release_rate = 1.4; // cranked, but its only outwire is gradient
        c.nodes[1].release_rate = 0.0;
        c.wires.push(Wire::gradient(0, 1, 0.25));
        let total0 = 20.0;
        for _ in 0..100 {
            c.step();
        }
        let total = c.nodes[0].storage + c.nodes[1].storage;
        assert!((total - total0).abs() < 1e-3, "mass conserved despite release: {total}");
        assert!((c.nodes[0].storage - c.nodes[1].storage).abs() < 0.1, "still equalizes");
    }

    /// A source at fixed potential charges a buffer toward that potential
    /// (a capacitor charging), gradient shrinking as it fills.
    #[test]
    fn gradient_charges_toward_source_potential() {
        let mut c = Circuit::default();
        c.nodes.push(node(NodeKind::Source)); // fixed potential = param
        c.nodes.push(node(NodeKind::Process(ProcessPrimitive::Buffering)));
        c.nodes[0].param = 5.0;
        c.nodes[1].release_rate = 0.0;
        c.wires.push(Wire::gradient(0, 1, 0.3));
        for _ in 0..300 {
            c.step();
        }
        assert!((c.nodes[1].storage - 5.0).abs() < 0.2, "charges to source potential");
    }

    // ── Conservation invariant: the whole bug class at once ──────────────
    //
    // The class is "an outflow computed but not delivered, or an inflow
    // accepted but not stored" (splitter, copy, amplifier, buffer-release —
    // each found one at a time). The systematic catch: random circuits +
    // the ledger equation asserted every tick.

    /// xorshift64 — deterministic, dependency-free.
    struct Rng(u64);
    impl Rng {
        fn next(&mut self) -> u64 {
            let mut x = self.0;
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            self.0 = x;
            x
        }
        fn f(&mut self) -> f32 {
            (self.next() % 1000) as f32 / 1000.0
        }
        fn pick(&mut self, n: usize) -> usize {
            (self.next() % n as u64) as usize
        }
    }

    fn assert_balanced(c: &Circuit, ctx: &str) {
        let scale = (c.emitted + c.nodes.iter().map(|n| n.initial_storage).sum::<f32>()).max(1.0);
        assert!(
            c.balance().abs() <= 1e-3 * scale,
            "{ctx}: tick {} leaks {} (emitted {}, stored {}, sunk {}, in-flight {}, dissipated {})",
            c.tick,
            c.balance(),
            c.emitted,
            c.stored(),
            c.sunk,
            c.in_flight(),
            c.dissipated,
        );
    }

    /// Random circuit over the CONSERVATIVE node set (everything Material;
    /// no signal processors, no friction). `guarantee_outlets` retries until
    /// every non-sink node has an outwire (no dead ends).
    fn random_conservative(seed: u64, guarantee_outlets: bool) -> Circuit {
        use ProcessPrimitive::*;
        let mut r = Rng(seed | 1);
        let mut c = Circuit::default();
        for _ in 0..1 + r.pick(2) {
            let mut nd = node(NodeKind::Source);
            nd.param = 0.5 + 2.5 * r.f();
            c.nodes.push(nd);
        }
        let kinds = [Buffering, Splitting, Combining, Modulating];
        for _ in 0..2 + r.pick(5) {
            let k = kinds[r.pick(kinds.len())];
            let mut nd = node(NodeKind::Process(k));
            if k == Buffering {
                nd.release_rate = 2.0 * r.f();
                nd.initial_storage = 10.0 * r.f();
                nd.storage = nd.initial_storage;
            }
            c.nodes.push(nd);
        }
        for _ in 0..1 + r.pick(2) {
            c.nodes.push(node(NodeKind::Sink));
        }
        // Wires obey what the UI enforces: none into a Source, none out of a
        // Sink. Cycles, fanouts, and gradient wires (from potentials) allowed.
        let total = c.nodes.len();
        let targets: Vec<usize> =
            (0..total).filter(|&i| !matches!(c.nodes[i].kind, NodeKind::Source)).collect();
        for i in 0..total {
            if matches!(c.nodes[i].kind, NodeKind::Sink) {
                continue;
            }
            for attempt in 0..1 + r.pick(2) {
                let t = targets[r.pick(targets.len())];
                if t == i {
                    if guarantee_outlets && attempt == 0 {
                        let t2 = *targets.iter().find(|&&x| x != i).unwrap();
                        c.wires.push(Wire::new(i, t2));
                    }
                    continue; // self-wires forbidden (sometimes leaves a dead end)
                }
                if r.f() < 0.25 && c.has_potential(i) {
                    c.wires.push(Wire::gradient(i, t, 0.1 + 0.4 * r.f()));
                    if guarantee_outlets
                        && !c.wires.iter().any(|w| w.from == i && w.mode == FlowMode::Pushed)
                    {
                        c.wires.push(Wire::new(i, t));
                    }
                } else {
                    c.wires.push(Wire::new(i, t));
                }
            }
        }
        c
    }

    /// Property: over the conservative set with every node given an outlet,
    /// the ledger balances every tick AND nothing dissipates — there is no
    /// intended loss channel in this set, so any dissipation is a leak.
    #[test]
    fn conservation_property_strict() {
        for seed in 1..=300u64 {
            let mut c = random_conservative(seed.wrapping_mul(0x9E3779B97F4A7C15), true);
            for _ in 0..50 {
                c.step();
                assert_balanced(&c, &format!("strict seed {seed}"));
                assert!(
                    c.dissipated.abs() <= 1e-3 * c.emitted.max(1.0),
                    "strict seed {seed}: conservative circuit dissipated {} at tick {}",
                    c.dissipated,
                    c.tick
                );
            }
        }
    }

    /// Property: same set but dead ends allowed — mass may dissipate (it
    /// evaporates at the dangling node) but the ledger must still be exact.
    #[test]
    fn conservation_property_with_dead_ends() {
        for seed in 1..=300u64 {
            let mut c = random_conservative(seed.wrapping_mul(0xD1B54A32D192ED03), false);
            for _ in 0..50 {
                c.step();
                assert_balanced(&c, &format!("dead-end seed {seed}"));
            }
        }
    }

    /// Property: the FULL palette — friction, sensors, amps, valves, signal
    /// sources, observation taps. Dissipation is expected; the ledger must
    /// still account every unit (any delivery double-count or undercount
    /// breaks the equation).
    #[test]
    fn conservation_property_full_palette() {
        for seed in 1..=300u64 {
            let mut r = Rng(seed.wrapping_mul(0xA0761D6478BD642F) | 1);
            let mut c = Circuit::default();
            for _ in 0..1 + r.pick(2) {
                let mut nd = node(NodeKind::Source);
                nd.param = 0.5 + 2.5 * r.f();
                if r.f() < 0.3 {
                    nd.out_substance = SubstanceType::Message.into();
                } else if r.f() < 0.3 {
                    nd.out_substance = SubstanceType::Energy.into();
                }
                c.nodes.push(nd);
            }
            for _ in 0..2 + r.pick(6) {
                let k = PALETTE[2 + r.pick(PALETTE.len() - 2)]; // any primitive
                let mut nd = node(k);
                if k == NodeKind::Process(ProcessPrimitive::Buffering) {
                    nd.release_rate = 2.0 * r.f();
                    nd.initial_storage = 10.0 * r.f();
                    nd.storage = nd.initial_storage;
                }
                c.nodes.push(nd);
            }
            for _ in 0..1 + r.pick(2) {
                c.nodes.push(node(NodeKind::Sink));
            }
            let total = c.nodes.len();
            let targets: Vec<usize> =
                (0..total).filter(|&i| !matches!(c.nodes[i].kind, NodeKind::Source)).collect();
            for i in 0..total {
                if matches!(c.nodes[i].kind, NodeKind::Sink) {
                    continue;
                }
                for _ in 0..1 + r.pick(2) {
                    let t = targets[r.pick(targets.len())];
                    if t == i {
                        continue;
                    }
                    // Gradient wires from ANY node — anything a user can do,
                    // the engine must keep balanced (non-potentials → inert).
                    if r.f() < 0.2 {
                        c.wires.push(Wire::gradient(i, t, 0.1 + 0.4 * r.f()));
                    } else {
                        c.wires.push(Wire::new(i, t));
                    }
                }
            }
            for _ in 0..60 {
                c.step();
                assert_balanced(&c, &format!("full-palette seed {seed}"));
            }
        }
    }

    // ── Targeted probes: the checkpoint's known suspects ─────────────────

    /// A sink is terminal: wiring onward from it must re-emit nothing.
    #[test]
    fn sink_never_reemits() {
        let mut c = Circuit::default();
        c.nodes.push(node(NodeKind::Source));
        c.nodes.push(node(NodeKind::Sink));
        c.nodes.push(node(NodeKind::Process(ProcessPrimitive::Buffering)));
        c.nodes[0].param = 2.0;
        c.wires.push(Wire::new(0, 1));
        c.wires.push(Wire::new(1, 2)); // illegal in UI; engine must not duplicate
        for _ in 0..20 {
            c.step();
            assert_balanced(&c, "sink re-emission");
        }
        assert!(c.nodes[2].storage.abs() < f32::EPSILON, "sink re-emitted into the buffer");
    }

    /// Flow wired into a Source (UI refuses; engine defends): the mass is
    /// shed to the ledger, not silently destroyed.
    #[test]
    fn inflow_to_source_is_ledgered() {
        let mut c = Circuit::default();
        c.nodes.push(node(NodeKind::Source));
        c.nodes.push(node(NodeKind::Source));
        c.nodes[0].param = 2.0;
        c.wires.push(Wire::new(0, 1));
        for _ in 0..10 {
            c.step();
            assert_balanced(&c, "inflow to source");
        }
        assert!(c.dissipated > 0.0, "shed inflow must appear in the ledger");
    }

    /// A gradient wire from a node with no potential is inert — before this
    /// fix it minted mass off the sender's activity without draining anything.
    #[test]
    fn gradient_from_process_node_is_inert() {
        let mut c = Circuit::default();
        c.nodes.push(node(NodeKind::Source));
        c.nodes.push(node(NodeKind::Process(ProcessPrimitive::Splitting)));
        c.nodes.push(node(NodeKind::Process(ProcessPrimitive::Buffering)));
        c.nodes.push(node(NodeKind::Sink)); // legit outlet for the splitter
        c.nodes[0].param = 3.0;
        c.nodes[2].release_rate = 0.0;
        c.wires.push(Wire::new(0, 1));
        c.wires.push(Wire::gradient(1, 2, 0.5)); // field from a non-potential
        c.wires.push(Wire::new(1, 3));
        assert_eq!(c.inert_gradient_wires(), vec![1], "advisory lists the inert wire");
        for _ in 0..30 {
            c.step();
            assert_balanced(&c, "inert gradient");
        }
        assert!(
            c.nodes[2].storage.abs() < f32::EPSILON,
            "gradient from a non-potential minted {} mass",
            c.nodes[2].storage
        );
    }

    /// A valve with no control wire sits open (was: closed by default,
    /// destroying every inflow). With control it sheds — to the ledger.
    #[test]
    fn valve_open_without_control_sheds_with() {
        let mut open = Circuit::default();
        open.nodes.push(node(NodeKind::Source));
        open.nodes.push(node(NodeKind::Process(ProcessPrimitive::Modulating)));
        open.nodes.push(node(NodeKind::Sink));
        open.nodes[0].param = 2.0;
        open.wires.push(Wire::new(0, 1));
        open.wires.push(Wire::new(1, 2));
        for _ in 0..20 {
            open.step();
            assert_balanced(&open, "open valve");
        }
        assert!(open.nodes[2].total > 30.0, "uncontrolled valve passes flow through");
        assert!(open.dissipated.abs() < 1e-3, "open valve sheds nothing");

        let mut gated = Circuit::default();
        gated.nodes.push(node(NodeKind::Source)); // 0 supply
        gated.nodes.push(node(NodeKind::Source)); // 1 control = 0.5
        gated.nodes.push(node(NodeKind::Process(ProcessPrimitive::Modulating)));
        gated.nodes.push(node(NodeKind::Sink));
        gated.nodes[0].param = 2.0;
        gated.nodes[1].param = 0.5;
        gated.nodes[1].out_substance = SubstanceType::Message.into();
        gated.wires.push(Wire::new(0, 2));
        gated.wires.push(Wire::new(1, 2));
        gated.wires.push(Wire::new(2, 3));
        for _ in 0..20 {
            gated.step();
            assert_balanced(&gated, "gated valve");
        }
        assert!(gated.dissipated > 0.0, "the blocked half is ledgered, not lost");
    }

    /// Output wired to nowhere evaporates — but onto the ledger, with the
    /// dead end surfaced as an advisory.
    #[test]
    fn dead_end_is_ledgered_and_surfaced() {
        let mut c = Circuit::default();
        c.nodes.push(node(NodeKind::Source));
        c.nodes.push(node(NodeKind::Process(ProcessPrimitive::Combining)));
        c.nodes[0].param = 2.0;
        c.wires.push(Wire::new(0, 1)); // combiner output goes nowhere
        assert_eq!(c.dead_ends(), vec![1]);
        for _ in 0..20 {
            c.step();
            assert_balanced(&c, "dead end");
        }
        assert!(c.dissipated > 0.0, "evaporated output appears in the ledger");
    }

    /// Friction (Propelling at η<1) and amplifier power draw are the model's
    /// intended dissipations — decided + documented in the module docs —
    /// and both are tracked.
    #[test]
    fn friction_and_amp_power_are_ledgered() {
        let mut c = Circuit::default();
        c.nodes.push(node(NodeKind::Source));
        c.nodes.push(node(NodeKind::Process(ProcessPrimitive::Propelling)));
        c.nodes.push(node(NodeKind::Sink));
        c.nodes[0].param = 2.0;
        c.nodes[1].param = 0.5; // η = 0.5: half arrives, half is friction
        c.wires.push(Wire::new(0, 1));
        c.wires.push(Wire::new(1, 2));
        for _ in 0..20 {
            c.step();
            assert_balanced(&c, "friction");
        }
        assert!(c.dissipated > 0.0 && (c.sunk - c.dissipated).abs() < 1.1, "η=0.5 splits evenly");

        let mut amp = Circuit::default();
        amp.nodes.push(node(NodeKind::Source)); // 0 signal
        amp.nodes.push(node(NodeKind::Source)); // 1 power
        amp.nodes.push(node(NodeKind::Process(ProcessPrimitive::Amplifying)));
        amp.nodes.push(node(NodeKind::Sink));
        amp.nodes[0].param = 1.0;
        amp.nodes[0].out_substance = SubstanceType::Message.into();
        amp.nodes[1].param = 2.5;
        amp.nodes[1].out_substance = SubstanceType::Energy.into();
        amp.wires.push(Wire::new(0, 2));
        amp.wires.push(Wire::new(1, 2));
        amp.wires.push(Wire::new(2, 3));
        for _ in 0..20 {
            amp.step();
            assert_balanced(&amp, "amp power");
        }
        assert!(amp.dissipated > 0.0, "the metered power draw is ledgered");
    }

    /// Matter doesn't copy: a Copying node relabeled to a physical substance
    /// splits across its fanout instead of duplicating to every receiver.
    #[test]
    fn matter_does_not_copy() {
        let mut c = Circuit::default();
        c.nodes.push(node(NodeKind::Source));
        c.nodes.push(node(NodeKind::Process(ProcessPrimitive::Copying)));
        c.nodes.push(node(NodeKind::Sink));
        c.nodes.push(node(NodeKind::Sink));
        c.nodes[0].param = 1.0;
        c.nodes[0].out_substance = SubstanceType::Message.into();
        c.nodes[1].out_substance = SubstanceType::Material.into(); // user relabel
        c.wires.push(Wire::new(0, 1));
        c.wires.push(Wire::new(1, 2));
        c.wires.push(Wire::new(1, 3));
        for _ in 0..10 {
            c.step();
        }
        assert!(
            (c.nodes[2].total - c.nodes[3].total).abs() < f32::EPSILON
                && c.nodes[2].total < c.tick as f32 * 0.51,
            "relabeled copy must split, not duplicate: {} + {}",
            c.nodes[2].total,
            c.nodes[3].total
        );
    }

    /// Mixed pushed + gradient outflow from one buffer — the checkpoint's
    /// remaining suspect — conserves exactly.
    #[test]
    fn mixed_pushed_and_gradient_buffer_conserves() {
        let mut c = Circuit::default();
        c.nodes.push(node(NodeKind::Source));
        c.nodes.push(node(NodeKind::Process(ProcessPrimitive::Buffering))); // 1
        c.nodes.push(node(NodeKind::Process(ProcessPrimitive::Buffering))); // 2 gradient target
        c.nodes.push(node(NodeKind::Sink)); // 3 pushed target
        c.nodes[0].param = 1.5;
        c.nodes[1].initial_storage = 12.0;
        c.nodes[1].storage = 12.0;
        c.nodes[1].release_rate = 0.8;
        c.nodes[2].release_rate = 0.0;
        c.wires.push(Wire::new(0, 1));
        c.wires.push(Wire::gradient(1, 2, 0.3));
        c.wires.push(Wire::new(1, 3));
        for _ in 0..100 {
            c.step();
            assert_balanced(&c, "mixed buffer");
        }
        assert!(c.dissipated.abs() < 1e-2, "nothing dissipates in this circuit");
    }

    /// A named substance inherits its base physics exactly: money splits and
    /// conserves like Material; votes copy like Message; a mismatch warning
    /// carries the human name.
    #[test]
    fn named_substance_inherits_base_physics() {
        let mut c = Circuit::default();
        c.nodes.push(node(NodeKind::Source));
        c.nodes.push(node(NodeKind::Process(ProcessPrimitive::Splitting)));
        c.nodes.push(node(NodeKind::Sink));
        c.nodes.push(node(NodeKind::Sink));
        let money = DeclaredSubstance::named("money", SubstanceType::Material, "$");
        c.nodes[0].out_substance = money.clone();
        c.nodes[1].out_substance = money;
        c.nodes[0].param = 4.0;
        c.wires.push(Wire::new(0, 1));
        c.wires.push(Wire::new(1, 2));
        c.wires.push(Wire::new(1, 3));
        for _ in 0..20 {
            c.step();
            assert_balanced(&c, "money splits");
        }
        assert!((c.nodes[2].total - c.nodes[3].total).abs() < 1e-3, "equal shares");
        assert!(c.dissipated.abs() < 1e-3, "money is conserved");

        let mut v = Circuit::default();
        v.nodes.push(node(NodeKind::Source));
        v.nodes.push(node(NodeKind::Process(ProcessPrimitive::Splitting)));
        v.nodes[0].out_substance =
            DeclaredSubstance::named("votes", SubstanceType::Message, "votes");
        v.wires.push(Wire::new(0, 1));
        let m = v.substance_mismatches();
        assert_eq!(m.len(), 1, "votes (Message) into a Splitter is flagged");
        assert_eq!(m[0].2.name, "votes", "the warning speaks the human name");
    }

    #[test]
    fn diversity_from_wiring_alone() {
        let mut c = Circuit::default();
        c.nodes.push(node(NodeKind::Source)); // 0
        c.nodes.push(node(NodeKind::Process(ProcessPrimitive::Buffering))); // 1
        c.nodes.push(node(NodeKind::Process(ProcessPrimitive::Buffering))); // 2
        c.nodes.push(node(NodeKind::Sink)); // 3
        // both buffers fed by 0, both feed 3 → SameKind (identical profiles)
        for (f, t) in [(0, 1), (0, 2), (1, 3), (2, 3)] {
            c.wires.push(Wire::new(f, t));
        }
        assert_eq!(c.diversity(), 3, "source, {{buffer,buffer}}, sink");
    }
}
