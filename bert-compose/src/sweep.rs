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
//!
//! The circuit builders live in `ladder.rs` (shared with the palette macros);
//! this module is the verification harness over them — signature detectors,
//! the LP tests, and the artifact emitter.

use crate::circuit::Circuit;

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
        let sign = if d > 1e-4 {
            1
        } else if d < -1e-4 {
            -1
        } else {
            0
        };
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ladder::*;

    /// LP: Potential Fields → equalization. Two stocks on a gradient flow
    /// converge with no controller, conserving — passive homeostasis. (a)
    #[test]
    fn sweep_potential_fields_equalize() {
        let mut c = potential_fields();
        let s = run_storage(&mut c, 200);
        let (a, b) = (*s[0].last().unwrap(), *s[1].last().unwrap());
        assert!(
            (a - b).abs() < 0.1,
            "field equalizes the two stocks: {a} vs {b}"
        );
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
        assert!(
            turning_points(stock) >= 6,
            "the active loop hunts — a limit cycle"
        );
        let tail_min = stock[stock.len() / 2..]
            .iter()
            .cloned()
            .fold(f32::MAX, f32::min);
        assert!(
            tail_min > 0.3,
            "low gain: a smooth cycle that never empties (min {tail_min:.2})"
        );
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
        let tail_min = stock[stock.len() / 2..]
            .iter()
            .cloned()
            .fold(f32::MAX, f32::min);
        assert!(
            tail_min < 0.1,
            "relaxation: it overshoots all the way to empty (min {tail_min:.2})"
        );
    }

    /// RESEARCH 1 / BOUNDARY — the naive predator-prey runs away because
    /// Buffering's release is zeroth-order: the predator's constant-rate death
    /// can't balance its growth, so its stock grows without bound while prey
    /// pins. This is the claim's EDGE, recorded as a fact: sustained LV cycling
    /// is not atomic here (needs first-order death — composable, not a
    /// primitive). (d?/boundary)
    #[test]
    fn sweep_coupled_predator_prey_runs_away() {
        let mut c = coupled_predator_prey();
        let s = run_storage(&mut c, 300);
        let prey = &s[1];
        let predator = &s[3];
        assert!(
            amplitude(prey) < 0.5,
            "prey pins (constant source refills it): amp {:.2}",
            amplitude(prey)
        );
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
        use crate::circuit::NodeKind;
        let mut c = networks();
        run_storage(&mut c, 40);
        let sunk: f32 = c
            .nodes
            .iter()
            .filter(|n| n.kind == NodeKind::Sink)
            .map(|n| n.total)
            .sum();
        assert!(sunk > 0.0 && c.balance().abs() < 1e-3, "fan conserves");
        assert!(
            c.diversity() >= 3,
            "source, splitter, sinks are distinct kinds"
        );
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
        let mut part = emergence_part();
        let ps = run_storage(&mut part, 200);
        assert!(amplitude(&ps[0]) < 1e-3, "the isolated buffer is inert");
        assert_eq!(part.diversity(), 1, "one part, one kind");

        let mut whole = feedback_regulation();
        let ws = run_storage(&mut whole, 300);
        assert!(
            amplitude(&ws[2]) > 1.0,
            "the homeostat cycles — dynamics no part has"
        );
        assert!(
            whole.diversity() > 1,
            "wiring raises kind-diversity: {}",
            whole.diversity()
        );
    }

    /// Boundary buckets, recorded as tests so the claim's edge is explicit:
    /// (b) Storage IS Buffering; (c) Hierarchy is structural (one-level only
    /// via export, true nesting is #75); (d) Evolution needs the agent layer.
    #[test]
    fn sweep_boundary_is_explicit() {
        // (b) the primitive itself holds state — not "constructed," atomic.
        // `emergence_part` is a lone unwired Buffer with stock 8, release 0.
        let mut c = emergence_part();
        run_storage(&mut c, 5);
        assert_eq!(
            c.nodes[0].storage, 8.0,
            "Storage = Buffering holds, bucket (b)"
        );
        // (c)/(d) are assertions about scope, documented in the artifact table.
    }
}

// ── artifact emitter (opt-in: writes the evidence bundle) ────────────────

/// Writes `troncale-sweep/{slug}.json` + `.csv` for every ladder rung, plus a
/// `README.md` outcome table. Opt-in so `cargo test` stays file-free:
///   cargo test emit_sweep_artifacts -- --ignored --nocapture
#[cfg(test)]
#[test]
#[ignore]
fn emit_sweep_artifacts() {
    use crate::export::to_world_model;
    use crate::ladder::LADDER;
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/troncale-sweep");
    std::fs::create_dir_all(dir).unwrap();
    let mut table = String::from(
        "# Troncale sweep — machine-demonstrated Linkage Propositions\n\n\
         Each rung is a circuit whose CSV shows the signature behavior of one of \
         Troncale's processes, climbing his own dependency ladder bottom-up. \
         Claim: *constructible along his stated dependency paths*, never \
         'reduces to N bricks' (he resists linear reduction — 1978). Every CSV \
         is mass-accounted (conservation ledger). Builders: `src/ladder.rs` \
         (shared with the palette macros); verification: `src/sweep.rs`.\n\n\
         | rung | bucket | Troncale provenance | signature | evidence |\n\
         |---|---|---|---|---|\n",
    );
    for rung in LADDER {
        let mut c = (rung.build)();
        let series = run_storage(&mut c, rung.ticks);
        let model = to_world_model(&c, rung.slug);
        std::fs::write(
            format!("{dir}/{}.json", rung.slug),
            serde_json::to_string_pretty(&model).unwrap(),
        )
        .unwrap();
        std::fs::write(format!("{dir}/{}.csv", rung.slug), c.csv()).unwrap();
        // a terse signature read for the table
        let stock = series.iter().max_by_key(|s| (amplitude(s) * 1e3) as i64);
        let sig = match rung.slug {
            s if s.contains("predator-prey") => {
                let pred = series
                    .iter()
                    .max_by_key(|s| (*s.last().unwrap() * 1e2) as i64)
                    .unwrap();
                format!("predator runs away → {:.0}", pred.last().unwrap())
            }
            s if s.contains("cycling") => {
                let st = stock.unwrap();
                format!(
                    "relaxation osc: {} turns, amp {:.1}, floors at 0",
                    turning_points(st),
                    amplitude(st)
                )
            }
            s if s.contains("feedback") => {
                let st = stock.unwrap();
                format!(
                    "limit cycle: {} turns, amp {:.1} (no fixed point)",
                    turning_points(st),
                    amplitude(st)
                )
            }
            s if s.contains("decay") => "monotone (zeroth-order) drain to ~0".to_string(),
            s if s.contains("potential") => "two stocks equalize → fixed point".to_string(),
            s if s.contains("networks") => format!("fan conserves, diversity {}", c.diversity()),
            s if s.contains("flows") => {
                format!("throughput {:.1} to sink", c.nodes.last().unwrap().total)
            }
            s if s.contains("emergence-part") => "inert (flat) — the isolated part".to_string(),
            _ => "see CSV".to_string(),
        };
        table.push_str(&format!(
            "| `{}` | ({}) | {} | {sig} | `{}.csv` |\n",
            rung.slug, rung.bucket, rung.provenance, rung.slug
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
