//! Pre-loaded classic systems — the on-ramp. Each loads a working circuit a
//! social scientist or systems theorist can run, read, and tinker with. They
//! triple as the educational library, Troncale-sweep artifacts, and tests.

use crate::circuit::{Circuit, DeclaredSubstance, Node, NodeKind, Wire, SUBSTANCES};
use bert_core::{ProcessPrimitive::*, SubstanceType};
use egui::pos2;

/// Look up a curated substance by name — examples speak human (water, money,
/// news), not just Energy/Material/Message.
fn substance(name: &str) -> DeclaredSubstance {
    let (n, b, u) = SUBSTANCES.iter().find(|(n, _, _)| *n == name).unwrap();
    DeclaredSubstance::named(n, *b, u)
}

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
    Example {
        name: "Battery (gradient flow)",
        blurb: "A field, not a pump: charge flows down a gradient and equalizes.",
        build: battery,
    },
    Example {
        name: "Two paths to balance",
        blurb: "Passive gradient vs. active feedback — same equilibrium, two mechanisms.",
        build: passive_vs_active,
    },
    Example {
        name: "Universal homeostat",
        blurb: "One regulator — read it as crypto, governance, neuro, or ecology (switch lenses).",
        build: universal_homeostat,
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
    c.nodes[0].out_substance = substance("water");
    c.nodes[1].out_substance = substance("water");
    c.wires.push(Wire::new(0, 1));
    c.wires.push(Wire::new(1, 2));
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
    for i in [0, 1, 2] {
        c.nodes[i].out_substance = substance("water");
    }
    c.wires.push(Wire::new(0, 1)); // supply → valve
    c.wires.push(Wire::new(1, 2)); // valve → tank
    c.wires.push(Wire::new(2, 3)); // tank → sink
    c.wires.push(Wire::new(2, 4)); // tank level sensed
    c.wires.push(Wire::new(4, 5)); // gauge → controller
    c.wires.push(Wire::new(5, 1)); // controller → valve (closes loop)
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
    c.nodes[0].out_substance = substance("money");
    c.nodes[1].out_substance = substance("money");
    c.wires.push(Wire::new(0, 1));
    c.wires.push(Wire::new(1, 2));
    c.wires.push(Wire::new(1, 3));
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
    c.nodes[0].out_substance = SubstanceType::Message.into();
    c.nodes[1].param = 3.0;
    c.nodes[1].out_substance = substance("electricity");
    c.nodes[2].param = 1.0; // gain 10, but power caps at 3
    c.wires.push(Wire::new(0, 2));
    c.wires.push(Wire::new(1, 2));
    c.wires.push(Wire::new(2, 3));
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
    c.nodes[0].out_substance = substance("news");
    c.nodes[1].out_substance = substance("news");
    c.wires.push(Wire::new(0, 1));
    c.wires.push(Wire::new(1, 2));
    c.wires.push(Wire::new(1, 3));
    c.wires.push(Wire::new(1, 4));
    c
}

/// A full stock and an empty one joined by a gradient flow — charge falls
/// down the potential until they equalize. No pump, no controller: the field
/// IS the driver (Mobus: forces/fields are generalized flows). The wire
/// visibly thins as the gradient closes.
fn battery() -> Circuit {
    let mut c = Circuit::default();
    c.nodes.push(n(NodeKind::Process(Buffering), 1, 420.0, 360.0)); // charged
    c.nodes.push(n(NodeKind::Process(Buffering), 2, 660.0, 360.0)); // empty
    c.nodes[0].initial_storage = 20.0;
    c.nodes[0].storage = 20.0;
    c.nodes[0].release_rate = 0.0;
    c.nodes[1].release_rate = 0.0;
    c.nodes[0].out_substance = substance("electricity");
    c.nodes[1].out_substance = substance("electricity");
    c.wires.push(Wire::gradient(0, 1, 0.25));
    c
}

