# Conservation Engine Redesign — Execution Scope (full tier)

*Goal: exact mass conservation for compartmental/population models (SIR with real
βSI, two-buffer Lotka-Volterra) without destabilizing the validated regulation
circuits. Sequel to commit `f7edb291` (Amplifying + conservative Modulating +
regulated Buffering).*

## Why / the decision

The push-based, read-not-consumed, random-order (`shuffle_do`) engine is correct
for **signal/regulation circuits** (oscillator, error-sensing) but structurally
cannot conserve mass for **stock-to-stock transfer**. Three verified failure modes:
gate double-read under shuffle (~18% leak), Sensing-via-flow drains the observed
stock, and `_produce_outputs` copies a buffer's release to *every* outflow (mass
creation). Faithful βSI needs all three resolved.

**Architecture decision: don't rewrite the engine — add an opt-in conservative
path.** Async regulation circuits and synchronous compartmental models are two
legitimately different update semantics. Isolate the new behavior behind a model
mode + a new flow type so the 48 existing green tests are untouched.

Python-only. No Rust changes (the Amplifying enum/schema already shipped).

> **Architecture review applied (2026-06-02).** A bert-architecture review found the
> first draft NO-GO and corrected it; this version incorporates the fixes. Full review:
> `halcyonic/operations/sessions/2026-06-02/references/conservation-redesign-arch-review.md`.
> The decisive correction: **observation is carried on a `parameters` flag, NOT a new
> `usability` value** — `usability:"Observation"` hard-fails the Rust serde loader
> (closed enum) and the TypeDB `@values` constraint, and headless Python tests would
> stay green while the model is silently unloadable in the app.

## Step 0 (do before any code) — observation carrier decision
Mark observation taps with `interaction.parameters` carrying `observation: true` (round-trips
through the existing parameters plumbing, `load.rs:173`; renders as a normal flow; zero Rust/
schema change). At Python build (`_build_flow_adjacency`) set `flow_info["observation"] = bool(...)`
on **every** flow so downstream code reads `flow.get("observation", False)` unambiguously. Do NOT
add a `usability` value.

---

## The three additive changes

### A. Synchronous update mode (`update_mode="synchronous"`)
`BertModel.__init__` gains `update_mode: str = "async"`. `BertModel.step()`:
- **async** (default, unchanged): `self.agents.shuffle_do("step")` — existing path.
- **synchronous**: two-phase, order-independent:
  1. **Snapshot** observable state: `self._level_snapshot = {a.bert_id: a.state.get("storage", 0.0) for a in self.agents}` — frozen pre-tick levels for observation flows.
  2. **Compute phase** (order-independent): every agent runs everything *except* writing shared flow amounts — i.e. `_process_inputs → _apply_forces → _condition_T → _act_by_primitive/_act`. Reads of incoming flow `amount` are automatically frozen because no agent has written outputs yet this tick.
  3. **Commit phase**: every agent runs `_produce_outputs → _enforce_conservation → _record_history`.
