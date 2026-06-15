//! Troncale's systems processes, as circuits built from Mobus primitives.
//!
//! This is the single source of truth for the dependency ladder: the palette
//! stamps these as macros (a "Feedback" block drops its primitive loop onto
//! the canvas, visible and editable — NOT a new atom), and the sweep
//! (`sweep.rs`) runs them as machine-demonstrated Linkage Propositions.
//!
//! The honesty is the point. A Troncale process is not a brick — it's a
//! *pattern wired from bricks*, which is exactly what the sweep proved. A few
//! of his processes DO coincide with atoms (Storage = the Buffering
//! primitive; Potential Fields = the gradient flow MODE), and those already
//! live in the primitive palette. The composites below stamp their loops so a
//! beginner can watch the process emerge from the parts.
//!
//! His own dependency ladder is the atomic→composite gradient: roots
//! (Potential Fields, Flows) sit nearest the atoms; higher processes
//! (Feedback → Oscillation → Networks) are increasingly composite.

use crate::circuit::{Circuit, Node, NodeKind, Wire};
use bert_core::ProcessPrimitive::*;

fn n(kind: NodeKind, num: usize, x: f32, y: f32) -> Node {
    Node::new(kind, num, egui::pos2(x, y))
}

/// One rung: a Troncale process, its primitive realization, and where it sits
/// in the four-bucket sweep (see `sweep.rs`).
///
/// `slug`/`provenance`/`bucket`/`ticks` are read by the sweep harness and
/// artifact emitter, which are `#[cfg(test)]` — so the release bin sees them
/// as unused. They're metadata of record, kept on the rung deliberately.
#[allow(dead_code)]
pub struct Rung {
    /// File-name slug for the sweep artifact bundle.
    pub slug: &'static str,
    /// Display name (the Troncale process).
    pub name: &'static str,
    /// What the run shows.
    pub blurb: &'static str,
    /// The honesty line: how it's wired from primitives.
    pub composition: &'static str,
    /// Troncale provenance (citation / dependency statement).
    pub provenance: &'static str,
    /// Sweep bucket: "a" constructible, "d?" feature-gated boundary, etc.
    pub bucket: &'static str,
    /// Ticks to run when emitting the artifact.
    pub ticks: usize,
    /// Offered as a stampable palette macro (clean constructive processes;
    /// boundary/degenerate rungs stay sweep-only).
    pub in_palette: bool,
    pub build: fn() -> Circuit,
}

pub const LADDER: &[Rung] = &[
    Rung {
        slug: "00-potential-fields",
        name: "Potential Fields",
        blurb: "Two stocks on a gradient equalize with no controller — passive homeostasis.",
        composition: "two Buffers joined by a gradient flow (the field is the driver)",
        provenance: "his deepest root: 'Flows require Potential Fields'; Mobus Ch.4 (fields are generalized flows)",
        bucket: "a",
        ticks: 200,
        in_palette: true,
        build: potential_fields,
    },
    Rung {
        slug: "01-flows",
        name: "Flows",
        blurb: "Substance crosses the system, throughput reaches the sink.",
        composition: "Source → Buffer → Sink",
        provenance: "his root: 'Flows require Potential Fields'",
        bucket: "a",
        ticks: 60,
        in_palette: true,
        build: flows,
    },
    Rung {
        slug: "03-feedback-regulation",
        name: "Feedback",
        blurb: "Negative feedback keeps the stock bounded — but it hunts (a limit cycle), it doesn't rest.",
        composition: "Sensing → Inverting → Modulating around a Buffer (the homeostat loop)",
        provenance: "his root; the homeostat (proven 6/09)",
        bucket: "a",
        ticks: 300,
        in_palette: true,
        build: feedback_regulation,
    },
    Rung {
        slug: "04-cycling-oscillation",
        name: "Oscillation",
        blurb: "Stiffen the loop and it overshoots to empty, then refills — a relaxation oscillator.",
        composition: "the same Feedback loop, high gain",
        provenance: "'Oscillations require Coupled Feedbacks require Cycling'",
        bucket: "a",
        ticks: 300,
        in_palette: true,
        build: cycling_oscillation,
    },
    Rung {
        slug: "06-decay",
        name: "Decay",
        blurb: "A stock releasing faster than it's fed drains to empty (linear, zeroth-order).",
        composition: "a Buffer whose release exceeds its inflow",
        provenance: "pathology family (Rheopathology / drain)",
        bucket: "a",
        ticks: 60,
        in_palette: true,
        build: decay,
    },
    Rung {
        slug: "07-networks",
        name: "Networks",
        blurb: "One inflow fans into many, conserved — the shares sum back to the input.",
        composition: "a Splitting node fanning out to several Sinks",
        provenance: "composition; Splitting/Combining fans",
        bucket: "a",
        ticks: 60,
        in_palette: true,
        build: networks,
    },
    // ── sweep-only (boundary / degenerate; not stampable) ────────────────
    Rung {
        slug: "05-coupled-predator-prey",
        name: "Coupled predator-prey",
        blurb: "Runs away — the predator grows unbounded under zeroth-order death.",
        composition: "two Buffers in mutual gating (prey eaten via a valve gated by predator level)",
        provenance: "predator-prey; the sweep's sharpest boundary (bert#85)",
        bucket: "d?",
        ticks: 300,
        in_palette: false,
        build: coupled_predator_prey,
    },
    Rung {
        slug: "08-emergence-part",
        name: "Emergence (the part)",
        blurb: "An isolated Buffer is inert — the contrast that makes the wired whole's dynamics emergent.",
        composition: "a single Buffer, unwired",
        provenance: "part of the part-vs-whole emergence probe",
        bucket: "a",
        ticks: 200,
        in_palette: false,
        build: emergence_part,
    },
];

