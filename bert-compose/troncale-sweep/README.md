# Troncale sweep — machine-demonstrated Linkage Propositions

Each rung is a circuit whose CSV shows the signature behavior of one of Troncale's processes, climbing his own dependency ladder bottom-up. Claim: *constructible along his stated dependency paths*, never 'reduces to N bricks' (he resists linear reduction — 1978). Every CSV is mass-accounted (conservation ledger). Builders: `src/ladder.rs` (shared with the palette macros); verification: `src/sweep.rs`.

| rung | bucket | Troncale provenance | signature | evidence |
|---|---|---|---|---|
| `00-potential-fields` | (a) | his deepest root: 'Flows require Potential Fields'; Mobus Ch.4 (fields are generalized flows) | two stocks equalize → fixed point | `00-potential-fields.csv` |
| `01-flows` | (a) | his root: 'Flows require Potential Fields' | throughput 87.0 to sink | `01-flows.csv` |
| `03-feedback-regulation` | (a) | his root; the homeostat (proven 6/09) | limit cycle: 21 turns, amp 6.7 (no fixed point) | `03-feedback-regulation.csv` |
| `04-cycling-oscillation` | (a) | 'Oscillations require Coupled Feedbacks require Cycling' | relaxation osc: 19 turns, amp 8.0, floors at 0 | `04-cycling-oscillation.csv` |
| `06-decay` | (a) | pathology family (Rheopathology / drain) | monotone (zeroth-order) drain to ~0 | `06-decay.csv` |
| `07-networks` | (a) | composition; Splitting/Combining fans | fan conserves, diversity 3 | `07-networks.csv` |
| `05-coupled-predator-prey` | (d?) | predator-prey; the sweep's sharpest boundary (bert#85) | predator runs away → 413 | `05-coupled-predator-prey.csv` |
| `05b-predator-prey-first-order` | (a) | predator-prey, resolved — first-order drain (τ) lifted bert#85 | damped LV spiral: prey swing 30 → 1, conserves | `05b-predator-prey-first-order.csv` |
| `08-emergence-part` | (a) | part of the part-vs-whole emergence probe | inert (flat) — the isolated part | `08-emergence-part.csv` |

## Findings worth keeping
- **Two routes to stability, not one.** A fixed-point equilibrium comes from the PASSIVE gradient (rung 0: two stocks equalize monotonically). The ACTIVE feedback loop does NOT reach a fixed point — it sustains a limit cycle (low gain) or a relaxation oscillation (high gain). Troncale's Equilibrium and Oscillation arise from *different mechanisms*, and the sweep makes the distinction mechanical.
- **Cycling is not optional.** Sustained oscillation falls out of delayed negative feedback automatically — exactly his dependency Oscillation→Feedback→Cycling, demonstrated rather than asserted.
- **The first-order boundary lifted.** Earlier this rung ran away: `Buffering`'s release was zeroth-order (a constant amount per tick), so the predator's constant-rate death could not balance a growing inflow. The `time_constant` τ parameter (release ≈ stock/τ) makes release proportional to stock — first-order decay and Lotka-Volterra are now expressible. The predation term βxy is realized as a genuine PRODUCT: the prey's first-order release (∝ prey) gated by a Sensing read of the predator level (∝ predator), `(x/τ)·clamp(k·y)` — VERIFIED in `sweep_sensing_modulating_is_a_product_not_a_sum`. A back-pressured valve keeps uneaten prey in the stock rather than shedding it, so the loop stays conservative. The result (rung `05b`) is a DAMPED LV spiral, not a closed orbit: trophic inefficiency dissipates at each transfer, so the orbit winds toward a fixed point. Closed orbits are LV's idealization; a mass-faithful food web damps. The first ecological demonstration of the conservation engine.

## Boundary (the claim's edge — these are NOT claimed)
- **Storage** — bucket (b): it IS the Buffering primitive, not constructed.
- **Coupled predator-prey (zeroth-order)** — bucket (d?), kept as the before-picture: the naive wiring runs away because constant-rate death can't balance a growing inflow. Its first-order resolution (rung `05b`, bucket (a)) sits beside it — the boundary the sweep flagged, now lifted.
- **Hierarchy** — bucket (c): structural; export nests one level, true composites-of-composites is #75, not the compose canvas.
- **Evolution** — bucket (d): variation+selection needs the agent layer (Mesa), not flow dynamics.
- **Symmetry-breaking** — open; needs differentiation we don't yet model.
