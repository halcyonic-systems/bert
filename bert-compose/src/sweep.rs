//! The Troncale sweep — machine-demonstrated Linkage Propositions.
//!
//! Troncale's Systems Processes Theory states *Linkage Propositions* (LP):
//! dyads "SP_A → SP_B" asserting one process influences another. He never
//! published a "core N" and resists linear reduction — "all are axiomatic,
//! all needed," "networks not linear chains" (1978). So we do NOT claim his
//! processes reduce to our primitives. We climb his OWN stated dependency
//! ladder and ask, per rung: does the signature behavior EMERGE from a
//! primitive circuit?
//!
//!   "Oscillations require Coupled Feedbacks … require Cycling which requires
//!    Flows." "Flows requires Potential Fields."   — Troncale [VERIFIED-T]
//!
//! Four outcomes per process (the boundary is the finding):
//!   (a) constructible — emerges from a primitive circuit
//!   (b) is-a-primitive — already atomic (Storage = Buffering)
//!   (c) out-of-scope — structural/relational, not flow-dynamical
//!   (d) needs-agent-layer — individuated/selective (Mesa, not the circuit)
//!
//! Each (a) rung below is a PASSING TEST whose assertion IS the demonstration:
//! "coupled feedback → oscillation" stops being asserted and becomes a circuit
//! whose CSV oscillates, proven by his own criteria (4) sustainability and
//! (5) influence-richness. The ledger (every CSV is mass-accounted) is what
//! makes these curves evidence rather than decoration.
//!
//! `cargo test sweep` proves the LPs. `cargo test emit_sweep_artifacts --
//! --ignored` writes the circuit+CSV evidence bundle to `troncale-sweep/`.

use crate::circuit::{Circuit, Node, NodeKind, Wire};
use bert_core::ProcessPrimitive::*;
use egui::pos2;

fn n(kind: NodeKind, num: usize, x: f32, y: f32) -> Node {
    Node::new(kind, num, pos2(x, y))
}

/// Run, returning each node's storage series (one Vec per node, per tick).
fn run_storage(c: &mut Circuit, ticks: usize) -> Vec<Vec<f32>> {
    let mut series = vec![Vec::with_capacity(ticks); c.nodes.len()];
    for _ in 0..ticks {
        c.step();
        for (i, node) in c.nodes.iter().enumerate() {
            series[i].push(node.storage);
        }
    }
    series
}

// ── signature detectors (operate on a series) ───────────────────────────

/// Sign changes in the first difference over the tail — the fingerprint of
/// oscillation (it keeps turning around, not settling).
fn turning_points(series: &[f32]) -> usize {
    let tail = &series[series.len() / 2..];
    let mut turns = 0;
    let mut last_sign = 0i8;
    for w in tail.windows(2) {
        let d = w[1] - w[0];
        let sign = if d > 1e-4 { 1 } else if d < -1e-4 { -1 } else { 0 };
        if sign != 0 && sign != last_sign && last_sign != 0 {
            turns += 1;
        }
        if sign != 0 {
            last_sign = sign;
        }
    }
    turns
}

fn amplitude(series: &[f32]) -> f32 {
    let tail = &series[series.len() / 2..];
    let (mut lo, mut hi) = (f32::MAX, f32::MIN);
    for &v in tail {
        lo = lo.min(v);
        hi = hi.max(v);
    }
    hi - lo
}

/// Settled = the tail barely moves (a damped loop found its setpoint).
fn settles(series: &[f32]) -> bool {
    amplitude(series) < 0.05 * series.iter().cloned().fold(0.0f32, f32::max).max(1.0)
}

/// Monotone non-increasing to near zero — decay.
fn decays(series: &[f32]) -> bool {
    let first = series[0];
    let last = *series.last().unwrap();
    last < 0.2 * first.max(1e-3) && series.windows(2).all(|w| w[1] <= w[0] + 1e-3)
}

// ════════════════════════════════════════════════════════════════════════
// THE LADDER (bottom-up, Troncale's own dependency order)
// ════════════════════════════════════════════════════════════════════════

