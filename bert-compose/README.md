# BERT Compose

**Touch the system.** Drag process primitives onto a canvas, wire them together, and watch matter, energy, and information actually flow — conserved every tick. Save it as an ordinary BERT model.

It's the minimal, tactile form of BERT issue [#75](https://github.com/halcyonic-systems/bert/issues/75)'s creation experience: the bricks are Mobus's atomic work processes (transfer functions ported from BERT's verified `python/agents.py`), the wiring is composition (unconditional, by theorem), the stocks hold state — and there are no error states by construction, because every wiring action produces a valid system.

## Run it

```bash
cargo run --release            # dev
./make-app.sh                  # build + install "BERT Compose.app" (macOS)
```

## What you can do

- **Add primitives** from the left palette (Buffering, Modulating, Sensing, Splitting, …) and wire `◦ → component`.
- **Stamp systems processes** — Troncale's patterns (Feedback, Oscillation, Networks, Potential Fields) drop their *primitive circuit* onto the canvas. They're not new atoms; you watch the process emerge from the bricks.
- **Load examples** organized to mirror halcyonic.systems: **Foundations** (the FLOWS/BONDS/FEEDBACK concepts, neutral lens), then domain-native examples for **Political Economy / Neuromorphics / Protocol Science / Ecology** (each opens in its own lens), then the cross-domain **Universal homeostat**.
- **Run / Step / Reset**, watch live flow on the wires and charts (`egui_plot`).
- **Read the conservation badge** (⚖, by the clock): green means every unit of physical mass is accounted. Hover it for the full ledger.
- **Name substances** in human terms (money, water, votes) over the conserved kinds — built for social scientists, not just engineers.
- **Switch lenses** (🔍) — the same model reads across the Halcyonic domain pillars: Political Economy, Neuromorphics, Protocol Science, Ecology. Same dynamics, four readings.
- **Save / Load** — round-trips through ordinary BERT JSON (drag a `.json` onto the window, too).
- **Ask hal** — sovereign in-app analysis of a run via the local hal stack; nothing leaves your machine.

## What it is (and isn't)

It's a **conservation-faithful systems simulator** on verified primitives, an **on-ramp to BERT** (it emits real BERT JSON), a **teaching instrument** (plain-English cards, relatable substances, lenses), and a **research instrument** — see the [Troncale sweep](troncale-sweep/README.md), which demonstrated his systems processes as machine-checked Linkage Propositions and mapped where primitive composition reaches its edge.

It is **not** an agent simulator (individuated tokens — *this* validator defecting — live one layer up in Mesa / TypeDB), not yet a hierarchy composer (nesting is [#75](https://github.com/halcyonic-systems/bert/issues/75)), and not yet first-order kinetics (proportional decay is [#85](https://github.com/halcyonic-systems/bert/issues/85)).

## Layout

| file | role |
|---|---|
| `src/circuit.rs` | **the engine** — primitives, the step function, the conservation ledger. No UI. |
| `src/ladder.rs` | Troncale's processes as primitive circuits (shared by the palette macros + the sweep) |
| `src/sweep.rs` | the Troncale sweep: signature detectors, LP tests, artifact emitter (`#[cfg(test)]`) |
| `src/export.rs` | the only JSON path — `to_world_model` (save) and `from_world_model` (load) |
| `src/examples.rs` | the on-ramp example library |
| `src/lens.rs` | the four domain lenses + the Systems identity |
| `src/docs.rs` | per-primitive teaching cards |
| `src/ui/*` | one module per panel (top_bar, status_bar, palette, inspector, charts, canvas, hal_window, about) |
| `src/app.rs` | application state + persistence (save, export, the latest-run contract) |

See [ARCHITECTURE.md](ARCHITECTURE.md) for the layers, the conservation contract, the save/load format, and the four-bucket sweep method. The product vision is in [DESIGN-VISION.md](DESIGN-VISION.md). Where it sits among systems-modeling tools (System Dynamics, AlgebraicJulia/CatColab, Monterey Phoenix, SysML, Palantir) is in [POSITIONING.md](POSITIONING.md).
