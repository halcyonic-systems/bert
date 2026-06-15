# BERT Compose — Architecture

One spine, layers on top. The whole thing exists to make one claim *tactile*: that the common core of systems science is small, behaves the same across domains, and conserves. Everything here serves that.

```
                 ┌─────────────────────────────────────────────┐
   UI (ui/*)     │ top_bar · palette · inspector · canvas ·     │  panels only,
                 │ charts · status_bar · hal_window · about     │  no physics
                 └───────────────────────┬─────────────────────┘
                                         │ reads/writes
   state (app.rs)  ──────────────────────┤  App: selection, run loop, persistence
                                         │
   ┌─────────────────────┬───────────────┴───────────┬───────────────────┐
   │ engine (circuit.rs) │ content (ladder, examples,│ io (export.rs)    │
   │  primitives, step,  │  lens, docs)              │  ↔ BERT JSON      │
   │  conservation ledger│                           │  (save + load)    │
   └─────────────────────┴───────────────────────────┴───────────────────┘
                                         ▲
   verification (sweep.rs, #[cfg(test)]) │  drives ladder, asserts the LPs,
                                         │  emits troncale-sweep/
```

## The engine — `circuit.rs`

A `Circuit` is `nodes: Vec<Node>` + `wires: Vec<Wire>`. One synchronous discrete `step()`: every transfer function reads the *previous* tick's wire amounts and writes the next, so feedback loops are ordinary dynamics with no special cases. Transfer functions are ports of BERT's verified `python/agents.py` (39 tests there).

**Substance trichotomy.** Energy and Material are conserved (split across fanouts, stored, never duplicated); Message is information (copies freely, gates, is manufactured by Inverting — never conserved). Every node has an `out_substance: DeclaredSubstance { name, base, unit }` — a human name over one of the three conserved `base` kinds. Dynamics read `.base` only; the name/unit are presentation and ride into the JSON.

**Flow modes.** A `Wire` is `pushed` (rate = a set parameter) or `gradient` (rate = `conductance · (level_from − level_to)`). Gradient is how Potential Fields enter — a field is a flow *mode*, not a node (Mobus Ch.4: forces/fields are generalized flows). Gradient flows only run from nodes with a *potential* (Sources, stocks); elsewhere they're inert.

### The conservation contract

The engine's central invariant. Per node, per tick:

```
dissipated += physical_in − physical_out − Δstorage
```

which makes the ledger hold **by construction**:

```
emitted + initial stocks == stored + sunk + in-flight + dissipated
```

`balance()` returns the residual; ≈0 means every unit of physical mass is accounted, and any *unintended* leak is a nonzero residual the property tests catch. `dissipated` is not a fudge factor — it's the sum of the **declared** loss channels: Propelling/Impeding friction, Amplifying power draw, Modulating shed, Sensing consumption, substance-mismatch shed, dead ends. Message is never ledgered (information doesn't conserve).

The property tests in `circuit.rs` build hundreds of random circuits and assert `balance() ≈ 0` every tick; they are mutation-tested (reverting any one leak fix makes them fail). This is why a curve out of bert-compose is *evidence*.

## Content layers

- **`ladder.rs`** — Troncale's systems processes as primitive circuits, the single source of truth shared by the palette macros (you *stamp* a process and its bricks appear) and the sweep tests. A `Rung` carries the builder plus its provenance, sweep bucket, and whether it's palette-stampable.
- **`examples.rs`** — the friendly on-ramp library (leaky bucket, thermostat, battery, the universal homeostat…).
- **`lens.rs`** — four domain vocabularies aligned to the halcyonic.systems pillars (Political Economy / Neuromorphics / Protocol Science / Ecology) + the Systems identity, each a 12-name relabeling (plus per-slot glosses) over the primitive slots. A lens is **pure presentation**: it never enters `circuit.rs`, which is asserted by the lens-invariance test (the universal homeostat's CSV is byte-identical under every lens).
- **`examples.rs`** — categorized to mirror the pillars: Foundations (concepts, neutral lens), the four domain examples (each loads in its lens), and the cross-domain Universal homeostat. A leaky bucket is never relabeled with crypto words — lenses apply only where they mean something.
- **`docs.rs`** — per-primitive teaching cards (plain English → math → substance → theory → transfer function).

## IO — `export.rs`

The only path to/from JSON, both directions:
- `to_world_model` — circuit → a BERT `WorldModel` (composite root + atomic subsystems carrying `AgentModel.primitives`; wires → flows; Source/Sink → environment externals).
- `from_world_model` — the inverse; Load and drag-a-`.json` both use it. Compose-only knobs ride in extensible fields (buffer `release_rate` in `cognitive_params`, gradient `conductance` as a flow `Parameter`) so the round-trip is lossless. Non-compose-shaped models error with a reason rather than dropping structure.

A round-trip test asserts behavioral identity (same physics over 30 ticks), not just structural equality.

## Verification — `sweep.rs` (the Troncale sweep)

The empirical instrument. It climbs Troncale's own dependency ladder and, per process, asks: does the signature behavior emerge from a primitive circuit? Each constructible process is a **passing test whose assertion is the demonstration** — "coupled feedback → oscillation" becomes a circuit whose CSV oscillates, proven by his own criteria (sustainability + influence-richness).

Four buckets sort every process and the boundary is the finding:
- **(a) constructible** — emerges from a primitive circuit
- **(b) is-a-primitive** — already atomic (Storage = Buffering)
- **(c) out-of-scope** — structural/relational, not flow-dynamical (Hierarchy → #75)
- **(d) needs-agent-layer** — individuated/selective (Evolution → Mesa)

`cargo test sweep` proves the LPs. `cargo test emit_sweep_artifacts -- --ignored` writes the circuit+CSV evidence bundle + outcome table to `troncale-sweep/`. The sweep's headline findings (two routes to stability; cycling is automatic; zeroth-order release is a hard floor) live in [troncale-sweep/README.md](troncale-sweep/README.md).

## Roadmap (open issues)

| # | what |
|---|---|
| [#82](https://github.com/halcyonic-systems/bert/issues/82) | "signal costs power" mode — messages aren't free, carriers cost Energy |
| [#83](https://github.com/halcyonic-systems/bert/issues/83) | export the four-lens homeostat as a publishable artifact bundle |
| [#84](https://github.com/halcyonic-systems/bert/issues/84) | lenses auto-apply domain substances (suggest / override) |
| [#85](https://github.com/halcyonic-systems/bert/issues/85) | proportional-release buffer mode → unlocks first-order decay + Lotka-Volterra |
| [#86](https://github.com/halcyonic-systems/bert/issues/86) | "Troncale mode" — processes as first-class collapsible composite components (with #75 nesting) |

## Conventions

- The engine has no UI; the UI has no physics; `export.rs` is the only JSON. Keep it that way.
- New behavior goes in `circuit.rs` with a property/regression test; the conservation invariant must keep holding.
- Rebuild the app with `./make-app.sh` after changes, and **kill stale instances first** — `open` re-focuses a running instance and serves the old binary.