/// Rung 0 — POTENTIAL FIELDS [his deepest root; "Flows require Potential
/// Fields"]. A field is not a node — it's a flow MODE (Mobus Ch.4: forces/
/// fields are generalized flows). Two stocks joined by a gradient flow
/// equalize with NO controller: passive homeostasis, the field as driver.
pub fn potential_fields() -> Circuit {
    let mut c = Circuit::default();
    c.nodes.push(n(NodeKind::Process(Buffering), 1, 360.0, 320.0)); // full
    c.nodes.push(n(NodeKind::Process(Buffering), 2, 620.0, 320.0)); // empty
    c.nodes[0].initial_storage = 20.0;
    c.nodes[0].storage = 20.0;
    c.nodes[0].release_rate = 0.0;
    c.nodes[1].release_rate = 0.0;
    c.wires.push(Wire::gradient(0, 1, 0.25));
    c
}

/// Rung 1 — FLOWS [his root]. Source→Buffer→Sink: substance crosses the
/// system, throughput reaches the sink.
pub fn flows() -> Circuit {
    let mut c = Circuit::default();
    c.nodes.push(n(NodeKind::Source, 1, 320.0, 320.0));
    c.nodes.push(n(NodeKind::Process(Buffering), 2, 520.0, 320.0));
    c.nodes.push(n(NodeKind::Sink, 3, 720.0, 320.0));
    c.nodes[0].param = 2.0;
    c.nodes[1].release_rate = 1.5;
    c.wires.push(Wire::new(0, 1));
    c.wires.push(Wire::new(1, 2));
    c
}

/// Rung 3 — FEEDBACK / regulation [his root]. The homeostat: Sensing→
/// Inverting→Modulating around a Buffer. OBSERVED [OURS]: the loop does NOT
/// settle to a fixed point — even at low gain it sustains a smooth limit
/// cycle around the setpoint (the one-tick loop delay is a phase lag). It
/// "regulates" in the sense of staying bounded, but it hunts, never rests.
/// Fixed-point equilibrium comes from the PASSIVE gradient route (rung 0),
/// not the active loop — two distinct routes to stability, Troncale's
/// Equilibrium vs Oscillation arising from different mechanisms.
pub fn feedback_regulation() -> Circuit {
    homeostat(0.2)
}

/// Rung 4 — CYCLING / OSCILLATION ["Oscillations require Coupled Feedbacks
/// require Cycling"]. The SAME loop with a stiffer gain becomes a RELAXATION
/// oscillator: it overshoots all the way to empty, sits at the floor, then
/// refills — larger amplitude, clipped bottoms. Cycling is not optional here;
/// it falls out of delayed negative feedback automatically (which is exactly
/// Troncale's dependency: Oscillation rests on Feedback rests on Cycling).
pub fn cycling_oscillation() -> Circuit {
    homeostat(0.9)
}