- Implementation: split `BertAgent.step()` into `compute()` and `commit()`. Keep `step()` as `self.compute(); self.commit()` so the async path is byte-identical (verify; gate step 1). In sync mode `model.step()` calls all `compute()` then all `commit()`, then `_record_system_history` + `datacollector.collect` once (after commit, same as async).
- **Phase membership (review-corrected):** compute = `_process_inputs, _apply_forces, _condition_T, _act_*`; commit = `_produce_outputs, _enforce_conservation, _record_history`. The `should_step`/`step_interval` skip guard MUST gate **both** phases identically (else non-`Second` time-constants desync — the suite won't catch it, all test models are `Second`). Initialize `self._level_snapshot = {}` in `__init__` unconditionally.
- **Forces & self-writing T-funcs (review B2):** forces use the same shared-dict aliasing (written in commit, read in compute) → sync makes forces always 1-tick-delayed. `_t_splitting`/`_t_copying` write `flow["amount"]` *in compute*, violating frozen-reads. **Conservative models must contain zero Force interactions and no Splitting/Copying** — assert this at build; document the delayed-force semantics.
- *Files:* `model.py` (`step`, `__init__`), `agents.py` (`BertAgent.step/compute/commit`).
- *Conservation property:* a transfer S→I debits S in S's compute (storage -= released, frozen inputs) and credits I from the flow amount S committed **last** tick. Each `released_T` lands in I exactly once at T+1; one tick's mass is "on the wire" and accounted. No double-read, no order dependence → exact.

### B. Observation flows (non-draining level reads)
A Sensor reads a stock's level without draining it (Mobus: sensing is "very low power").
Carrier = the `parameters` flag from Step 0, **not** a usability value.
- At model build (`_build_flow_adjacency`): `flow_info["observation"] = bool(params.get("observation"))` on every flow. Source must not treat observation outflows as mass outflows.
- **Three exclusion sites** (review B3 — `_enforce_conservation` is the easily-missed third): exclude observation outflows from (1) `_t_buffering`'s cached `_base_demand` (at capture time, first step), (2) `_produce_outputs` mass-writing loop, and (3) `_enforce_conservation`'s `me_outflow` sum **and** its scaling loop. Missing (3) lets an observation E/M flow inflate `me_outflow`, firing spurious `ratio` scaling on the real transfer → conservation broken.
- `_t_sensing`: for observation incoming flows read `agent.model._level_snapshot.get(flow["_source_id"], 0.0)` (frozen pre-tick level) instead of `flow["amount"]`, scaled by `agency_capacity`; use `.get` (external sources aren't agents). Build-time-warn if an observation source isn't a Buffering agent. Non-observation sensor flows keep current behavior — **existing circuits unchanged** (predicate must be `flow.get("observation", False)`, never `flow["observation"]`).
- *Files:* `model.py` (`_build_flow_adjacency`, `_level_snapshot`, `__init__` init), `agents.py` (`_t_sensing`, `_t_buffering`, `_produce_outputs`, `_enforce_conservation`). **No schema.tql / no json_bridge usability / no Rust change.**

### C. (Defensive, optional) outflow-split for multiple physical outflows
Only needed if a conservative model has a buffer with >1 *mass* outflow. The new
models are wired to avoid this (one transfer outflow + observation taps), so this
is **out of scope unless a model requires it**. If needed: in `_produce_outputs`,
allocate `activity` across Energy/Material outflows (sum ≤ activity, by capacity
weight) instead of copying; Message outflows still copy. Audit async models first
(energy-chain uses explicit Splitting; verify no others rely on duplication).

---

## The models (synchronous, exact conservation)

### SIR — real βSI
- `S, I, R`: Buffering, `update_mode="synchronous"`.
- `I → ISensor` **Observation** flow; `ISensor` (Sensing, agency = β-proxy) → Message control into `S`. S's regulated release = `base_demand · release_factor · clamp(β·I, 0,1)` = the βSI infection transfer, drawn from S, deposited in I. Bilinear (S-limited × I-control), conservative.
- `I → R` transfer at constant γ-rate (I's regulated release; or a second observation+sensor if state-dependent recovery is wanted).
- Seed storages in-harness (S₀=100, I₀=1, R₀=0). `test_sir_epidemic`: S↓ monotone, I unimodal single peak, R↑ monotone, **S+I+R conserved < 0.1%**.

### Lotka-Volterra — two-buffer
- `Prey, Predator`: Buffering, synchronous. Prey growth = regulated inflow; predation = Prey's regulated release controlled by an Observation read of Predator level (βRF); Predator death = regulated release to a sink; Predator gain fed by the predation transfer.
- `test_lotka_volterra`: bounded sustained oscillation (turning points ≥ 6, bounded), prey-leads-predator phase lag. With sync determinism the cycle should be cleaner than async; if two-buffer still won't sustain, the single-buffer oscillator remains the documented fallback (`process-primitives.md:330`).

### Conservative-buffer rules (review B4/B7 — make exactness structural, not emergent)
A buffer participating in conservative transfer must: (1) have **infinite capacity** on its transfer outflow (no `min(activity, capacity)` clamp destroying debited mass), (2) emit **exactly** `state["activity"]` (T-debit and `_produce_outputs`-emit identical), (3) be **exempt from `_enforce_conservation` scaling**, and (4) bypass `_condition_T`'s `release_factor` wobble (which would distort the βSI/βRF rate and could violate "I unimodal"). Implement as a `conservative=True` buffer flag (or derive from `update_mode=="synchronous"`).

### Conservation invariant
`BertModel.total_conserved_mass()` = Σ buffer storage + Σ in-flight transfer-flow `amount` (M/E, **exclude observation flows**, dedup by `id()`). `test_conservation_closed_loop` (sync, **`perturbations={}`** — perturbations inject/remove mass and are incompatible with the invariant): closed S→I→R, assert per-tick `conservation_deficit == 0` on every buffer **and** `abs(total_conserved_mass(t) - total_conserved_mass(0)) < 1e-9`. Exact means epsilon-exact; a 0.1% tolerance is a tell that leaks remain.

---

## Execution order (each step keeps the suite green)

1. **Split `step()` → `compute()`/`commit()`**, async path calls both inline. Run suite → expect 48/48 unchanged (pure refactor).
2. **Add `update_mode` + synchronous two-phase** `model.step()` + level snapshot. No model uses it yet → 48/48 unchanged.
3. **Observation flows** (`parameters` carrier, Step 0): build-time tagging, the **three** exclusion sites (demand, produce, `_enforce_conservation`), `_t_sensing` snapshot read with `.get`. No existing flow has the flag → re-run suite, **48/48 unchanged** (hard gate). Add `test_observation_nondraining` (buffer level read leaves storage intact).
4. **`test_conservation_closed_loop`** (sync, `perturbations={}`, conservative buffers, inf-capacity transfers): assert per-tick `conservation_deficit==0` **and** `< 1e-9` total-mass drift. **Epsilon-exact gate — do not proceed until exact.**
5. **SIR** model + `test_sir_epidemic` (headless: in-code build + mesa_runner). **Thread `--update-mode` through `mesa_runner.run_json` → `BertModel.__init__`** (`mesa_runner.py:111` currently drops it) here, not at step 8.
6. **LV** model + `test_lotka_volterra` (honest; fallback documented).
7. **Docs**: update `process-primitives.md` (synchronous mode, observation flows, the two validated conservative compositions). Regenerate `*-spec.json` + compiled JSON in `assets/models/local/test-primitives/`.
8. **UI pass** (last): load `sir-epidemic.json` / `lotka-volterra.json` in the Tauri app for Sayama screenshots. With the `parameters` carrier the models **load and render normally** (no serde/schema break). Panel passes `update_mode` → `src-tauri/src/simulation.rs` (`--update-mode`, default async) + the Leptos launch panel. *Small Rust/UI touch — arg passthrough only.*

## Verify (headless-first)
```
cd bert/python
./venv/bin/python test_primitives.py                  # 48 existing must stay green at every step
./venv/bin/python -m pytest test_primitives.py -k "observation or conservation or sir or lotka" -q
./venv/bin/python mesa_runner.py --json-path ../assets/models/local/test-primitives/sir-epidemic.json --steps 80 --seed 42 --run-id sir --update-mode synchronous
```

## Risk register
- **Sync changes dynamics of any model that opts in** — mitigated: only NEW models use sync; existing stay async. The dynamics question is empirical (re-tune in headless Python before UI).
- **Observation carrier** = `parameters` flag (resolved): verify the spec→generate→load path preserves `parameters.observation` (it does for `parameters`, `load.rs:173`). Do NOT use a `usability` value — that hard-fails the Rust serde enum + TypeDB `@values`.
- **Two-buffer LV may still not cycle** even with exact conservation (discrete-time predator-prey can damp) — single-buffer oscillator fallback, labeled honestly.
- **`compute()/commit()` split must not change async results** — step 1 is a pure refactor; assert byte-identical suite output before adding sync.
- **mesa_runner / Tauri arg passthrough** is the only Rust/UI touch; keep `--update-mode` defaulted to async so nothing else changes.

## Out of scope (deliberately)
Global engine rewrite; converting existing circuits to sync; outflow-split (unless a model needs it). The mode flag makes those unnecessary for the conservation goal.
```
