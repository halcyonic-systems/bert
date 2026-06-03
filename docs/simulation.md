# Simulation

*How BERT goes from a static model to running dynamics.*

**Status**: Operational (v0.4.0+). Mesa bridge via subprocess runs any BERT model. Process primitive dispatch, multi-timescale stepping, history conditioning, conservation enforcement, and perturbation injection are all live. Dashboard renders flow and system timeseries. Matrix construction and eigenvalue analysis are planned.

**Ties to**: [process-primitives.md](process-primitives.md) (T-function definitions), [simulation-linalg-bridge.md](simulation-linalg-bridge.md) (formal linear algebra mapping), [lifecycle-dynamics.md](lifecycle-dynamics.md) (phase transitions over long runs), [system-language-spec.md](system-language-spec.md) §4 (execution mapping spec)

---

## Architecture Overview

BERT's simulation turns a structural model (the 8-tuple `S = <C, N, E, G, B, T, H, dt>`) into running dynamics by mapping each element to an executable counterpart:

| 8-tuple element | Static role | Simulation role |
|---|---|---|
| **C** (components) | Set of subsystems | Mesa agents; each contributes dimensions to the state vector v(t) |
| **N** (internal network) | Coupling graph | Flow wiring between agents; determines which state dimensions influence which |
| **T** (transformation) | What each subsystem does | Process primitive dispatch: `PRIMITIVE_T[name](agent, state, in, out)` |
| **H** (history) | What the system has learned | Rolling window of past state snapshots; conditions T via `_condition_T()` |
| **dt** (timescale) | Temporal resolution | `step_interval` per agent; `should_step(tick)` gate |
| **B** (boundary) | Membrane properties | Porosity, interfaces; advisory in v1 (not computationally enforced) |
| **G** (external flows) | I/O channels | Source agent flows with perturbation injection |
| **E** (environment) | External objects | External entities become flow sources/sinks without agent instantiation |

The fundamental distinction: S defines the state space, v(t) is a point moving through it, T moves the point, H records where it has been.

---

## Execution Pipeline

Two data paths:

```
Path A (TypeDB):   BERT JSON --> bert-typedb transpiler --> TypeDB graph
                                                               |
                   Tauri spawns mesa_runner.py -------> reads TypeDB -> Mesa agents -> observations -> TypeDB
                                                                                                       |
                   BERT UI polls TypeDB ------------------------------------------> dashboard <--------+

Path B (JSON-direct):  BERT JSON --> json_bridge.py --> DataFrames
                                                            |
                   Tauri spawns mesa_runner.py --json-path -> Mesa agents -> temp files
                                                                                |
                   BERT UI polls /tmp/{run_id}_status.json ------> dashboard <--+
```

**Path A** (TypeDB) is the full pipeline: transpile the model to a typed graph, simulate against it, write observations back, query results with TypeQL joins. Requires TypeDB running.

**Path B** (JSON-direct) skips TypeDB entirely. The `json_bridge.py` reader parses the BERT WorldModel JSON into DataFrames matching the TypeDB schema shape. Results write to temp files. This is the default path in the current UI.

Both paths produce identical `SimulationResults` (flow timeseries + system timeseries).

### Subprocess Bridge

The Tauri backend (`src-tauri/src/simulation.rs`) spawns `mesa_runner.py` as a child process:

```
python/venv/bin/python3 python/mesa_runner.py \
    --seed 42 --steps 200 --run-id {uuid} \
    --json-path {resolved_path} \
    [--params '{"F0.1": 5.0}']
```

The runner emits JSON progress lines to stdout. The Tauri frontend polls for completion via temp file status (JSON path) or TypeDB query (TypeDB path). On completion, results are read and rendered.

---

## How to Launch

1. **Open a model** in BERT (any `.json` model with subsystems and flows).
2. Click **Simulate** in the toolbar. The SimPanel opens.
3. **Configure parameters**:
   - **Steps**: number of simulation ticks (default 200)
   - **Seed**: random seed for reproducibility (default 42)
   - **Flow overrides**: adjust individual flow amounts via the Inputs panel
   - **Perturbations**: schedule external shock multipliers at specific steps (CLI only: `--perturbation STEP:MULTIPLIER`)
4. Click **Run**. The panel shows a progress indicator as the subprocess runs.
5. On completion, results render as charts in the dashboard.

**Requirements**: Running from source (`cargo tauri dev`) with the Python venv at `python/venv/` set up. See [BUILD.md](BUILD.md) for setup.

---

## The State Vector v(t)