/// Two tanks reaching the same steady level by different means. Left pair:
/// a gradient flow equalizes them passively. Right loop: a source + valve +
/// sensing-feedback holds a tank at a setpoint actively. Both are Troncale
/// regulation processes; one is a field, one is a controller.
fn passive_vs_active() -> Circuit {
    let mut c = Circuit::default();
    // Passive: full buffer --gradient--> empty buffer.
    c.nodes.push(n(NodeKind::Process(Buffering), 1, 320.0, 250.0)); // 0
    c.nodes.push(n(NodeKind::Process(Buffering), 2, 520.0, 250.0)); // 1
    c.nodes[0].initial_storage = 16.0;
    c.nodes[0].storage = 16.0;
    c.nodes[0].release_rate = 0.0;
    c.nodes[1].release_rate = 0.0;
    c.wires.push(Wire::gradient(0, 1, 0.25));
    // Active: supply -> valve -> tank -> sink, with sensing->inverting feedback.
    c.nodes.push(n(NodeKind::Source, 3, 320.0, 470.0)); // 2
    c.nodes.push(n(NodeKind::Process(Modulating), 4, 480.0, 470.0)); // 3
    c.nodes.push(n(NodeKind::Process(Buffering), 5, 640.0, 470.0)); // 4
    c.nodes.push(n(NodeKind::Sink, 6, 800.0, 470.0)); // 5
    c.nodes.push(n(NodeKind::Process(Sensing), 7, 640.0, 600.0)); // 6
    c.nodes.push(n(NodeKind::Process(Inverting), 8, 480.0, 600.0)); // 7
    c.nodes[2].param = 3.0;
    c.nodes[4].release_rate = 1.0;
    c.nodes[6].param = 0.2;
    for i in [0, 1, 2, 3, 4] {
        c.nodes[i].out_substance = substance("water");
    }
    c.wires.push(Wire::new(2, 3));
    c.wires.push(Wire::new(3, 4));
    c.wires.push(Wire::new(4, 5));
    c.wires.push(Wire::new(4, 6));
    c.wires.push(Wire::new(6, 7));
    c.wires.push(Wire::new(7, 3));
    c
}

/// The same negative-feedback regulator as the thermostat, but with auto
/// names and a domain-neutral substance — built to be READ through every
/// lens. Difficulty adjustment, quorum throttle, spike threshold, limiting
/// factor: one circuit, four readings, identical dynamics. The K≅2 artifact.
fn universal_homeostat() -> Circuit {
    let mut c = Circuit::default();
    c.nodes.push(n(NodeKind::Source, 1, 360.0, 320.0)); // 0
    c.nodes.push(n(NodeKind::Process(Modulating), 2, 520.0, 320.0)); // 1
    c.nodes.push(n(NodeKind::Process(Buffering), 3, 680.0, 320.0)); // 2
    c.nodes.push(n(NodeKind::Sink, 4, 840.0, 320.0)); // 3
    c.nodes.push(n(NodeKind::Process(Sensing), 5, 680.0, 480.0)); // 4
    c.nodes.push(n(NodeKind::Process(Inverting), 6, 520.0, 480.0)); // 5
    c.nodes[0].param = 3.0;
    c.nodes[2].release_rate = 1.0;
    c.nodes[4].param = 0.2;
    c.wires.push(Wire::new(0, 1));
    c.wires.push(Wire::new(1, 2));
    c.wires.push(Wire::new(2, 3));
    c.wires.push(Wire::new(2, 4));
    c.wires.push(Wire::new(4, 5));
    c.wires.push(Wire::new(5, 1));
    c
}

/// The universal homeostat, exposed for the lens-invariance test in lens.rs.
#[cfg(test)]
pub fn universal_homeostat_for_test() -> Circuit {
    universal_homeostat()
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
            // Every example's mass must be fully accounted by the ledger.
            let scale = (c.emitted
                + c.nodes.iter().map(|n| n.initial_storage).sum::<f32>())
            .max(1.0);
            assert!(
                c.balance().abs() <= 1e-3 * scale,
                "{} leaks: residual {} (dissipated {})",
                ex.name,
                c.balance(),
                c.dissipated
            );
        }
    }
}
