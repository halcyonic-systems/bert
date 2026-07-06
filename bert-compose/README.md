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

The teaching showcase is the Ecology example **Predator and prey** — a mass-accounted Lotka-Volterra. You build it from parts (a Source, two stocks, a sensor, a valve), the oscillation *emerges from the wiring* rather than from an equation you type, and the conservation badge stays green every tick. It spirals to a fixed point because trophic transfer dissipates — the honest answer the textbook's closed orbits idealize away — and the autocatalytic-growth variant recovers a sustained, still-conserving limit cycle. Teaching dynamics and conservation in the same breath, where no other stock-and-flow tool can stop a model from silently leaking mass.

It is **not** an agent simulator (individuated tokens — *this* validator defecting — live one layer up in Mesa / TypeDB) and not yet a hierarchy composer (nesting is [#75](https://github.com/halcyonic-systems/bert/issues/75)). First-order kinetics — proportional decay via the `time_constant` mode, which unlocks Lotka-Volterra — now ships ([#85](https://github.com/halcyonic-systems/bert/issues/85), closed).

## The other face — bert-lenses (convergence)

bert-compose and [bert-lenses](../../bert-lenses/) are the two halves of one creation instrument, split along the Mobus 8-tuple ⟨C,N,E,G,B,T,H,Δt⟩. **Lenses builds the structural face from the ground up** — its Klir → Bunge → Mobus ladder accretes C (things), N (relations/bonds), then E/B/G (environment, boundary, typed flows). **Compose is the dynamical face** — it supplies T (transfer functions) and Δt (the tick), and its conserved, recorded run *is* H. The seam is exact: the Mobus rung is the last structural rung and the first runnable shape, so **Run is one more mode transition** (Mobus-structural → Operational), gated like every other rung — and the parameters compose asks for are precisely the slots Mobus leaves parametric ([MOBUS.md](MOBUS.md)). K≅2 says the *dynamical core* is small; systemness is bigger than its dynamical core, and the rest is what lenses builds — compose isn't a physics tool that strayed from the mission, it's one of the two faces.

The two "lens" vocabularies are orthogonal axes over the same kernel: **tradition lenses** in bert-lenses (how much structure the vocabulary commits to) × **domain lenses** here (what the slots are called). They are different operations — mode transitions gate structure and emit loss witnesses; domain skins are pure relabelings — so they stay **distinct types**, never one `Lens` abstraction.

Two honesty clauses (council-audited, 2026-07-06). First, the Operational gate is **stricter than `validate_mode(Mobus)`**: a valid Mobus structure is not yet a well-formed circuit (a component with no outgoing flow, an open boundary with no source/sink terms — the ledger holds by construction *given a well-formed circuit*, not given any valid model). The rung gets its own predicate, `validate_operational`, checking what this engine actually requires — and Klir/Bunge-mode models fail it *by design*, loudly, never via silent partial projection. Second, upgrading also demands a **component → work-process mapping** (which primitive is this authored component?) — a real design problem, not an assumption. Convergence path, phase-gated: (0) headless `validate_operational` + round-trip property tests (WorldModel → circuit → WorldModel, ledger balanced) before any UI crosses the seam; (1) a read-only "check consistency" audit mode in lenses calling this engine (`circuit.rs`, already UI-free) as a crate; (2) Run, landing as lenses' Arc 4 (see its [ROADMAP](../../bert-lenses/ROADMAP.md)).

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

See [ARCHITECTURE.md](ARCHITECTURE.md) for the layers, the conservation contract, the save/load format, and the four-bucket sweep method. The product vision is in [DESIGN-VISION.md](DESIGN-VISION.md). How it's grounded in Mobus's source (the primitives, the conservation = waste heat, the 8-tuple) is in [MOBUS.md](MOBUS.md). Where it sits among systems-modeling tools (System Dynamics, AlgebraicJulia/CatColab, Monterey Phoenix, SysML, Palantir) is in [POSITIONING.md](POSITIONING.md).