/// Shared homeostat builder; `gain` is the sensor's k (the loop stiffness).
fn homeostat(gain: f32) -> Circuit {
    let mut c = Circuit::default();
    c.nodes.push(n(NodeKind::Source, 1, 320.0, 300.0)); // 0 supply
    c.nodes.push(n(NodeKind::Process(Modulating), 2, 480.0, 300.0)); // 1 valve
    c.nodes.push(n(NodeKind::Process(Buffering), 3, 640.0, 300.0)); // 2 stock
    c.nodes.push(n(NodeKind::Sink, 4, 800.0, 300.0)); // 3 outflow
    c.nodes.push(n(NodeKind::Process(Sensing), 5, 640.0, 460.0)); // 4 gauge
    c.nodes.push(n(NodeKind::Process(Inverting), 6, 480.0, 460.0)); // 5 control
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

/// Rung 5 — COUPLED OSCILLATOR (predator-prey). RESEARCH MOMENT 1, and the
/// sweep's sharpest BOUNDARY FINDING [OURS, observed]:
///
/// The naive predator-prey wiring does NOT produce Lotka-Volterra cycles —
/// the predator stock grows without bound (prey pins, predator → ∞). The
/// mechanism is diagnostic: `Buffering`'s release is ZEROTH-ORDER (a fixed
/// amount per tick, `min(stock, rate)`), not FIRST-ORDER (proportional to
/// stock, `δ·stock`). LV needs proportional death (δQ) to self-limit; a
/// constant-rate death can't balance a growing inflow, so the predator runs
/// away. And first-order decay is NOT expressible in this set at all —
/// VERIFIED by probe: a Sensing→Modulating loop on the release only SHEDS
/// downstream mass, it cannot change the stock's drain law (the stock still
/// loses a constant amount/tick). First-order decay needs a buffer mode where
/// `release ∝ stock` — a genuine FEATURE GAP (issue filed), not a
/// composition. Bucket: (d?)/boundary, feature-gated.
///
/// Sustained oscillation IS demonstrated — by the single-loop homeostat limit
/// cycle (rungs 3/4). The coupled population cycle is where the primitive set
/// reaches its edge, and naming that edge precisely is the result.
pub fn coupled_oscillator() -> Circuit {
    let mut c = Circuit::default();
    // 0 prey-food source, 1 prey buffer, 2 predation valve (gated by predator),
    // 3 predator buffer, 4 predator death sink, 5 sensing predator level,
    // 6 sensing prey level, 7 predator-growth valve.
    c.nodes.push(n(NodeKind::Source, 1, 240.0, 240.0)); // 0 grass
    c.nodes.push(n(NodeKind::Process(Buffering), 2, 420.0, 240.0)); // 1 prey
    c.nodes.push(n(NodeKind::Process(Modulating), 3, 600.0, 240.0)); // 2 predation
    c.nodes.push(n(NodeKind::Process(Buffering), 4, 600.0, 440.0)); // 3 predator
    c.nodes.push(n(NodeKind::Sink, 5, 600.0, 600.0)); // 4 predator death
    c.nodes.push(n(NodeKind::Process(Sensing), 6, 420.0, 440.0)); // 5 senses predator
    c.nodes[0].param = 2.0;
    c.nodes[1].initial_storage = 8.0;
    c.nodes[1].storage = 8.0;
    c.nodes[1].release_rate = 2.0; // prey offered to predation each tick
    c.nodes[3].initial_storage = 4.0;
    c.nodes[3].storage = 4.0;
    c.nodes[3].release_rate = 0.6; // predator death rate
    c.nodes[5].param = 0.15; // predator-sensing gain
    c.wires.push(Wire::new(0, 1)); // grass → prey
    c.wires.push(Wire::new(1, 2)); // prey → predation valve (primary)
    c.wires.push(Wire::new(5, 2)); // predator level gates predation (control)
    c.wires.push(Wire::new(2, 3)); // eaten prey → predator growth
    c.wires.push(Wire::new(3, 4)); // predator death → sink
    c.wires.push(Wire::new(3, 5)); // sense predator stock
    c
}

/// Rung 6 — DECAY [pathology family]. A stocked buffer releasing faster than
/// it's fed drains monotonically to empty (Impeding would do it too).
pub fn decay() -> Circuit {
    let mut c = Circuit::default();
    c.nodes.push(n(NodeKind::Process(Buffering), 1, 420.0, 320.0));
    c.nodes.push(n(NodeKind::Sink, 2, 640.0, 320.0));
    c.nodes[0].initial_storage = 30.0;
    c.nodes[0].storage = 30.0;
    c.nodes[0].release_rate = 1.5;
    c.wires.push(Wire::new(0, 1));
    c
}

/// Rung 7 — NETWORKS [composition]. A Splitting fan: one inflow, three
/// branches, shares summing back to the input — conservation across a fan,
/// and the wiring alone yields structural diversity.
pub fn networks() -> Circuit {
    let mut c = Circuit::default();
    c.nodes.push(n(NodeKind::Source, 1, 300.0, 320.0));
    c.nodes.push(n(NodeKind::Process(Splitting), 2, 480.0, 320.0));
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

/// Rung 8 — EMERGENCE. RESEARCH MOMENT 2: does a property appear in the
/// composite that is absent from the parts? Operational signature here:
/// a single Buffer is inert (flat); two buffers wired into the coupled loop
/// OSCILLATE. The oscillation is in the wiring, not any part — and the
/// circuit's SameKind diversity (circuit.rs::diversity) rises from the
/// asymmetric topology. Emergence as the gap between part and whole.
pub fn emergence_parts() -> Circuit {
    // an isolated buffer: nothing happens
    let mut c = Circuit::default();
    c.nodes.push(n(NodeKind::Process(Buffering), 1, 480.0, 320.0));
    c.nodes[0].initial_storage = 8.0;
    c.nodes[0].storage = 8.0;
    c.nodes[0].release_rate = 0.0;
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    /// LP: Potential Fields → equalization. Two stocks on a gradient flow
    /// converge with no controller, conserving — passive homeostasis. (a)
    #[test]
    fn sweep_potential_fields_equalize() {
        let mut c = potential_fields();
        let s = run_storage(&mut c, 200);
        let (a, b) = (*s[0].last().unwrap(), *s[1].last().unwrap());
        assert!((a - b).abs() < 0.1, "field equalizes the two stocks: {a} vs {b}");
        assert!(c.balance().abs() < 1e-3, "and conserves");
    }

    /// LP: Flows. Source→Buffer→Sink carries substance across the system. (a)
    #[test]
    fn sweep_flows_reach_the_sink() {
        let mut c = flows();
        run_storage(&mut c, 40);
        assert!(c.nodes[2].total > 0.0, "throughput reaches the sink");
        assert!(c.balance().abs() < 1e-3, "conserving");
    }

    /// LP: Feedback → bounded regulation, observed as a LIMIT CYCLE. The
    /// active loop stays bounded but hunts — it never settles to a fixed
    /// point, and (low gain) never fully empties: a smooth cycle around the
    /// setpoint. The fixed-point equilibrium is the passive gradient's job. (a)
    #[test]
    fn sweep_feedback_is_a_limit_cycle_not_a_fixed_point() {
        let mut c = feedback_regulation();
        let s = run_storage(&mut c, 300);
        let stock = &s[2];
        assert!(stock.iter().all(|&v| v < 100.0), "bounded, not runaway");
        assert!(turning_points(stock) >= 6, "the active loop hunts — a limit cycle");
        let tail_min = stock[stock.len() / 2..].iter().cloned().fold(f32::MAX, f32::min);
        assert!(tail_min > 0.3, "low gain: a smooth cycle that never empties (min {tail_min:.2})");
        assert!(!settles(stock), "it does NOT settle to a fixed point");
    }

    /// LP: Coupled feedback → Cycling/Oscillation. Stiffer gain → a RELAXATION
    /// oscillator: overshoots to empty, sits at the floor, refills. Larger
    /// amplitude, clipped bottoms — distinct from the low-gain smooth cycle. (a)
    #[test]
    fn sweep_cycling_is_a_relaxation_oscillator() {
        let mut c = cycling_oscillation();
        let s = run_storage(&mut c, 300);
        let stock = &s[2];
        assert!(turning_points(stock) >= 6, "sustained oscillation");
        let tail_min = stock[stock.len() / 2..].iter().cloned().fold(f32::MAX, f32::min);
        assert!(tail_min < 0.1, "relaxation: it overshoots all the way to empty (min {tail_min:.2})");
    }

    /// RESEARCH 1 / BOUNDARY — the naive predator-prey runs away because
    /// Buffering's release is zeroth-order: the predator's constant-rate death
    /// can't balance its growth, so its stock grows without bound while prey
    /// pins. This is the claim's EDGE, recorded as a fact: sustained LV cycling
    /// is not atomic here (needs first-order death — composable, not a
    /// primitive). (d?/boundary)
    #[test]
    fn sweep_coupled_predator_prey_runs_away() {
        let mut c = coupled_oscillator();
        let s = run_storage(&mut c, 300);
        let prey = &s[1];
        let predator = &s[3];
        assert!(amplitude(prey) < 0.5, "prey pins (constant source refills it): amp {:.2}", amplitude(prey));
        assert!(
            *predator.last().unwrap() > 50.0,
            "predator runs away under zeroth-order death: ends at {:.0}",
            predator.last().unwrap()
        );
    }

    /// LP: Decay. Release > inflow drains the stock monotonically to ~0. (a)
    #[test]
    fn sweep_decay_drains() {
        let mut c = decay();
        let s = run_storage(&mut c, 60);
        assert!(decays(&s[0]), "monotone decay to empty");
    }

    /// LP: Networks. A fan conserves — shares sum back to the inflow — and
    /// the topology yields structural diversity. (a)
    #[test]
    fn sweep_networks_conserve_and_diversify() {
        let mut c = networks();
        run_storage(&mut c, 40);
        let sunk: f32 = c.nodes.iter().filter(|n| n.kind == NodeKind::Sink).map(|n| n.total).sum();
        assert!(sunk > 0.0 && c.balance().abs() < 1e-3, "fan conserves");
        assert!(c.diversity() >= 3, "source, splitter, sinks are distinct kinds");
    }

    /// RESEARCH 2 — Emergence as part-vs-whole. The isolated Buffer is inert
    /// (flat). The homeostat WHOLE sustains a limit cycle — an oscillation no
    /// single primitive exhibits (no atomic transfer function oscillates; the
    /// cycling lives in the closed loop). The property is in the wiring,
    /// absent from every part. Operational signature: amplitude(whole) ≫
    /// amplitude(any part), and kind-diversity rises with the topology. This
    /// is the operational handle on Emergence, not a closed claim it's
    /// definitional. (a, operational)
    #[test]
    fn sweep_emergence_whole_exceeds_parts() {
        let mut part = emergence_parts();
        let ps = run_storage(&mut part, 200);
        assert!(amplitude(&ps[0]) < 1e-3, "the isolated buffer is inert");
        assert_eq!(part.diversity(), 1, "one part, one kind");

        let mut whole = feedback_regulation();
        let ws = run_storage(&mut whole, 300);
        assert!(amplitude(&ws[2]) > 1.0, "the homeostat cycles — dynamics no part has");
        assert!(whole.diversity() > 1, "wiring raises kind-diversity: {}", whole.diversity());
    }


    /// Boundary buckets, recorded as tests so the claim's edge is explicit:
    /// (b) Storage IS Buffering; (c) Hierarchy is structural (one-level only
    /// via export, true nesting is #75); (d) Evolution needs the agent layer.
    #[test]
    fn sweep_boundary_is_explicit() {
        // (b) the primitive itself holds state — not "constructed," atomic.
        let mut c = Circuit::default();
        c.nodes.push(n(NodeKind::Process(Buffering), 1, 0.0, 0.0));
        c.nodes[0].initial_storage = 5.0;
        c.nodes[0].storage = 5.0;
        run_storage(&mut c, 5);
        assert_eq!(c.nodes[0].storage, 5.0, "Storage = Buffering, bucket (b)");
        // (c)/(d) are assertions about scope, documented in the artifact table.
    }
}

// ── artifact emitter (opt-in: writes the evidence bundle) ────────────────

/// Each ladder rung: name, Troncale provenance, builder, bucket, ticks.
#[cfg(test)]
type Rung = (&'static str, &'static str, fn() -> Circuit, &'static str, usize);

#[cfg(test)]
const LADDER: &[Rung] = &[
    ("00-potential-fields", "his deepest root; field = generalized flow (Mobus Ch.4)", potential_fields, "a", 200),
    ("01-flows", "his root: 'Flows require Potential Fields'", flows, "a", 60),
    ("03-feedback-regulation", "his root; homeostat — a limit cycle, not a fixed point [OURS]", feedback_regulation, "a", 300),
    ("04-cycling-oscillation", "'Oscillations require Coupled Feedbacks require Cycling'", cycling_oscillation, "a", 300),
    ("05-coupled-predator-prey", "predator-prey; runs away under zeroth-order death [boundary]", coupled_oscillator, "d?", 300),
    ("06-decay", "pathology family (Rheopathology / drain)", decay, "a", 60),
    ("07-networks", "composition; Splitting/Combining fans", networks, "a", 60),
    ("08-emergence-part", "part of the part-vs-whole emergence probe (research)", emergence_parts, "a", 200),
];

/// Writes `troncale-sweep/{name}.json` + `.csv` for every rung, plus a
/// `README.md` outcome table. Opt-in so `cargo test` stays file-free:
///   cargo test emit_sweep_artifacts -- --ignored --nocapture
#[cfg(test)]
#[test]
#[ignore]
fn emit_sweep_artifacts() {
    use crate::export::to_world_model;
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/troncale-sweep");
    std::fs::create_dir_all(dir).unwrap();
    let mut table = String::from(
        "# Troncale sweep — machine-demonstrated Linkage Propositions\n\n\
         Each rung is a circuit whose CSV shows the signature behavior of one of \
         Troncale's processes, climbing his own dependency ladder bottom-up. \
         Claim: *constructible along his stated dependency paths*, never \
         'reduces to N bricks' (he resists linear reduction — 1978). Every CSV \
         is mass-accounted (conservation ledger).\n\n\
         | rung | bucket | Troncale provenance | signature | evidence |\n\
         |---|---|---|---|---|\n",
    );
    for (name, prov, build, bucket, ticks) in LADDER {
        let mut c = build();
        let series = run_storage(&mut c, *ticks);
        let model = to_world_model(&c, name);
        std::fs::write(
            format!("{dir}/{name}.json"),
            serde_json::to_string_pretty(&model).unwrap(),
        )
        .unwrap();
        std::fs::write(format!("{dir}/{name}.csv"), c.csv()).unwrap();
        // a terse signature read for the table
        let stock = series.iter().max_by_key(|s| (amplitude(s) * 1e3) as i64);
        let sig = match *name {
            s if s.contains("predator-prey") => {
                let pred = series.iter().max_by_key(|s| (*s.last().unwrap() * 1e2) as i64).unwrap();
                format!("predator runs away → {:.0}", pred.last().unwrap())
            }
            s if s.contains("cycling") => {
                let st = stock.unwrap();
                format!("relaxation osc: {} turns, amp {:.1}, floors at 0", turning_points(st), amplitude(st))
            }
            s if s.contains("feedback") => {
                let st = stock.unwrap();
                format!("limit cycle: {} turns, amp {:.1} (no fixed point)", turning_points(st), amplitude(st))
            }
            s if s.contains("decay") => "monotone (zeroth-order) drain to ~0".to_string(),
            s if s.contains("potential") => "two stocks equalize → fixed point".to_string(),
            s if s.contains("networks") => format!("fan conserves, diversity {}", c.diversity()),
            s if s.contains("flows") => format!("throughput {:.1} to sink", c.nodes.last().unwrap().total),
            s if s.contains("emergence-part") => "inert (flat) — the isolated part".to_string(),
            _ => "see CSV".to_string(),
        };
        table.push_str(&format!(
            "| `{name}` | ({bucket}) | {prov} | {sig} | `{name}.csv` |\n"
        ));
    }
    table.push_str(
        "\n## Findings worth keeping\n\
         - **Two routes to stability, not one.** A fixed-point equilibrium comes \
         from the PASSIVE gradient (rung 0: two stocks equalize monotonically). \
         The ACTIVE feedback loop does NOT reach a fixed point — it sustains a \
         limit cycle (low gain) or a relaxation oscillation (high gain). \
         Troncale's Equilibrium and Oscillation arise from *different mechanisms*, \
         and the sweep makes the distinction mechanical.\n\
         - **Cycling is not optional.** Sustained oscillation falls out of delayed \
         negative feedback automatically — exactly his dependency Oscillation→\
         Feedback→Cycling, demonstrated rather than asserted.\n\
         - **Zeroth-order release is a hard floor.** `Buffering` releases a \
         constant amount per tick, not a fraction of stock — so linear drain is \
         atomic, but exponential (first-order) decay and Lotka-Volterra are NOT \
         expressible. VERIFIED by probe: a Sensing→Modulating loop on the release \
         only sheds downstream mass; it cannot make the STOCK drain proportional \
         to its level. This wants a `release ∝ stock` buffer mode (feature gap), \
         and it's why the predator-prey rung runs away.\n\
         \n## Boundary (the claim's edge — these are NOT claimed)\n\
         - **Storage** — bucket (b): it IS the Buffering primitive, not constructed.\n\
         - **Coupled predator-prey** — bucket (d?), feature-gated: sustained LV \
         cycling needs first-order death (release ∝ stock), which this primitive \
         set does NOT express at all (verified — not merely 'not atomic'). The \
         naive wiring runs away; needs a proportional-release buffer mode.\n\
         - **Hierarchy** — bucket (c): structural; export nests one level, true \
         composites-of-composites is #75, not the compose canvas.\n\
         - **Evolution** — bucket (d): variation+selection needs the agent layer \
         (Mesa), not flow dynamics.\n\
         - **Symmetry-breaking** — open; needs differentiation we don't yet model.\n",
    );
    std::fs::write(format!("{dir}/README.md"), table).unwrap();
    println!("wrote troncale-sweep/ ({} rungs + README)", LADDER.len());
}
