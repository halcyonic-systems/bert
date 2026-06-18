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

/// Full peak-to-trough swing over the whole slice (unlike `amplitude`, which
/// reads only the tail) — used to show an orbit damping early→late.
fn stock_span(series: &[f32]) -> f32 {
    let (mut lo, mut hi) = (f32::MAX, f32::MIN);
    for &v in series {
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

    /// MECHANISM — does Sensing→Modulating realize a genuine PRODUCT (the
    /// bilinear predation term βxy), or only a gated-linear term? This is the
    /// crux the predator-prey rung turns on. Build a 2-stock probe with both
    /// stocks PINNED at fixed levels and read the predation flow:
    ///
    ///   prey Buffer(τ) ──► Modulating ──► Sink
    ///                         ▲ control
    ///   predator Buffer ──► Sensing(k) ──┘   (observation tap, non-draining)
    ///
    /// The prey buffer's first-order release is `x/τ` (∝ prey x); the Sensing
    /// gate is `clamp(k·y)` (∝ predator y while unsaturated). Their composition
    /// in the valve is `(x/τ)·clamp(k·y)` — a TRUE product. The first-order
    /// drain (τ) is what makes the prey limb proportional; before it shipped,
    /// the release was zeroth-order (constant) and no x·y was expressible. This
    /// is the lift that turns the runaway rung below into a constructible one.
    #[test]
    fn sweep_sensing_modulating_is_a_product_not_a_sum() {
        use crate::circuit::{Node, NodeKind, Wire};
        use bert_core::ProcessPrimitive::{Buffering, Modulating, Sensing};

        const TAU: f32 = 4.0; // prey first-order drain
        const K: f32 = 0.05; // Sensing gain (gate = clamp(K·y))

        // Predation flow with both stocks held at x (prey) and y (predator).
        let flow = |x: f32, y: f32| -> f32 {
            let mut c = Circuit::default();
            let node = |kind, num, px, py| Node::new(kind, num, egui::pos2(px, py));
            c.nodes.push(node(NodeKind::Process(Buffering), 1, 240.0, 240.0)); // 0 prey
            c.nodes
                .push(node(NodeKind::Process(Modulating), 2, 420.0, 240.0)); // 1 valve
            c.nodes.push(node(NodeKind::Sink, 3, 600.0, 240.0)); // 2 eaten
            c.nodes
                .push(node(NodeKind::Process(Buffering), 4, 240.0, 440.0)); // 3 predator
            c.nodes
                .push(node(NodeKind::Process(Sensing), 5, 420.0, 440.0)); // 4 gauge
            c.nodes[0].time_constant = TAU; // proportional prey release
            c.nodes[4].param = K; // Sensing gain
            c.wires.push(Wire::new(0, 1)); // prey → valve
            c.wires.push(Wire::new(1, 2)); // valve → sink
            c.wires.push(Wire::new(3, 4)); // predator level → Sensing (tap)
            c.wires.push(Wire::new(4, 1)); // Sensing signal → valve gate
                                           // Pin both stocks and let the 2-tick
                                           // signal pipeline fill, then read the flow.
            for _ in 0..6 {
                c.nodes[0].storage = x;
                c.nodes[3].storage = y;
                c.step();
            }
            c.nodes[1].activity
        };

        let near = |a: f32, b: f32| (a - b).abs() <= 0.02 * b.max(1.0);

        // Unsaturated regime (K·y < 1): a clean bilinear (x/τ)·(K·y).
        let base = flow(8.0, 4.0); // (8/4)·(0.05·4) = 0.40
        assert!(near(base, 0.40), "baseline product wrong: {base:.3}");
        assert!(near(flow(16.0, 4.0), 2.0 * base), "∝ prey x (doubling x → 2×)");
        assert!(near(flow(8.0, 8.0), 2.0 * base), "∝ predator y (doubling y → 2×)");
        assert!(near(flow(16.0, 8.0), 4.0 * base), "joint: doubling BOTH → 4× (a product)");

        // The discriminator: a SUM would predict f(16,8) ≈ f(16,4)+f(8,8)−f(8,4)
        // = 1.2, not 4×base = 1.6. The gap proves it is multiplicative.
        let sum_prediction = flow(16.0, 4.0) + flow(8.0, 8.0) - base;
        assert!(
            (flow(16.0, 8.0) - sum_prediction).abs() > 0.3,
            "must be a product, not a sum (got {:.2} vs sum-model {:.2})",
            flow(16.0, 8.0),
            sum_prediction
        );

        // The knee: past saturation (K·y ≥ 1, i.e. y ≥ 20) the gate pins at 1
        // and the flow goes linear-in-x only — doubling y no longer moves it.
        // The full LV must keep predator stock below this to stay bilinear.
        assert!(near(flow(8.0, 40.0), flow(8.0, 80.0)), "saturated gate: y stops mattering");
        assert!(near(flow(8.0, 40.0), 2.0), "saturated flow = x/τ (gate=1)");
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

    /// RESEARCH 1 / LIFTED — predator-prey with FIRST-ORDER death. The runaway
    /// above is the same wiring minus two things now available: a `time_constant`
    /// on the predator (death γy ∝ stock, the lift bert#85 was waiting on) and
    /// back-pressure on the predation valve (uneaten prey stays prey, never
    /// shed). The βxy term is the Sensing→Modulating product verified above.
    /// Result: a DAMPED Lotka-Volterra spiral — prey peaks, predator peaks a
    /// quarter-cycle later, and the orbit winds in toward a fixed point because
    /// trophic inefficiency dissipates. Closed orbits are LV's idealization; a
    /// mass-faithful food web damps. And it conserves every tick — the first
    /// ecological demonstration of the conservation engine. (a, lifted)
    #[test]
    fn sweep_predator_prey_damps_and_conserves() {
        let mut c = predator_prey_first_order();
        let s = run_storage(&mut c, 240);
        let prey = &s[1];
        let predator = &s[4];
        let span = |x: &[f32]| {
            x.iter().cloned().fold(f32::MIN, f32::max) - x.iter().cloned().fold(f32::MAX, f32::min)
        };
        let argmax = |x: &[f32]| {
            x.iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap()
                .0
        };

        // Bounded — the runaway is cured (predator no longer grows unbounded).
        assert!(prey.iter().all(|&v| v < 60.0), "prey bounded");
        assert!(
            predator.iter().all(|&v| v < 30.0),
            "predator bounded (the runaway is cured)"
        );

        // A real oscillation, not a creep to equilibrium: both stocks swing
        // wide over the first cycle.
        assert!(
            span(&prey[..120]) > 15.0,
            "prey oscillates: span {:.1}",
            span(&prey[..120])
        );
        assert!(
            span(&predator[..120]) > 5.0,
            "predator oscillates: span {:.1}",
            span(&predator[..120])
        );

        // Predator lags prey — the LV phase signature (it chases abundance).
        assert!(
            argmax(prey) < argmax(predator),
            "predator peak ({}) lags prey peak ({})",
            argmax(predator),
            argmax(prey)
        );

        // DAMPED, not a closed orbit: the spiral winds in — late swing ≪ early.
        assert!(
            span(&prey[120..]) < 0.5 * span(&prey[..120]),
            "orbit damps toward a fixed point (early {:.1} → late {:.1})",
            span(&prey[..120]),
            span(&prey[120..])
        );

        // The prize: a CONSERVATIVE Lotka-Volterra. Every tick's ledger balances.
        let scale =
            (c.emitted + c.nodes.iter().map(|n| n.initial_storage).sum::<f32>()).max(1.0);
        assert!(
            c.balance().abs() <= 1e-3 * scale,
            "conserves: residual {:.5}",
            c.balance()
        );
    }

    /// Sweep `k` (Sensing predation gain) × `η` (Propelling efficiency) through
    /// a predator-prey builder, classifying each cell's orbit and confirming
    /// conservation. `eta_node`/`k_node` are the node indices of the two knobs;
    /// `prey`/`pred` the stock indices to read. Returns the worst relative
    /// ledger residual over the grid (the engine must balance for every cell).
    fn basin_grid(
        label: &str,
        build: fn() -> Circuit,
        eta_node: usize,
        k_node: usize,
        prey_i: usize,
        pred_i: usize,
        ticks: usize,
    ) -> f32 {
        const KS: &[f32] = &[0.02, 0.04, 0.06, 0.08, 0.10];
        const ETAS: &[f32] = &[0.3, 0.5, 0.7, 0.9];
        let peaks = |s: &[f32]| -> Vec<usize> {
            (1..s.len() - 1)
                .filter(|&i| s[i] > s[i - 1] && s[i] >= s[i + 1])
                .collect()
        };

        println!("\n  {label}  ({ticks} ticks)");
        println!("  cell = <class> p<period> [×<damping/cycle> if damping]");
        print!("   k ╲ η │");
        for &eta in ETAS {
            print!("   η={eta:<3.1}        ");
        }
        println!();
        println!("  ───────┼{}", "─".repeat(15 * ETAS.len()));

        let mut worst_residual = 0.0f32;
        for &k in KS {
            print!("  k={k:<4.2} │");
            for &eta in ETAS {
                let mut c = build();
                c.nodes[eta_node].param = eta;
                c.nodes[k_node].param = k;
                let s = run_storage(&mut c, ticks);
                let prey = &s[prey_i];
                let pred = &s[pred_i];

                let scale =
                    (c.emitted + c.nodes.iter().map(|n| n.initial_storage).sum::<f32>()).max(1.0);
                worst_residual = worst_residual.max(c.balance().abs() / scale);

                let pred_max = pred.iter().cloned().fold(f32::MIN, f32::max);
                let cell = if *pred.last().unwrap() < 0.1 {
                    "extinct".to_string()
                } else if pred_max > 100.0 {
                    "RUNAWAY".to_string()
                } else {
                    let pk = peaks(prey);
                    let period = if pk.len() >= 2 {
                        (pk[pk.len() - 1] - pk[0]) as f32 / (pk.len() - 1) as f32
                    } else {
                        0.0
                    };
                    // Late-quarter swing: large & steady ⇒ limit cycle (RINGS);
                    // small ⇒ settled to a fixed point.
                    let late = stock_span(&prey[3 * ticks / 4..]);
                    if late < 0.5 {
                        format!("settles p{period:.0}")
                    } else if late >= 3.0 {
                        format!("RINGS p{period:.0} a{late:.0}")
                    } else {
                        // genuinely damping — report the per-cycle ratio
                        let final_prey: f32 = prey[ticks - 30..].iter().sum::<f32>() / 30.0;
                        let over: Vec<f32> = pk
                            .iter()
                            .map(|&i| prey[i] - final_prey)
                            .filter(|&o| o > 0.05)
                            .collect();
                        if over.len() >= 2 && over[1] > 0.05 {
                            format!("damps p{period:.0} ×{:.0}", over[0] / over[1])
                        } else {
                            format!("damps p{period:.0}")
                        }
                    }
                };
                print!(" {cell:<14}");
            }
            println!();
        }
        worst_residual
    }

    /// RESEARCH — predator-prey parameter BASIN, both prey-growth models side by
    /// side. Constant-immigration prey (`predator_prey_first_order`) vs.
    /// autocatalytic αx prey (`predator_prey_alpha_growth`). Each cell runs the
    /// REAL engine; conservation is asserted across BOTH grids. Opt-in:
    ///   cargo test sweep_predator_prey_basin -- --ignored --nocapture
    #[test]
    #[ignore]
    fn sweep_predator_prey_basin() {
        // Constant immigration: η = node 3, k = node 6, prey = 1, predator = 4.
        let r1 = basin_grid(
            "CONSTANT-IMMIGRATION prey (dx = S − βxy) — stabilizing",
            predator_prey_first_order,
            3,
            6,
            1,
            4,
            800,
        );
        // Autocatalytic αx: η = node 5, k = node 8, prey = 2, predator = 6.
        let r2 = basin_grid(
            "AUTOCATALYTIC αx prey (dx = αx − βxy) — Rosenzweig-MacArthur",
            predator_prey_alpha_growth,
            5,
            8,
            2,
            6,
            1500,
        );
        let worst = r1.max(r2);
        println!("\n  conservation: worst ledger residual across both grids = {worst:.2e} (relative)");
        assert!(worst < 1e-3, "conservation must hold across the whole basin");
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
            s if s.contains("first-order") => {
                let prey = &series[1];
                let early = stock_span(&prey[..prey.len() / 2]);
                let late = stock_span(&prey[prey.len() / 2..]);
                format!("damped LV spiral: prey swing {early:.0} → {late:.0}, conserves")
            }
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
         - **The first-order boundary lifted.** Earlier this rung ran away: \
         `Buffering`'s release was zeroth-order (a constant amount per tick), so \
         the predator's constant-rate death could not balance a growing inflow. \
         The `time_constant` τ parameter (release ≈ stock/τ) makes release \
         proportional to stock — first-order decay and Lotka-Volterra are now \
         expressible. The predation term βxy is realized as a genuine PRODUCT: \
         the prey's first-order release (∝ prey) gated by a Sensing read of the \
         predator level (∝ predator), `(x/τ)·clamp(k·y)` — VERIFIED in \
         `sweep_sensing_modulating_is_a_product_not_a_sum`. A back-pressured \
         valve keeps uneaten prey in the stock rather than shedding it, so the \
         loop stays conservative. The result (rung `05b`) is a DAMPED LV spiral, \
         not a closed orbit: trophic inefficiency dissipates at each transfer, so \
         the orbit winds toward a fixed point. Closed orbits are LV's \
         idealization; a mass-faithful food web damps. The first ecological \
         demonstration of the conservation engine.\n\
         \n## Boundary (the claim's edge — these are NOT claimed)\n\
         - **Storage** — bucket (b): it IS the Buffering primitive, not constructed.\n\
         - **Coupled predator-prey (zeroth-order)** — bucket (d?), kept as the \
         before-picture: the naive wiring runs away because constant-rate death \
         can't balance a growing inflow. Its first-order resolution (rung `05b`, \
         bucket (a)) sits beside it — the boundary the sweep flagged, now lifted.\n\
         - **Hierarchy** — bucket (c): structural; export nests one level, true \
         composites-of-composites is #75, not the compose canvas.\n\
         - **Evolution** — bucket (d): variation+selection needs the agent layer \
         (Mesa), not flow dynamics.\n\
         - **Symmetry-breaking** — open; needs differentiation we don't yet model.\n",
    );
    std::fs::write(format!("{dir}/README.md"), table).unwrap();
    println!("wrote troncale-sweep/ ({} rungs + README)", LADDER.len());
}