The state vector is a column of numbers, one per internal variable per subsystem. Which variables a subsystem contributes depends on its process primitive:

| Primitive | Variable(s) in v(t) | Persistent across ticks? |
|---|---|---|
| Buffering | `storage`, `activity` | YES — storage accumulates |
| Combining | `activity` (= sum of inflows) | NO — recomputed |
| Splitting | `activity` (= total input) | NO — recomputed |
| Propelling | `activity` (= input × efficiency) | NO — recomputed |
| Impeding | `activity`, `back_pressure` | NO — recomputed |
| Sensing | `signal`, `activity` | NO — recomputed |
| Modulating | `activity`, `control_signal` | NO — recomputed |
| Inverting | `activity` | NO — recomputed |
| Copying | `activity` | NO — recomputed |

All agents also carry `throughput` (sum of incoming flow amounts) and `conservation_deficit` (post-step enforcement residual).

Buffering is the only primitive with persistent state. Its storage level carries across ticks — it IS the H dimension at the primitive level. All other primitives are Markovian: they recompute from current inputs each step.

---

## Process Primitives and Their T-Functions

The 9 process primitives (plus Inverting from Ch. 4 = 10 total) are the atomic operations that define what each leaf-level subsystem *does*. The primitive IS the executable specification: if you know a subsystem is Buffering, you know its T-function accumulates.

### T-Function Summary

| Primitive | T-function | Matrix form | Nature |
|---|---|---|---|
| **Buffering** | `s(t+1) = s(t) + in - out` | `[1]*s + (in-out)` | Affine (constant term) |
| **Combining** | `activity = sum(E/M inflows)` | `[1 1 1 ...]*inflows` | Linear (weighted sum) |
| **Splitting** | `out_i = total / n` | `(1/n)*a` | Linear, conserves M/E |
| **Propelling** | `out = in * efficiency` | `[η]*in` | Linear (scalar multiply) |
| **Impeding** | `out = in * (1 - impedance)` | `[1-r]*in` | Linear (back-pressure) |
| **Sensing** | `signal = k * physical_in` | `[k]*in` | Linear (transduction: E/M → Message) |
| **Modulating** | `out = primary * f(control)` | `diag multiply` | Bilinear (two inputs, nonlinear) |
| **Inverting** | `out = max - in` | `[-1]*in + max` | Affine (complement) |
| **Copying** | `out_i = in` (each output) | `[1;1;1;...]*in` | Linear (fan-out, non-conserving) |

Key properties:
- **Buffering** and **Inverting** are affine, not linear — they have constant terms.
- **Modulating** is bilinear — product of two inputs. This is where the system becomes genuinely nonlinear.
- **Splitting** conserves (valid for Material/Energy). **Copying** replicates (valid for Message only). Substance type constrains which T-functions are legal.

### Dispatch Mechanism

`agents.py` implements each T-function as a standalone function registered in the `PRIMITIVE_T` dispatch dict:

```python
PRIMITIVE_T = {
    "Buffering":  _t_buffering,
    "Combining":  _t_combining,
    "Splitting":  _t_splitting,
    "Propelling": _t_propelling,
    "Impeding":   _t_impeding,
    "Sensing":    _t_sensing,
    "Modulating": _t_modulating,
    "Inverting":  _t_inverting,
    "Copying":    _t_copying,
}
```

When an agent has primitives assigned, `_act_by_primitive()` dispatches each in sequence. Composite subsystems (e.g., Mining = Combining + Propelling) execute their primitives as function composition: `T_Mining = T_Propelling ∘ T_Combining`.

When no primitives are assigned, the agent falls back to archetype-specific `_act()` logic (Economy, Governance, Agent, Passive).

### Substance Type Enforcement

| Primitive | Energy | Material | Message |
|---|---|---|---|
| Buffering | yes | yes | yes |
| Combining | yes | yes | — |
| Splitting | yes | yes | — |
| Propelling | yes | yes | yes |
| Impeding | yes | yes | yes |
| Sensing | yes (in) | yes (in) | yes (out only) |
| Modulating | yes (primary) | yes (primary) | yes (control) |
| Inverting | — | — | yes |
| Copying | — | — | yes |

For the full primitive definitions and composition patterns, see [process-primitives.md](process-primitives.md).

---

## Agent Step Cycle

Every simulation tick, each agent executes a 7-phase step:

