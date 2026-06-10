//! Pre-loaded systems — the on-ramp, organized to mirror halcyonic.systems.
//!
//! **Foundations** teach the systems concepts (FLOWS / BONDS / FEEDBACK) and
//! stay in the neutral Systems lens — a leaky bucket relabeled "Supply" is
//! noise. The **domain** examples (Political Economy / Neuromorphics /
//! Protocol Science / Ecology) are native to one domain and load in that
//! domain's lens, where the renaming actually means something. The **Universal
//! homeostat** is the one cross-domain showcase: sweep every lens and the CSV
//! is identical (the K≅2 artifact).

use crate::circuit::{Circuit, DeclaredSubstance, Node, NodeKind, Wire, SUBSTANCES};
use bert_core::{ProcessPrimitive::*, SubstanceType};
use egui::pos2;

/// Look up a curated substance by name — examples speak human (water, money,
/// news), not just Energy/Material/Message.
fn substance(name: &str) -> DeclaredSubstance {
    let (n, b, u) = SUBSTANCES.iter().find(|(n, _, _)| *n == name).unwrap();
    DeclaredSubstance::named(n, *b, u)
}

/// Lens indices (see `lens::LENSES`): the menu groups + auto-applied lens map
/// onto the halcyonic.systems pillars.
const SYSTEMS: usize = 0;
const POLITICAL_ECONOMY: usize = 1;
const NEUROMORPHICS: usize = 2;
const PROTOCOL_SCIENCE: usize = 3;
const ECOLOGY: usize = 4;

/// Example categories, in menu order — mirrors the site's pillars.
#[derive(Clone, Copy, PartialEq)]
pub enum Category {
    Foundations,
    PoliticalEconomy,
    Neuromorphics,
    ProtocolScience,
    Ecology,
    CrossDomain,
}

impl Category {
    pub fn label(&self) -> &'static str {
        match self {
            Category::Foundations => "Foundations",
            Category::PoliticalEconomy => "Political Economy",
            Category::Neuromorphics => "Neuromorphics",
            Category::ProtocolScience => "Protocol Science",
            Category::Ecology => "Ecology",
            Category::CrossDomain => "Cross-domain",
        }
    }
    pub const ORDER: &'static [Category] = &[
        Category::Foundations,
        Category::PoliticalEconomy,
        Category::Neuromorphics,
        Category::ProtocolScience,
        Category::Ecology,
        Category::CrossDomain,
    ];
}

pub struct Example {
    pub name: &'static str,
    /// One line: what it shows.
    pub blurb: &'static str,
    pub category: Category,
    /// Lens auto-applied on load (Foundations stay in Systems; domain examples
    /// open in their domain's lens).
    pub lens: usize,
    pub build: fn() -> Circuit,
}

