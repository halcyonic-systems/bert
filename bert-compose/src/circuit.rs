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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Wire {
    pub from: usize,
    pub to: usize,
}

#[derive(Default)]
pub struct Circuit {
    pub nodes: Vec<Node>,
    pub wires: Vec<Wire>,
    pub tick: u64,
}

impl Circuit {
    pub fn reset(&mut self) {
        for n in &mut self.nodes {
            n.storage = n.initial_storage;
            n.activity = 0.0;
            n.total = 0.0;
        }
        self.tick = 0;
    }

    /// Substance carried by a wire = the sender's output substance.
    pub fn wire_substance(&self, w: &Wire) -> SubstanceType {
        self.nodes[w.from].out_substance
    }

    /// One synchronous step: all transfer functions read the previous tick's
    /// wire amounts (sender activity at t-1) and write activity for t.
    pub fn step(&mut self) {
        let n = self.nodes.len();
        // Previous-tick amount arriving over each wire, split when the sender
        // fans out a conservative substance (Splitting/Copying handle their
        // own fanout semantics; everything else divides physical flow).
        // A wire Buffer → Sensing is an observation tap: the sensor reads the
        // stock's LEVEL without draining it (python/agents.py: observation
        // flows read a frozen level snapshot; "sensing is very low power").
        let is_observation = |w: &Wire| -> bool {
            matches!(self.nodes[w.from].kind, NodeKind::Process(ProcessPrimitive::Buffering))
                && matches!(self.nodes[w.to].kind, NodeKind::Process(ProcessPrimitive::Sensing))
        };
        let amount_on = |w: &Wire| -> f32 {
            if is_observation(w) {
                return self.nodes[w.from].storage; // non-draining level read
            }
            let sender = &self.nodes[w.from];
            // Observation taps don't drain, so they don't count toward fanout.
            let outs = self
                .wires
                .iter()
                .filter(|x| x.from == w.from && !is_observation(x))
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
            let incoming: Vec<(SubstanceType, f32)> = self
                .wires
                .iter()
                .filter(|w| w.to == i)
                .map(|w| (self.wire_substance(w), amount_on(w)))
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
                    // storage += inflow; release = min(storage, rate · gate)
                    ProcessPrimitive::Buffering => {
                        let mut storage = next_storage[i] + physical;
                        let gate = if self.wires.iter().any(|w| {
                            w.to == i && self.wire_substance(w) == SubstanceType::Message
                        }) {
                            message.clamp(0.0, 1.0)
                        } else {
                            1.0
                        };
                        let released = (node.release_rate * gate).min(storage);
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
        c.wires.push(Wire { from: 0, to: 1 });
        c.wires.push(Wire { from: 1, to: 2 });
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
        c.wires.push(Wire { from: 0, to: 1 }); // supply → valve
        c.wires.push(Wire { from: 1, to: 2 }); // valve → stock
        c.wires.push(Wire { from: 2, to: 3 }); // stock outflow sensed
        c.wires.push(Wire { from: 3, to: 4 }); // sensor → inverter
        c.wires.push(Wire { from: 4, to: 1 }); // control closes the loop (gate)
        c.wires.push(Wire { from: 2, to: 5 }); // stock → sink
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
        c.wires.push(Wire { from: 0, to: 2 });
        c.wires.push(Wire { from: 1, to: 2 });
        c.wires.push(Wire { from: 2, to: 3 });
        for _ in 0..5 {
            c.step();
        }
        // desired = 1.0 * 10 = 10, but only 2.5 energy available
        assert!((c.nodes[2].activity - 2.5).abs() < f32::EPSILON);
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
            c.wires.push(Wire { from: f, to: t });
        }
        assert_eq!(c.diversity(), 3, "source, {{buffer,buffer}}, sink");
    }
}