/// The palette's stampable macros (clean constructive processes only).
pub fn palette_macros() -> impl Iterator<Item = &'static Rung> {
    LADDER.iter().filter(|r| r.in_palette)
}

/// Look up a rung by its display name — used by the inspector to describe a
/// stamped node's parent process.
pub fn by_name(name: &str) -> Option<&'static Rung> {
    LADDER.iter().find(|r| r.name == name)
}

// ── builders (the canonical circuits) ────────────────────────────────────

/// POTENTIAL FIELDS — a field is a flow MODE, not a node (Mobus Ch.4). Two
/// stocks joined by a gradient flow equalize with no controller.
pub fn potential_fields() -> Circuit {
    let mut c = Circuit::default();
    c.nodes
        .push(n(NodeKind::Process(Buffering), 1, 360.0, 320.0));
    c.nodes
        .push(n(NodeKind::Process(Buffering), 2, 620.0, 320.0));
    c.nodes[0].initial_storage = 20.0;
    c.nodes[0].storage = 20.0;
    c.nodes[0].release_rate = 0.0;
    c.nodes[1].release_rate = 0.0;
    c.wires.push(Wire::gradient(0, 1, 0.25));
    c
}

/// FLOWS — Source→Buffer→Sink: substance crosses the system.
pub fn flows() -> Circuit {
    let mut c = Circuit::default();
    c.nodes.push(n(NodeKind::Source, 1, 320.0, 320.0));
    c.nodes
        .push(n(NodeKind::Process(Buffering), 2, 520.0, 320.0));
    c.nodes.push(n(NodeKind::Sink, 3, 720.0, 320.0));
    c.nodes[0].param = 2.0;
    c.nodes[1].release_rate = 1.5;
    c.wires.push(Wire::new(0, 1));
    c.wires.push(Wire::new(1, 2));
    c
}

/// FEEDBACK — the homeostat. Low gain: a smooth limit cycle around the
/// setpoint (it hunts, never rests — fixed-point equilibrium is the passive
/// gradient's job, not the active loop's).
pub fn feedback_regulation() -> Circuit {
    homeostat(0.2)
}

/// OSCILLATION — the same loop, stiff gain: a relaxation oscillator that
/// overshoots to empty and refills. Cycling falls out of delayed feedback
/// automatically (his Oscillation→Feedback→Cycling dependency).
pub fn cycling_oscillation() -> Circuit {
    homeostat(0.9)
}