pub const EXAMPLES: &[Example] = &[
    // ── Foundations — the systems concepts, neutral lens ─────────────────
    Example {
        name: "Leaky bucket",
        blurb: "FLOWS: a store fills and drains — the simplest dynamics.",
        category: Category::Foundations,
        lens: SYSTEMS,
        build: leaky_bucket,
    },
    Example {
        name: "Splitting a budget",
        blurb: "BONDS: conservation you can watch — one inflow, two equal shares.",
        category: Category::Foundations,
        lens: SYSTEMS,
        build: budget_split,
    },
    Example {
        name: "Thermostat",
        blurb: "FEEDBACK: the system regulates itself to a setpoint.",
        category: Category::Foundations,
        lens: SYSTEMS,
        build: homeostat,
    },
    Example {
        name: "Battery (gradient)",
        blurb: "POTENTIAL FIELDS: a field, not a pump — charge falls down a gradient.",
        category: Category::Foundations,
        lens: SYSTEMS,
        build: battery,
    },
    Example {
        name: "Two paths to balance",
        blurb: "Passive gradient vs. active feedback — same equilibrium, two mechanisms.",
        category: Category::Foundations,
        lens: SYSTEMS,
        build: passive_vs_active,
    },
    Example {
        name: "Megaphone (amp + power)",
        blurb: "Amplification is metered — a weak signal grows only as far as the power allows.",
        category: Category::Foundations,
        lens: SYSTEMS,
        build: megaphone,
    },
    Example {
        name: "Spreading the word",
        blurb: "Information copies for free; matter never could.",
        category: Category::Foundations,
        lens: SYSTEMS,
        build: broadcast,
    },
    // ── Political Economy ────────────────────────────────────────────────
    Example {
        name: "Public budget on a quorum",
        blurb: "A treasury whose spending a quorum throttles, opposition damping as it depletes — self-governing fiscal policy.",
        category: Category::PoliticalEconomy,
        lens: POLITICAL_ECONOMY,
        build: public_budget,
    },
    // ── Neuromorphics ────────────────────────────────────────────────────
    Example {
        name: "Integrate-and-fire neuron",
        blurb: "A membrane integrating stimulus, inhibition holding it at threshold — homeostatic firing.",
        category: Category::Neuromorphics,
        lens: NEUROMORPHICS,
        build: integrate_and_fire,
    },
    // ── Protocol Science ─────────────────────────────────────────────────
    Example {
        name: "Difficulty-adjusted issuance",
        blurb: "Issuance into supply, difficulty retargeting to hold a setpoint — a protocol regulating its own money.",
        category: Category::ProtocolScience,
        lens: PROTOCOL_SCIENCE,
        build: difficulty_issuance,
    },
    // ── Ecology / Energy (Odum) ──────────────────────────────────────────
    Example {
        name: "Energy through a meadow",
        blurb: "Sunlight into biomass, a limiting factor holding it at carrying capacity — Odum's energese.",
        category: Category::Ecology,
        lens: ECOLOGY,
        build: meadow,
    },
    // ── Cross-domain showcase ────────────────────────────────────────────
    Example {
        name: "Universal homeostat",
        blurb: "One regulator — sweep the lens and read it as economy, neuro, protocol, or ecology. The CSV never changes.",
        category: Category::CrossDomain,
        lens: SYSTEMS,
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

// ── Domain examples ──────────────────────────────────────────────────────
//
// Each is native to ONE domain and loads in that domain's lens. They share
// the regulation skeleton on purpose — that sameness IS K≅2 — but carry the
// domain's substance and read naturally under its lens.

/// The shared negative-feedback regulator: a flow gated into a stock, the
/// stock sensed and fed back to throttle the gate. `sub` flows on the physical
/// path; the sensor/controller carry Message.
fn regulator(sub: DeclaredSubstance, rate: f32, release: f32, gain: f32) -> Circuit {
    let mut c = Circuit::default();
    c.nodes.push(n(NodeKind::Source, 1, 360.0, 320.0)); // 0 inflow
    c.nodes.push(n(NodeKind::Process(Modulating), 2, 520.0, 320.0)); // 1 gate
    c.nodes.push(n(NodeKind::Process(Buffering), 3, 680.0, 320.0)); // 2 stock
    c.nodes.push(n(NodeKind::Sink, 4, 840.0, 320.0)); // 3 outflow
    c.nodes.push(n(NodeKind::Process(Sensing), 5, 680.0, 480.0)); // 4 sensor
    c.nodes.push(n(NodeKind::Process(Inverting), 6, 520.0, 480.0)); // 5 controller
    c.nodes[0].param = rate;
    c.nodes[2].release_rate = release;
    c.nodes[4].param = gain;
    for i in [0, 1, 2] {
        c.nodes[i].out_substance = sub.clone();
    }
    c.wires.push(Wire::new(0, 1));
    c.wires.push(Wire::new(1, 2));
    c.wires.push(Wire::new(2, 3));
    c.wires.push(Wire::new(2, 4));
    c.wires.push(Wire::new(4, 5));
    c.wires.push(Wire::new(5, 1));
    c
}

/// POLITICAL ECONOMY — revenue gated by a quorum into a treasury, spending
/// allocated to two programs, opposition damping the quorum as the treasury
/// fills. A self-governing fiscal loop (reads under the Political Economy lens
/// as Constituency → Quorum gate → Registry → Allocation → Enactment).
fn public_budget() -> Circuit {
    let money = substance("money");
    let mut c = Circuit::default();
    c.nodes.push(n(NodeKind::Source, 1, 320.0, 300.0)); // 0 revenue
    c.nodes.push(n(NodeKind::Process(Modulating), 2, 470.0, 300.0)); // 1 quorum
    c.nodes.push(n(NodeKind::Process(Buffering), 3, 620.0, 300.0)); // 2 treasury
    c.nodes.push(n(NodeKind::Process(Splitting), 4, 770.0, 300.0)); // 3 allocation
    c.nodes.push(n(NodeKind::Sink, 5, 900.0, 230.0)); // 4 program A
    c.nodes.push(n(NodeKind::Sink, 6, 900.0, 370.0)); // 5 program B
    c.nodes.push(n(NodeKind::Process(Sensing), 7, 620.0, 470.0)); // 6 monitor
    c.nodes.push(n(NodeKind::Process(Inverting), 8, 470.0, 470.0)); // 7 opposition
    c.nodes[0].param = 3.0;
    c.nodes[2].release_rate = 1.0;
    c.nodes[6].param = 0.2;
    for i in [0, 1, 2, 3] {
        c.nodes[i].out_substance = money.clone();
    }
    c.wires.push(Wire::new(0, 1)); // revenue → quorum
    c.wires.push(Wire::new(1, 2)); // quorum → treasury
    c.wires.push(Wire::new(2, 3)); // treasury → allocation
    c.wires.push(Wire::new(3, 4)); // → program A
    c.wires.push(Wire::new(3, 5)); // → program B
    c.wires.push(Wire::new(2, 6)); // treasury sensed
    c.wires.push(Wire::new(6, 7)); // monitor → opposition
    c.wires.push(Wire::new(7, 1)); // opposition → quorum (closes loop)
    c
}

/// NEUROMORPHICS — stimulus gated through a synapse onto a membrane,
/// inhibition holding it near threshold. Integrate-and-fire as a homeostat
/// (Stimulus → Synapse → Membrane → Effector, Receptor → Inhibition).
fn integrate_and_fire() -> Circuit {
    regulator(DeclaredSubstance::named("charge", SubstanceType::Energy, "mV"), 3.0, 1.0, 0.25)
}

/// PROTOCOL SCIENCE — issuance gated by difficulty into circulating supply,
/// an oracle retargeting difficulty to hold a setpoint. A protocol regulating
/// its own money (Issuance → Difficulty → Supply → Burn, Oracle → Retarget).
fn difficulty_issuance() -> Circuit {
    regulator(DeclaredSubstance::named("tokens", SubstanceType::Material, "coins"), 3.0, 1.0, 0.2)
}

/// ECOLOGY / ENERGY — sunlight gated by a limiting factor into biomass,
/// damping holding it at carrying capacity. Odum's energese as a homeostat
/// (Inflow → Limiting factor → Biomass → Respiration, Indicator → Damping).
fn meadow() -> Circuit {
    regulator(substance("sunlight"), 3.0, 1.0, 0.2)
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
