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
    /// Output substance type (wires created from this node inherit it).
    pub out_substance: SubstanceType,
    /// Source rate / agency capacity (gain, efficiency, k…) depending on kind.
    pub param: f32,
    /// Buffer release rate per tick.
    pub release_rate: f32,
    /// Buffer starting stock — the "this system HAS a quantity" assertion.
    /// Exported as AgentModel.initial_state{"storage"} (what Mesa seeds).
    pub initial_storage: f32,
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
            out_substance: kind.default_out(),
            param: if kind == NodeKind::Source { 1.0 } else { 0.5 },
            release_rate: 1.0,
            initial_storage: 0.0,
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
    }

    /// Substance carried by a wire = the sender's output substance.
    pub fn wire_substance(&self, w: &Wire) -> SubstanceType {
        self.nodes[w.from].out_substance
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

    pub fn step(&mut self) {
        let n = self.nodes.len();
        let nw = self.wires.len();

        // ── Gradient flows (Potential Fields): rate = conductance·(Δlevel),
        // forward-only, read from pre-tick levels (synchronous). Capped so a
        // buffer source can't drain below zero in one tick. ──
        let mut grad: Vec<f32> = self
            .wires
            .iter()
            .map(|w| {
                if w.mode == FlowMode::Gradient {
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
        // A wire Buffer → Sensing is an observation tap: the sensor reads the
        // stock's LEVEL without draining it ("sensing is very low power").
        let is_observation = |w: &Wire| -> bool {
            matches!(self.nodes[w.from].kind, NodeKind::Process(ProcessPrimitive::Buffering))
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
            match sender.kind {
                NodeKind::Process(ProcessPrimitive::Copying) => sender.activity, // replicates
                _ if sender.out_substance == SubstanceType::Message => sender.activity,
                _ => sender.activity / outs, // conserve Energy/Material across fanout
            }
        };

        let mut next_activity = vec![0.0f32; n];
        let mut next_storage: Vec<f32> = self.nodes.iter().map(|x| x.storage).collect();
        let mut sink_add = vec![0.0f32; n];

        for (i, node) in self.nodes.iter().enumerate() {
            let incoming: Vec<(SubstanceType, f32)> = (0..nw)
                .filter(|&k| self.wires[k].to == i)
                .map(|k| (self.wire_substance(&self.wires[k]), amount_on(k)))
                .collect();
            let physical: f32 = incoming
                .iter()
                .filter(|(s, _)| *s != SubstanceType::Message)
                .map(|(_, a)| a)
                .sum();
            let message: f32 = incoming
                .iter()
                .filter(|(s, _)| *s == SubstanceType::Message)
                .map(|(_, a)| a)
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
                            .filter(|(s, _)| *s == SubstanceType::Energy)
                            .map(|(_, x)| x)
                            .sum();
                        let gain = 1.0 + 9.0 * a;
                        (message * gain).min(power)
                    }
                    // physical → signal (crosses substance, never drains)
                    ProcessPrimitive::Sensing => physical * a,
                    // primary gated by control in [0,1]
                    ProcessPrimitive::Modulating => physical * message.clamp(0.0, 1.0),
                    ProcessPrimitive::Inverting => (1.0 - message).max(0.0),
                    ProcessPrimitive::Copying => message,
                },
            };
        }

        for (i, node) in self.nodes.iter_mut().enumerate() {
            node.activity = next_activity[i];
            node.storage = next_storage[i];
            node.total += sink_add[i];
        }
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

    /// The recorded run as CSV: tick, then activity/storage/total per node.
    pub fn csv(&self) -> String {
        let mut out = String::from("tick");
        for node in &self.nodes {
            let name = node.name.replace(',', " ");
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
    /// silently ignored. Returns (node index, what it wants, what it's fed).
    pub fn substance_mismatches(&self) -> Vec<(usize, SubstanceType, SubstanceType)> {
        let mut out = Vec::new();
        for (i, node) in self.nodes.iter().enumerate() {
            for w in self.wires.iter().filter(|w| w.to == i) {
                let got = self.wire_substance(w);
                if !node.kind.consumes(got) {
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
        c.nodes[0].out_substance = SubstanceType::Message;
        c.nodes[1].param = 2.5;
        c.nodes[1].out_substance = SubstanceType::Energy;
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
        assert_eq!(m[0], (1, SubstanceType::Message, SubstanceType::Material));
        // Amplifying fed Material is also flagged now (was silently zeroing).
        let mut amp = Circuit::default();
        amp.nodes.push(node(NodeKind::Source)); // Material
        amp.nodes.push(node(NodeKind::Process(ProcessPrimitive::Amplifying)));
        amp.wires.push(Wire::new(0, 1));
        assert_eq!(amp.substance_mismatches().len(), 1, "Material -> Amplifying flagged");
        // Splitting fed Message is flagged (you split matter, you copy info).
        let mut sp = Circuit::default();
        sp.nodes.push(node(NodeKind::Process(ProcessPrimitive::Copying)));
        sp.nodes[0].out_substance = SubstanceType::Message;
        sp.nodes.push(node(NodeKind::Process(ProcessPrimitive::Splitting)));
        sp.wires.push(Wire::new(0, 1));
        assert_eq!(sp.substance_mismatches().len(), 1, "Message -> Splitting flagged");
        // Setting the source to emit Message clears it.
        c.nodes[0].out_substance = SubstanceType::Message;
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
