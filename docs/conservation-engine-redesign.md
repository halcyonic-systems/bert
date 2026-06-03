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

---

## The three additive changes

### A. Synchronous update mode (`update_mode="synchronous"`)
`BertModel.__init__` gains `update_mode: str = "async"`. `BertModel.step()`:
- **async** (default, unchanged): `self.agents.shuffle_do("step")` — existing path.
- **synchronous**: two-phase, order-independent:
  1. **Snapshot** observable state: `self._level_snapshot = {a.bert_id: a.state.get("storage", 0.0) for a in self.agents}` — frozen pre-tick levels for observation flows.
  2. **Compute phase** (order-independent): every agent runs everything *except* writing shared flow amounts — i.e. `_process_inputs → _apply_forces → _condition_T → _act_by_primitive/_act`. Reads of incoming flow `amount` are automatically frozen because no agent has written outputs yet this tick.
  3. **Commit phase**: every agent runs `_produce_outputs → _enforce_conservation → _record_history`.
- Implementation: split `BertAgent.step()` into `compute()` and `commit()`. Keep `step()` as `self.compute(); self.commit()` so the async path is identical. In sync mode `model.step()` calls all `compute()` then all `commit()`.
- *Files:* `model.py` (`step`, `__init__`), `agents.py` (`BertAgent.step/compute/commit`).
- *Conservation property:* a transfer S→I debits S in S's compute (storage -= released, frozen inputs) and credits I from the flow amount S committed **last** tick. Each `released_T` lands in I exactly once at T+1; one tick's mass is "on the wire" and accounted. No double-read, no order dependence → exact.

### B. Observation flows (non-draining level reads)
New flow semantics so a Sensor can read a stock's level without draining it (Mobus:
sensing is "very low power"). A flow with `usability == "Observation"`:
- At model build (`_build_flow_adjacency`, model.py): tag `flow_info["observation"] = True`. The **source** must not treat it as a mass outflow.
- `_t_buffering` demand + `_produce_outputs`: **exclude** observation outflows from `_base_demand` and from mass-writing (skip them in the outflow loop).
- `_t_sensing` (agents.py): for observation incoming flows, read `agent.model._level_snapshot[flow["_source_id"]]` (the frozen pre-tick level) instead of `flow["amount"]`; scale by `agency_capacity`. Regular (non-observation) sensor flows keep current flow-amount behavior — **existing circuits unchanged**.
- *Why a snapshot:* makes the level read order-independent in sync mode (and harmless in async).
- *Files:* `model.py` (`_build_flow_adjacency`, snapshot), `agents.py` (`_t_sensing`, `_t_buffering`, `_produce_outputs`), `schema.tql` + `json_bridge.py` (allow `"Observation"` usability), `model.py` `PRIMITIVE_SUBSTANCE_VALID` (Sensing already takes E/M).

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

### Conservation invariant
`BertModel.total_conserved_mass()` = Σ buffer storage + Σ in-flight transfer-flow `amount` (M/E, exclude observation flows, dedup by `id()`). `test_conservation_closed_loop` (sync): closed S→I→R, drift < 0.1% over 200 ticks.

---

## Execution order (each step keeps the suite green)

1. **Split `step()` → `compute()`/`commit()`**, async path calls both inline. Run suite → expect 48/48 unchanged (pure refactor).
2. **Add `update_mode` + synchronous two-phase** `model.step()` + level snapshot. No model uses it yet → 48/48 unchanged.
3. **Observation flows**: build-time tagging, buffer demand/produce exclusion, `_t_sensing` snapshot read, schema/json_bridge `"Observation"`. No existing flow uses it → 48/48 unchanged. Add a focused `test_observation_nondraining` (buffer level read leaves storage intact).
4. **`test_conservation_closed_loop`** (sync, gated S→I→R via regulated Buffering + observation) → prove < 0.1% drift. This is the gate: do not proceed until exact.
5. **SIR** model + `test_sir_epidemic` (headless: in-code build + mesa_runner on generated JSON).
6. **LV** model + `test_lotka_volterra` (honest; fallback documented).
7. **Docs**: update `process-primitives.md` (synchronous mode, observation flows, the two validated conservative compositions). Regenerate `*-spec.json` + compiled JSON in `assets/models/local/test-primitives/`.
8. **UI pass** (last): load `sir-epidemic.json` / `lotka-volterra.json` in the Tauri app for Sayama screenshots. Sim launch panel must pass `update_mode` through `mesa_runner.py` args → `src-tauri/src/simulation.rs` (add `--update-mode`, default async) + the Leptos launch panel. *Small Rust/UI touch — only the arg passthrough, not the engine.*

## Verify (headless-first)
```
cd bert/python
./venv/bin/python test_primitives.py                  # 48 existing must stay green at every step
./venv/bin/python -m pytest test_primitives.py -k "observation or conservation or sir or lotka" -q
./venv/bin/python mesa_runner.py --json-path ../assets/models/local/test-primitives/sir-epidemic.json --steps 80 --seed 42 --run-id sir --update-mode synchronous
```

## Risk register
- **Sync changes dynamics of any model that opts in** — mitigated: only NEW models use sync; existing stay async. The dynamics question is empirical (re-tune in headless Python before UI).
- **Observation flow as a new usability** must round-trip through json_bridge + TypeDB schema + the generator. Verify the spec→generate→load path emits/keeps `usability:"Observation"`.
- **Two-buffer LV may still not cycle** even with exact conservation (discrete-time predator-prey can damp) — single-buffer oscillator fallback, labeled honestly.
- **`compute()/commit()` split must not change async results** — step 1 is a pure refactor; assert byte-identical suite output before adding sync.
- **mesa_runner / Tauri arg passthrough** is the only Rust/UI touch; keep `--update-mode` defaulted to async so nothing else changes.

## Out of scope (deliberately)
Global engine rewrite; converting existing circuits to sync; outflow-split (unless a model needs it). The mode flag makes those unnecessary for the conservation goal.
```
