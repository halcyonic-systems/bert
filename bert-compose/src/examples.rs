//! Pre-loaded classic systems — the on-ramp. Each loads a working circuit a
//! social scientist or systems theorist can run, read, and tinker with. They
//! triple as the educational library, Troncale-sweep artifacts, and tests.

use crate::circuit::{Circuit, Node, NodeKind, Wire};
use bert_core::{ProcessPrimitive::*, SubstanceType};
use egui::pos2;

pub struct Example {
    pub name: &'static str,
    /// One line: what it shows.
    pub blurb: &'static str,
    pub build: fn() -> Circuit,
}

pub const EXAMPLES: &[Example] = &[
    Example {
        name: "Leaky bucket",
        blurb: "A store fills and drains — the simplest dynamics.",
        build: leaky_bucket,
    },
    Example {
        name: "Thermostat (homeostat)",
        blurb: "Negative feedback: the system regulates itself to a setpoint.",
        build: homeostat,
    },
    Example {
        name: "Splitting a budget",
        blurb: "Conservation you can watch: one inflow, two equal shares.",
        build: budget_split,
    },
    Example {
        name: "Megaphone (amp + power)",
        blurb: "A weak signal amplified — but only as far as the power allows.",
        build: megaphone,
    },
    Example {
        name: "Spreading the word",
        blurb: "Information copies for free; matter never could.",
        build: broadcast,
    },
];

fn n(kind: NodeKind, num: usize, x: f32, y: f32) -> Node {
    Node::new(kind, num, pos2(x, y))
}

/// Source → Buffer → Sink. The buffer starts full and drains.
fn leaky_bucket() -> Circuit {
    let mut c = Circuit::default();
    c.nodes.push(n(NodeKind::Source, 1, 380.0, 380.0));
    c.nodes.push(n(NodeKind::Process(Buffering), 2, 560.0, 380.0));
    c.nodes.push(n(NodeKind::Sink, 3, 740.0, 380.0));
    c.nodes[0].param = 0.5; // slow inflow
    c.nodes[1].initial_storage = 20.0; // starts full
    c.nodes[1].release_rate = 2.0; // drains faster than it fills
    c.nodes[1].storage = 20.0;
    c.wires.push(Wire { from: 0, to: 1 });
    c.wires.push(Wire { from: 1, to: 2 });
    c
}

/// Supply → Valve → Tank → Sink, with the tank's level sensed, inverted, and
/// fed back to the valve. The loop settles the tank toward a steady level.
fn homeostat() -> Circuit {
    let mut c = Circuit::default();
    c.nodes.push(n(NodeKind::Source, 1, 360.0, 320.0)); // 0 supply
    c.nodes.push(n(NodeKind::Process(Modulating), 2, 520.0, 320.0)); // 1 valve
    c.nodes.push(n(NodeKind::Process(Buffering), 3, 680.0, 320.0)); // 2 tank
    c.nodes.push(n(NodeKind::Sink, 4, 840.0, 320.0)); // 3 outflow
    c.nodes.push(n(NodeKind::Process(Sensing), 5, 680.0, 480.0)); // 4 gauge
    c.nodes.push(n(NodeKind::Process(Inverting), 6, 520.0, 480.0)); // 5 controller
    c.nodes[0].param = 3.0;
    c.nodes[2].release_rate = 1.0;
    c.nodes[4].param = 0.2; // gauge gain
    c.wires.push(Wire { from: 0, to: 1 }); // supply → valve
    c.wires.push(Wire { from: 1, to: 2 }); // valve → tank
    c.wires.push(Wire { from: 2, to: 3 }); // tank → sink
    c.wires.push(Wire { from: 2, to: 4 }); // tank level sensed
    c.wires.push(Wire { from: 4, to: 5 }); // gauge → controller
    c.wires.push(Wire { from: 5, to: 1 }); // controller → valve (closes loop)
    c
}

/// One Source, a Splitter, two Sinks. The shares always sum to the inflow.
fn budget_split() -> Circuit {
    let mut c = Circuit::default();
    c.nodes.push(n(NodeKind::Source, 1, 380.0, 380.0));
    c.nodes.push(n(NodeKind::Process(Splitting), 2, 560.0, 380.0));
    c.nodes.push(n(NodeKind::Sink, 3, 740.0, 300.0));
    c.nodes.push(n(NodeKind::Sink, 4, 740.0, 460.0));
    c.nodes[0].param = 4.0;
    c.wires.push(Wire { from: 0, to: 1 });
    c.wires.push(Wire { from: 1, to: 2 });
    c.wires.push(Wire { from: 1, to: 3 });
    c
}

/// A Message signal + an Energy power source into an Amplifier → Sink. Output
/// is capped by the power — the lesson the amp-with-power footgun teaches.
fn megaphone() -> Circuit {
    let mut c = Circuit::default();
    c.nodes.push(n(NodeKind::Source, 1, 360.0, 300.0)); // 0 signal
    c.nodes.push(n(NodeKind::Source, 2, 360.0, 460.0)); // 1 power
    c.nodes.push(n(NodeKind::Process(Amplifying), 3, 560.0, 380.0)); // 2 amp
    c.nodes.push(n(NodeKind::Sink, 4, 740.0, 380.0)); // 3
    c.nodes[0].param = 1.0;
    c.nodes[0].out_substance = SubstanceType::Message;
    c.nodes[1].param = 3.0;
    c.nodes[1].out_substance = SubstanceType::Energy;
    c.nodes[2].param = 1.0; // gain 10, but power caps at 3
    c.wires.push(Wire { from: 0, to: 2 });
    c.wires.push(Wire { from: 1, to: 2 });
    c.wires.push(Wire { from: 2, to: 3 });
    c
}

/// A Message source → Copying → three Sinks. Each gets the full message.
fn broadcast() -> Circuit {
    let mut c = Circuit::default();
    c.nodes.push(n(NodeKind::Source, 1, 360.0, 380.0));
    c.nodes.push(n(NodeKind::Process(Copying), 2, 540.0, 380.0));
    c.nodes.push(n(NodeKind::Sink, 3, 720.0, 280.0));
    c.nodes.push(n(NodeKind::Sink, 4, 720.0, 380.0));
    c.nodes.push(n(NodeKind::Sink, 5, 720.0, 480.0));
    c.nodes[0].param = 1.0;
    c.nodes[0].out_substance = SubstanceType::Message;
    c.wires.push(Wire { from: 0, to: 1 });
    c.wires.push(Wire { from: 1, to: 2 });
    c.wires.push(Wire { from: 1, to: 3 });
    c.wires.push(Wire { from: 1, to: 4 });
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every bundled example must be valid (no substance mismatches) and
    /// actually do something when run — the on-ramp can't ship dead circuits.
    #[test]
    fn all_examples_live_and_clean() {
        for ex in EXAMPLES {
            let mut c = (ex.build)();
            assert!(
                c.substance_mismatches().is_empty(),
                "{} has a substance mismatch",
                ex.name
            );
            for _ in 0..30 {
                c.step();
            }
            let moved: f32 = c.nodes.iter().map(|n| n.total + n.activity + n.storage).sum();
            assert!(moved > 0.0, "{} never moved anything", ex.name);
        }
    }
}