```
1. should_step(tick)         Gate: skip if tick % step_interval != 0
2. _process_inputs()         Accumulate incoming flow amounts into throughput
3. _apply_forces()           Force interactions modulate agency_capacity
4. _condition_T()            Read H (history) to set conditioning parameters
5. _act_by_primitive()       Dispatch T-function per primitive (or archetype fallback)
6. _produce_outputs()        Write activity to outgoing flow amounts (with capacity clamping)
7. _enforce_conservation()   Clamp M/E outflows to inflow budget; Message exempt
```

Then `_record_history()` appends the current state snapshot to the rolling window (maxlen=100).

### Force Interactions

Forces shape behavior without substance transfer. A Governance subsystem constraining an Economy subsystem via Force means the governance agent's output activity modulates the economy agent's `agency_capacity`:

```python
pos_factor = 0.5 + 0.5 * min(pos_signal, 2.0)
neg_factor = 1.5 - 0.5 * min(neg_signal, 2.0)
agency_capacity = base_capacity * pos_factor * neg_factor
```

Positive forces amplify capability. Negative forces dampen it. This is Mobus's Mechanism 5 (hierarchical governance) — negative feedback via parameter injection.

### History Conditioning (the H Element)

After two or more history snapshots exist, `_condition_T()` modifies the transformation before dispatch:

| Agent kind | H usage | Effect |
|---|---|---|
| **Reactive** | None (stateless) | Pure T(input), no memory |
| **Anticipatory** | Predicts activity trend via exponential smoothing | `prediction_factor` scales `agency_capacity` (0.7–1.3 range) |
| **Intentional** | Tracks goal-relative performance | `effort_factor` adjusts capacity: below-mean performance increases effort |
| **Buffering** (any kind) | Tracks storage trend | `_release_factor` adjusts outflow rate (0.5–1.5 range) |

In linear algebra terms: stateless agents apply `v(t+1) = T·v(t) + u(t)` with the same T every step. History-conditioned agents apply `v(t+1) = T(H(t))·v(t) + u(t)` — the system is nonlinear even if each individual T is linear, because the choice of T depends on the trajectory.

For the full H-element theory, see [h-element-theory.md](h-element-theory.md).

---

## Multi-Timescale Stepping

Each agent's `time_constant` maps to a step interval:

| Time constant | Ticks per step |
|---|---|
| Millisecond, Second | 1 |
| Minute | 60 |
| Hour | 3,600 |
| Day | 86,400 |
| Week | 604,800 |
| Month | 2,592,000 |
| Year | 31,536,000 |

On each tick, `should_step(tick)` returns True only if `tick % step_interval == 0`. This implements Mobus's hierarchical dt: subsystems with longer time constants step less frequently.

See [simulation-linalg-bridge.md](simulation-linalg-bridge.md) for eigenvalue analysis and the seven mechanisms for boundedness.

---

## Archetype Agent Classes

When process primitives are not assigned, behavior falls back to archetype-specific classes:

| Archetype | Class | Behavior |
|---|---|---|
| **Economy** | `EconomySystem` | Reads Energy/Material flows, maintains `resource_level`, scales by efficiency, decays at 0.95/step |
| **Governance** | `GovernanceSystem` | Reads Message flows, maintains `consensus` (0–1), `rule_strength` = consensus × agency_capacity |
| **Agent** | `AgentSystem` | Autonomous. Kind-dependent: Reactive (signal tracking), Anticipatory (prediction smoothing), Intentional (goal-seeking) |
| **Unspecified** | `PassiveSystem` | Pass-through relay. `activity = throughput`. No autonomous behavior |

The root system (level 0) is not instantiated as an agent — it is the Model-level container.

---

## Conservation and Boundedness

### Post-Step Conservation

`_enforce_conservation()` runs after every step. Material and Energy outflows are clamped to the inflow budget (including current storage for Buffering agents). Message flows are exempt — information replicates without conservation constraints.

```python
inflow_budget = me_inflow + storage
if me_outflow > inflow_budget:
    ratio = inflow_budget / me_outflow
    for flow in outgoing:  # M/E only
        flow["amount"] *= ratio
```

### Seven Mechanisms for Boundedness

| # | Mechanism | Implementation status |
|---|---|---|
| 1 | **Conservation** (1st/2nd Laws) | Implemented — `_enforce_conservation()` clamps M/E outflows |
| 2 | **Lawful state space** (Bunge) | Not implemented |
| 3 | **Boundary interfaces** (capacity-limited) | Not implemented (interfaces are pass-through) |
| 4 | **Edge capacity** (flow throughput ceiling) | Implemented v0.4.0 — `capacity` field, clamped in `_produce_outputs()` |
| 5 | **Hierarchical governance** (Force interactions) | Implemented v0.4.0 — `force_polarity`, positive/negative modulation |
| 6 | **Multi-timescale damping** | Partially — `should_step()` supports different intervals |
| 7 | **Environmental selection** | Not implemented (evolutionary timescale) |