fn homeostat(gain: f32) -> Circuit {
    let mut c = Circuit::default();
    c.nodes.push(n(NodeKind::Source, 1, 320.0, 300.0)); // 0 supply
    c.nodes
        .push(n(NodeKind::Process(Modulating), 2, 480.0, 300.0)); // 1 valve
    c.nodes
        .push(n(NodeKind::Process(Buffering), 3, 640.0, 300.0)); // 2 stock
    c.nodes.push(n(NodeKind::Sink, 4, 800.0, 300.0)); // 3 outflow
    c.nodes.push(n(NodeKind::Process(Sensing), 5, 640.0, 460.0)); // 4 gauge
    c.nodes
        .push(n(NodeKind::Process(Inverting), 6, 480.0, 460.0)); // 5 control
    c.nodes[0].param = 3.0;
    c.nodes[2].release_rate = 1.0;
    c.nodes[4].param = gain;
    c.wires.push(Wire::new(0, 1));
    c.wires.push(Wire::new(1, 2));
    c.wires.push(Wire::new(2, 3));
    c.wires.push(Wire::new(2, 4));
    c.wires.push(Wire::new(4, 5));
    c.wires.push(Wire::new(5, 1));
    c
}

/// COUPLED PREDATOR-PREY — the boundary. Runs away: the predator stock grows
/// unbounded because Buffering's release is zeroth-order (constant amount/
/// tick), so constant-rate death can't balance a growing inflow. First-order
/// (proportional) death isn't expressible here — bert#85.
pub fn coupled_predator_prey() -> Circuit {
    let mut c = Circuit::default();
    c.nodes.push(n(NodeKind::Source, 1, 240.0, 240.0)); // 0 grass
    c.nodes
        .push(n(NodeKind::Process(Buffering), 2, 420.0, 240.0)); // 1 prey
    c.nodes
        .push(n(NodeKind::Process(Modulating), 3, 600.0, 240.0)); // 2 predation
    c.nodes
        .push(n(NodeKind::Process(Buffering), 4, 600.0, 440.0)); // 3 predator
    c.nodes.push(n(NodeKind::Sink, 5, 600.0, 600.0)); // 4 predator death
    c.nodes.push(n(NodeKind::Process(Sensing), 6, 420.0, 440.0)); // 5 senses predator
    c.nodes[0].param = 2.0;
    c.nodes[1].initial_storage = 8.0;
    c.nodes[1].storage = 8.0;
    c.nodes[1].release_rate = 2.0;
    c.nodes[3].initial_storage = 4.0;
    c.nodes[3].storage = 4.0;
    c.nodes[3].release_rate = 0.6;
    c.nodes[5].param = 0.15;
    c.wires.push(Wire::new(0, 1));
    c.wires.push(Wire::new(1, 2));
    c.wires.push(Wire::new(5, 2));
    c.wires.push(Wire::new(2, 3));
    c.wires.push(Wire::new(3, 4));
    c.wires.push(Wire::new(3, 5));
    c
}

/// DECAY — release > inflow drains the stock monotonically (linear).
pub fn decay() -> Circuit {
    let mut c = Circuit::default();
    c.nodes
        .push(n(NodeKind::Process(Buffering), 1, 420.0, 320.0));
    c.nodes.push(n(NodeKind::Sink, 2, 640.0, 320.0));
    c.nodes[0].initial_storage = 30.0;
    c.nodes[0].storage = 30.0;
    c.nodes[0].release_rate = 1.5;
    c.wires.push(Wire::new(0, 1));
    c
}

/// NETWORKS — a Splitting fan; shares sum back to the inflow, conserved.
pub fn networks() -> Circuit {
    let mut c = Circuit::default();
    c.nodes.push(n(NodeKind::Source, 1, 300.0, 320.0));
    c.nodes
        .push(n(NodeKind::Process(Splitting), 2, 480.0, 320.0));
    c.nodes.push(n(NodeKind::Sink, 3, 680.0, 220.0));
    c.nodes.push(n(NodeKind::Sink, 4, 680.0, 320.0));
    c.nodes.push(n(NodeKind::Sink, 5, 680.0, 420.0));
    c.nodes[0].param = 6.0;
    c.wires.push(Wire::new(0, 1));
    c.wires.push(Wire::new(1, 2));
    c.wires.push(Wire::new(1, 3));
    c.wires.push(Wire::new(1, 4));
    c
}

/// EMERGENCE (the part) — a single isolated Buffer is inert; the contrast
/// that makes the wired whole's dynamics emergent.
pub fn emergence_part() -> Circuit {
    let mut c = Circuit::default();
    c.nodes
        .push(n(NodeKind::Process(Buffering), 1, 480.0, 320.0));
    c.nodes[0].initial_storage = 8.0;
    c.nodes[0].storage = 8.0;
    c.nodes[0].release_rate = 0.0;
    c
}
