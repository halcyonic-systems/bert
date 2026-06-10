# Troncale sweep — machine-demonstrated Linkage Propositions

Each rung is a circuit whose CSV shows the signature behavior of one of Troncale's processes, climbing his own dependency ladder bottom-up. Claim: *constructible along his stated dependency paths*, never 'reduces to N bricks' (he resists linear reduction — 1978). Every CSV is mass-accounted (conservation ledger).

| rung | bucket | Troncale provenance | signature | evidence |
|---|---|---|---|---|
| `00-potential-fields` | (a) | his deepest root; field = generalized flow (Mobus Ch.4) | two stocks equalize → fixed point | `00-potential-fields.csv` |
| `01-flows` | (a) | his root: 'Flows require Potential Fields' | throughput 87.0 to sink | `01-flows.csv` |
| `03-feedback-regulation` | (a) | his root; homeostat — a limit cycle, not a fixed point [OURS] | limit cycle: 21 turns, amp 6.7 (no fixed point) | `03-feedback-regulation.csv` |
| `04-cycling-oscillation` | (a) | 'Oscillations require Coupled Feedbacks require Cycling' | relaxation osc: 19 turns, amp 8.0, floors at 0 | `04-cycling-oscillation.csv` |
| `05-coupled-predator-prey` | (d?) | predator-prey; runs away under zeroth-order death [boundary] | predator runs away → 413 | `05-coupled-predator-prey.csv` |
| `06-decay` | (a) | pathology family (Rheopathology / drain) | monotone (zeroth-order) drain to ~0 | `06-decay.csv` |
| `07-networks` | (a) | composition; Splitting/Combining fans | fan conserves, diversity 3 | `07-networks.csv` |
| `08-emergence-part` | (a) | part of the part-vs-whole emergence probe (research) | inert (flat) — the isolated part | `08-emergence-part.csv` |

## Findings worth keeping
- **Two routes to stability, not one.** A fixed-point equilibrium comes from the PASSIVE gradient (rung 0: two stocks equalize monotonically). The ACTIVE feedback loop does NOT reach a fixed point — it sustains a limit cycle (low gain) or a relaxation oscillation (high gain). Troncale's Equilibrium and Oscillation arise from *different mechanisms*, and the sweep makes the distinction mechanical.
- **Cycling is not optional.** Sustained oscillation falls out of delayed negative feedback automatically — exactly his dependency Oscillation→Feedback→Cycling, demonstrated rather than asserted.
- **Zeroth-order release is a hard floor.** `Buffering` releases a constant amount per tick, not a fraction of stock — so linear drain is atomic, but exponential (first-order) decay and Lotka-Volterra are NOT expressible. VERIFIED by probe: a Sensing→Modulating loop on the release only sheds downstream mass; it cannot make the STOCK drain proportional to its level. This wants a `release ∝ stock` buffer mode (feature gap), and it's why the predator-prey rung runs away.

## Boundary (the claim's edge — these are NOT claimed)
- **Storage** — bucket (b): it IS the Buffering primitive, not constructed.
- **Coupled predator-prey** — bucket (d?), feature-gated: sustained LV cycling needs first-order death (release ∝ stock), which this primitive set does NOT express at all (verified — not merely 'not atomic'). The naive wiring runs away; needs a proportional-release buffer mode.
- **Hierarchy** — bucket (c): structural; export nests one level, true composites-of-composites is #75, not the compose canvas.
- **Evolution** — bucket (d): variation+selection needs the agent layer (Mesa), not flow dynamics.
- **Symmetry-breaking** — open; needs differentiation we don't yet model.