---

## Dashboard

Simulation results render in the SimPanel:

- **Flow timeseries**: line charts of flow amounts over ticks, one line per interaction, grouped by sink subsystem
- **System timeseries**: line charts of internal state variables (activity, storage, consensus, etc.) over ticks
- **Agent comparison table**: side-by-side view of agent states at the current tick
- **CSV export**: full results (tick, type, id, name, key, value) via save dialog

The dashboard design target follows the "Grassmann View" — five panels making the state vector, coupling structure, transformation action, trajectory history, and hierarchical block structure directly visible. See [simulation-linalg-bridge.md](simulation-linalg-bridge.md) for the full dashboard specification.

### Observation Schema

| Entity | Fields | Links to |
|---|---|---|
| `simulation_run` | run_id, model_ref, seed, tick_count, run_status | — |
| `flow_observation` | run_id, tick, observed_amount | interaction via `observes_interaction` |
| `system_observation` | run_id, tick, observation_key, observed_value | system via `observes_system` |

Observations are collected every 10 ticks (configurable via `WRITE_INTERVAL`) and at the final step.

---

## Perturbations

External shocks can be injected at specific simulation steps. A perturbation multiplies all external source flow amounts by a scalar:

```
--perturbation 50:2.0    # double external inputs at step 50
--perturbation 100:0.5   # halve external inputs at step 100
```

"External" means flows whose source is not an instantiated agent (i.e., flows from environmental entities). This tests system resilience: how does the model respond when inputs double or drop?

Perturbations are currently CLI-only. The Inputs panel in the UI supports static flow overrides but not time-scheduled perturbations.

---

## Composition and the Block Matrix

Composite subsystems combine multiple primitives. Mining = Combining + Propelling. The composite T dispatches each primitive's T in sequence with internal wiring: `T_Mining = T_Propelling ∘ T_Combining`.

At the parent level, child states combine the same way, and the parent's T is block-structured:

```
T_Parent = | T_Child1    F_1→2      0          |
           | 0           T_Child2    F_2→3     |
           | F_3→1      0           T_Child3   |
```

Diagonal blocks are each child's own transformation. Off-diagonal blocks are coupling matrices derived from the interaction graph N. Zero blocks indicate no direct flow. N (the internal network) IS the sparsity pattern of the parent's T matrix.

Matrix construction is not yet implemented in code. The simulation currently runs flat agent stepping. See [simulation-linalg-bridge.md](simulation-linalg-bridge.md) for the target architecture.

---

## File Map

| File | Role |
|---|---|
| `python/mesa_runner.py` | CLI entry point; orchestrates read, simulate, write |
| `python/model.py` | `BertModel(Model)` — agent creation, flow wiring, substance validation, conservation, stepping |
| `python/agents.py` | `BertAgent(Agent)` + 4 archetype subclasses + 9 T-functions in `PRIMITIVE_T` |
| `python/json_bridge.py` | BERT JSON → DataFrames (no TypeDB dependency) |
| `python/typedb_bridge.py` | TypeDB read/write for systems, interactions, observations |
| `src-tauri/src/simulation.rs` | Tauri commands: `launch_simulation`, `poll_*`, `get_*_results`, `export_simulation_csv` |
| `src/leptos_app/simulation/` | Frontend: `SimPanel`, `LaunchPanel`, `InputsPanel`, `LineChart`, `AgentComparisonTable` |
| `src/leptos_app/simulation/types.rs` | Shared types: `RunInfo`, `LaunchParams`, `SimulationResults`, timeseries structs |

---

## Known Limitations

- **Flat stepping**: all agents step via `shuffle_do("step")` each tick, gated by `should_step()`. True hierarchical block stepping (child iterations within parent step) is not yet implemented.
- **Boundary properties** (porosity, fuzziness) are advisory only — not enforced in simulation.
- **H-element** is a rolling window of scalar snapshots. Structured history (procedural/declarative/episodic per [h-element-theory.md](h-element-theory.md)) is deferred to v2.
- **No eigenvalue analysis** or stability diagnostics.
- **Perturbation scheduling** is CLI-only.
- **Amplifying** (the 10th primitive) has a T-function defined in [process-primitives.md](process-primitives.md) but is absent from the Rust `ProcessPrimitive` enum and Python `PRIMITIVE_T` dispatch.
