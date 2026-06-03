# Atomic Work Processes — Consolidated Reference

*Mobus source definitions, computational extensions, and cross-framework validation.*
*Consolidates: `universal-processes.md`, `universal-processes-mapping.md`, Mobus Ch. 3 §3.5.2.2.4, Ch. 4 §4.3.3.1*

---

## Source Authority

Mobus names **9 atomic work processes** across two figures in Ch. 3. A 10th (Inverting) appears as a functional component in a Ch. 4 composition example. This document treats it as a legitimate 10th primitive (see rationale below).

**Naming**: Mobus has indicated in conversation that he prefers renaming "atomic work processes" to **process primitives** or similar. This document uses both terms interchangeably. The BERT data model uses `ProcessPrimitive`.

**Fig. 3.18** — Four simple work processes (substrate-level: material, energy, or messages):
Combining, Splitting, Impeding, Buffering

**Fig. 3.19** — Five additional atomic processes (signal/pattern-level):
Copying, Propelling, Sensing, Amplifying, Modulating

**Ch. 4 Fig. 4.12** — Composition example using: Sensing, Inverter, Modulator, Combiner
(Inverting appears here as a circuit component, not as a standalone primitive definition)

**The two-tier structure matters:** Fig. 3.18 processes operate on substrate flows (material/energy). Fig. 3.19 processes operate on pattern/signal flows (information). This distinction constrains which primitives are valid for which substance types.

---

## Why Process Primitives Matter

Most systems frameworks stop at structure — components, boundaries, flows, hierarchy. Process primitives answer the question nobody else does: what do the components actually *do*?

**They're the verb layer.** Every other systems formalism gives you nouns (entities, components, subsystems) and connectives (flows, interactions, networks). Process primitives give you verbs — what transformation each leaf-level component performs. Without them, you have a wiring diagram but no semantics.

**They're the recursion stopping condition.** The primitives aren't just a classification scheme — they define when you're *done* decomposing. A subsystem is atomic when its T function is one of these ten. That's a formal criterion, not a judgment call.

**They bridge structure and simulation.** If you know a subsystem is Buffering, you know its T function accumulates. If it's Sensing, it transduces substance types. The primitive *is* the executable specification. No other systems framework provides this — Forrester gives stocks and flows but no primitive vocabulary for transformations; Ostrom gives institutional grammar but no process algebra; Klir's epistemological hierarchy describes systems at different knowledge levels without specifying what the transformations compute.

**On the "too engineering-heavy" critique:** The primitives aren't borrowed from engineering — they're abstractions *above* engineering that happen to also describe it. Buffering shows up in capacitors, pH chemistry, ATP storage, and institutional memory. That's not engineering bias, that's universality. The critique confuses the examples (often engineering) with the concept (domain-independent).

---

## What Makes Them Atomic

> "Atomic components are those subsystems that need no further deconstruction in order to understand the SOI internals. What makes them atomic is that they involve a minimum of inputs and outputs and their transformative work is easily handled as an opaque box." — Mobus Ch. 3

> "All processes transform low-quality material, energy, or messages into high-quality versions of the same. Work processes require the input of high-potential energy to drive the work itself. In doing work, according to the Second Law of Thermodynamics, some of the energy does not accomplish work, but is transformed to a low-potential form — waste heat." — Mobus Ch. 3, Fig. 3.17 caption

These are the **stopping conditions for recursive decomposition**. When you deconstruct a system and reach a subsystem whose transformation is one of these, you stop. The primitive *is* the T function at the leaf level.

---

## Tier 1: Substrate-Level Processes (Fig. 3.18)

### Combining
**Mobus**: "combining two substances into a more complex substance (a product) with the loss of some heat and waste substance"

**T function** *(extension)*:
```
activity = Σ(all_inflows)
waste_heat = binding_energy_cost
```

**Substance constraint**: Material or Energy (conserves with waste). Message combining is Copying + Modulating.

**Cross-domain**: atomic fusion, chemical bonding, protein synthesis, data fusion, business mergers, coalition formation

---

### Splitting
**Mobus**: "splitting a single substance flow into two (or more) products with some loss of energy and substance"

**T function** *(extension)*:
```
each_outflow = inflow × ratio_i
Σ(ratios) = 1  (conservation)
waste = energy_cost_of_separation
```

**Substance constraint**: Material or Energy (must conserve). For Message flows, use Copying instead (replication, not conservation).

**Cross-domain**: nuclear fission, decomposition, chromatography, catabolism, process forking, asset distribution

---

### Impeding
**Mobus**: "impeding a flow or slowing the rate of flow with a consequent back-pressure"

**T function** *(extension)*:
```
outflow = inflow × (1 - impedance_factor)
back_pressure = inflow × impedance_factor
```

**Substance constraint**: Any substance type. Creates back-pressure on upstream.

**Cross-domain**: electrical resistance, semipermeable membranes, blood-brain barrier, throttling, trade barriers, gatekeeping

---

### Buffering
**Mobus**: "buffering a flow, used to smooth out the flow volumes over time"

**Ch. 4 clarification**: "a 'raw' stock being used simply as a buffer and without regulating controls" — **passive element**, not an active work process. The stopping condition for recursion when you hit a reservoir.

**T function** *(extension)*:
```
dStorage/dt = inflow - outflow
outflow = min(demand, available)
```

**Substance constraint**: Any substance type. The stock *is* the history — Buffering is inherently H-dependent.

**Note on active vs passive**: Mobus is explicit that a buffer without regulatory controls is passive. A regulated reservoir (with sensing + modulating) is a *composition* of primitives, not a single Buffering primitive.

**Cross-domain**: capacitors, pH buffers, ATP storage, memory caches, inventory, institutional memory

---

## Tier 2: Signal/Pattern-Level Processes (Fig. 3.19)

### Copying
**Mobus**: "takes a patterned input substance and an un-patterned one, outputting the original input (think of it as a template) and a copy of the pattern in the second output (plus some waste from imprinting the pattern)"

**T function** *(extension)*:
```
for each output: outflow = inflow  (replication, does NOT conserve)
fidelity = 1 - (errors / total_information)
waste = imprinting_energy
```

**Substance constraint**: Message flows only (information replication). Material/Energy flows cannot copy — they must Split (conservation).

**Cross-domain**: stimulated emission, template polymerization, DNA replication, data duplication, franchise models, meme propagation

---

### Propelling
**Mobus**: "work done to push a substance against a gradient, like a pump pushing water through a pipe"

**T function** *(extension)*:
```
outflow = inflow × efficiency
kinetic_output = potential_input × efficiency
waste_heat = potential_input × (1 - efficiency)
```

**Substance constraint**: Any substance type. Requires energy input to drive the push.

**Cross-domain**: rocket propulsion, molecular pumps, muscle contraction, active transport, process execution, investment momentum

---

### Sensing
**Mobus**: "responds to a modulated force or energy flow, where the modulation is a kind of pattern. It outputs a modulated energy flow, usually very low power, which encodes the modulation (or variation over time) in the applied force"

**T function** *(extension)*:
```
signal_out = transduction_function(physical_input)
substance_change: Energy/Material → Message (transduction)
```

**Substance constraint**: Input is Energy or Material; output is Message. This is the **substance-type crossing point** — the only primitive that changes substance type by definition.

**Cross-domain**: photoelectric effect, chemical indicators, sensory receptors, input devices, market research, surveillance

---

### Amplifying
**Mobus**: "adds energy (power) to a weak but modulated energy input to produce a 'copied' modulated high-powered energy flow output"

**T function** *(extension)*:
```
output = input × gain_factor
energy_consumed = (gain_factor - 1) × input_magnitude
```

**Substance constraint**: Energy or Message. Preserves pattern, increases magnitude.

**Cross-domain**: optical amplification, catalytic acceleration, enzymatic catalysis, hormonal cascades, multiplier effects, viral spread

---

### Modulating
**Mobus**: "applying a modulated energy flow (signal) to a substance flow to produce a modulated output substance flow. Note that when the input substance is a force or energy flow, we have the sensor or amplifier effect."

**T function** *(extension)*:
```
output = primary_flow × f(control_signal)
requires TWO input channels: primary + control
```

**Substance constraint**: Primary can be any substance type. Control is Message. This is the **two-channel primitive** — the only one requiring distinct input roles.

**Cross-domain**: AM/FM modulation, enzyme regulation, gene expression, signal processing, interest rate effects, leadership influence

---

### Inverting
**Mobus Ch. 4 Fig. 4.12**: "takes as input a message and does what its name implies; it inverts the sense of the message code (for numerical values it effectively changes the sign of the value)"

Not explicitly named in the Ch. 3 figure captions, but Ch. 4 references "Figs. 3.17, 3.18, and 3.19" for the full set of atomic processes, implying Inverting is in Fig. 3.19's image. Treated here as the **10th process primitive**.

**Rationale for inclusion**: Inverting is not a degenerate case of Modulating. Modulating requires two input channels (primary + control); Inverting has one channel (single-input sign flip). You cannot reduce it to Modulating without fabricating a phantom control signal. More importantly, the canonical error-sensing circuit (Sensing → Inverting → Modulating → Combining) treats it as a distinct component alongside three canonical primitives. Without Inverting, negative feedback becomes positive feedback — the composition breaks.

**T function** *(extension)*:
```
output = max_signal - input  (continuous)
output = NOT(input)          (logical)
```

**Substance constraint**: Message only.

**Cross-domain**: phase reversal, redox reactions, competitive inhibition, NOT gates, short selling, opposition movements

---

## Composition Patterns

Mobus Ch. 3: "Systems are generally speaking compositions of atomic work processes and so accomplish more complex processes."

Ch. 4 Fig. 4.12 demonstrates the canonical composition: **error-sensing circuit** = Sensing + Inverting + Modulating + Combining. This pattern "can represent anything from molecular feedback at the origin of life to a nuclear plant control system."

| Pattern | Composition | Emergent Function |
|---|---|---|
| Feedback control | Sensing → Inverting → Modulating → Combining | Cybernetic regulation |
| Energy cycle | Combining → Amplifying → Splitting | Thermodynamic work |
| Information processing | Sensing → Copying → Modulating | Knowledge construction |
| Self-maintenance | Copying → Combining → Buffering | Autonomous persistence |

---

## Cross-Framework Validation

*(Original analysis from `universal-processes-mapping.md` — not Mobus source material)*

| Framework | Processes Covered | Coverage |
|---|---|---|
| Thermodynamics | 4/10 (Buffering, Impeding, Propelling, Sensing) | 40% |
| Chemistry | 6/10 (Combining, Splitting, Inverting, Impeding, Propelling, Modulating) | 60% |
| Shannon Information Theory | 5/10 (Sensing, Modulating, Copying, Propelling, Impeding) | 50% |
| Wiener Cybernetics | 6/10 (Sensing, Impeding, Propelling, Buffering, Modulating, Combining) | 60% |
| Kauffman Autonomous Agents | 6/10 (Combining, Modulating, Impeding, Copying, Buffering, Sensing+Propelling) | 60% |

Processes uniquely contributed by Mobus's framework: **Amplifying** (energy concentration for work), **Splitting** as a distinct primitive from Combining (not just the reverse), and the full signal-processing tier.

---

## Verification Contract

Each primitive has a diagnostic perturbation test in `python/test_primitives.py` that proves its characteristic transfer function is correctly implemented. These tests are the executable specification — if a primitive's test passes, the T-function is correct in Mobus's sense.

### Individual Primitive Diagnostics

| Primitive | Diagnostic Test | Expected Response | What It Proves |
|---|---|---|---|
| Buffering | Inflow >> demand, observe storage | Storage accumulates, output decoupled from input | Integrator (stateful, temporal lag) |
| Combining | 2x ONE of two equal inputs | 1.5x total (not 2x) | Summation (linear, not passthrough) |
| Splitting | 2x input, check outputs | Each output 2x, sum = input | Conservation (outputs sum to input) |
| Propelling | 2x input | Output 2x at η level | Linear gain with efficiency |
| Impeding | 2x input | activity + back_pressure = input | Energy balance (resistance creates backpressure) |
| Sensing | 2x physical input | 2x Message output | Substance crossing (Energy/Material → Message) |
| Modulating | 2x both inputs | **4x** output (not 2x) | Bilinearity (product of two inputs) |
| Inverting | Increase Message input | Output **decreases** | Affine complement (moves opposite to input) |
| Copying | Message input | Sum of outputs > input | Non-conservation (information replicates) |

**Key invariant**: No two primitives produce the same perturbation response. The response shape IS the primitive's identity.

### Composition Diagnostics

| Composition | Primitives | Expected Behavior | What It Proves |
|---|---|---|---|
| Negative feedback | Sensing → Inverting → Modulating (loop) | Converges to fixed point (~7.50) | Feedback stabilization (the homeostat pattern) |
| Information fanout | Sensing → Copying → 2× Modulating | Both modulators track same signal | One stimulus → parallel control (non-conservative broadcast) |
| Shock absorption | Perturbation → Buffer | Storage absorbs shock, output stays smooth | Temporal decoupling (buffer IS the H dimension) |
| Oscillation (limit cycle) | Buffering → Sensing → Inverting → Modulating (loop) | Sustained bounded oscillation (~11 turning points, amplitude ~7) | Integrator inside a negative-feedback loop → periodic dynamics emerge from composition |

### Validated Composition: Error-Sensing Feedback Circuit

The canonical Mobus Ch. 4 composition is implemented as a loadable BERT model at `assets/models/local/test-primitives/error-sensing-circuit.json`. Four individually Markovian primitives compose into a thermostat:

```
Energy Supply (10.0) → Modulator → Combiner → Regulated Output
                          ↑              ↓
                       Inverter ← Sensor (feedback)
```

- **Sensor** (agency_capacity=0.05): transduces Combiner's Energy output to a Message signal
- **Inverter**: computes error = 1.0 - sensed_value
- **Modulator**: gates supply Energy by error signal (primary × control)
- **Combiner**: sums modulated correction into output

**Behavior**: Converges to 8.0 at steady state. After 2× perturbation at step 100, re-converges to 13.33. Zero standard deviation at both setpoints — perfect regulation from four stateless components.

**Why this matters**: No primitive "knows" about regulation. Sensing transduces, Inverting flips, Modulating scales, Combining sums. Regulation is a property of the *circuit*, not any component. This is Mobus's core claim made executable: systems properties emerge from process composition, not from intelligent parts. All four primitives are confirmed Markovian by `test_markovian_primitives` (Option C design decision, h-element-theory.md §10).

**Test**: `test_error_sensing_circuit` in `python/test_primitives.py` — loads JSON, runs 200 steps with perturbation, asserts convergence and recovery.

**Dynamic regimes by input energy**: At low input (10), smooth convergence. At medium (20), damped oscillation. At high (100), bang-bang limit cycle — the sensor saturates the Inverter's [0, 1] range, causing binary on/off switching. The fix: add a Buffer to the feedback loop for temporal smoothing, or reduce sensor gain.

Adding a Buffer to the loop is itself instructive: at *sufficient* loop gain the buffered negative-feedback loop does not merely smooth — its integration adds the phase lag that turns the converging fixed point into a **sustained limit cycle**. This is the Oscillator composition (`test_composition_oscillator`): Buffering → Sensing → Inverting → Modulating, producing bounded periodic dynamics with no hand-coded oscillator. It is the foundation the Lotka-Volterra / SIR demos (#76) stand on, where oscillation must likewise emerge from composition.

### Validated Composition: Regulated Buffer

Mobus Fig. 4.17 inventory control pattern at `assets/models/local/test-primitives/regulated-buffer.json`. A Buffering primitive wrapped in a feedback regulation circuit:

```
Energy Supply → Valve (Modulating) → Buffer (Buffering) → Regulated Output
                      ↑                       ↓
                   Inverter ← Sensor (feedback on buffer level)
```

**What it proves**: Option C vindicated — you don't need H-conditioned primitives when a circuit around a dumb buffer does the job. The regulation mechanism is visible in the wiring rather than hidden inside the primitive.

**Test**: `test_regulated_buffer` — loads JSON, runs 200 steps with perturbation, asserts storage accumulates and remains stable.

### Validated Composition: Oscillator (Limit Cycle)

Loadable model at `assets/models/local/test-primitives/oscillator.json`. **Same topology as the Regulated Buffer** (Modulating + Buffering + Sensing + Inverting feedback loop) — what differs is the *tuning*:

```
Energy Supply → Modulator → Buffer → Oscillating Output
                    ↑           ↓
                 Inverter ← Sensor (feedback on buffer level)
```

- **Sensor** gain raised (0.05 → 0.2) and **buffer demand** raised (3 → 5) relative to the regulated buffer
- The Buffer's integration adds the phase lag that turns the *converging fixed point* into a *sustained bounded limit cycle*

**Behavior**: sustained oscillation of the buffer's output — ~15 turning points over 80 steps, amplitude ~8.4 — bounded by the Modulating `[0,2]` and Inverting `max(0, ·)` clamps. **The same circuit regulates or oscillates depending only on loop gain and demand** — regime is a property of the tuning, not the parts.

**Why it matters**: this is oscillation-from-composition made loadable and demo-ready. It is the foundation the Lotka-Volterra / SIR demos (#76) stand on, where population/epidemic oscillation must likewise emerge from primitive composition.

**Tests**: `test_composition_oscillator` (builds the loop in-code) and `test_oscillator` (loads `oscillator.json` via `json_bridge` and confirms the generated model reproduces the limit cycle). Spec: `oscillator-spec.json`.

## Update Modes: Async (Regulation) vs Synchronous (Conservation)

The Mesa engine runs in one of two update modes, selected per model by
`BertModel(update_mode=...)` (default `"async"`); thread it through the runner with
`mesa_runner.py --update-mode synchronous`.

- **async** — `agents.shuffle_do("step")`: push-based, read-not-consumed, random order.
  Correct for **signal/regulation circuits** (oscillator, error-sensing): the things
  above all use it, and it is the validated default.
- **synchronous** — two-phase, order-independent: a pre-tick **level snapshot**, then
  every agent's `compute()` (T-functions read flow amounts still frozen at last tick's
  committed values), then every agent's `commit()` (shared flow writes). Required for
  **exact mass conservation** in stock-to-stock transfer. One tick's transferred mass
  sits "on the wire" (in the flow amount) between the source's debit and the sink's
  credit, so `BertModel.total_conserved_mass()` (= Σ storage + Σ in-flight transfer
  amount) is invariant to machine epsilon, not Σ storage alone.

These are genuinely different update semantics, not a flag hack: regulation wants the
shuffled push; conservation wants the synchronized ledger. Existing async circuits are
untouched (byte-identical step split).

**Observation flows** — a Sensor reading a stock's *level* without draining it (Mobus:
sensing is "very low power"). Carried on `interaction.parameters` as `observation:true`
(parsed in `json_bridge`; **not** a new `usability` value — that would break the Rust
serde enum and TypeDB `@values`). An observation tap is excluded from buffer demand,
`_produce_outputs`, and `_enforce_conservation`; `_t_sensing` reads the frozen snapshot
level instead of the flow amount. Test: `test_observation_nondraining`.

**Conservative buffers** — a Buffering agent in synchronous mode emits exactly its
debited release (no capacity clamp), is exempt from `_enforce_conservation` rescaling
(it self-limits via `min(adjusted, storage)`), and holds `release_factor` at 1.0 (no
trend wobble to distort the transfer rate). This makes mass-exactness structural.
Gate: `test_conservation_closed_loop` — a closed S→I→R chain holds total mass exact to
1e-9 over 60 ticks with zero per-tick deficit.

### Validated Composition: SIR Epidemic (Conservative Compartmental)

Loadable model at `assets/models/local/test-primitives/sir-epidemic.json` (spec
`sir-epidemic-spec.json`, compiled via `compile_spec.py`). Synchronous mode.

```
Susceptible ──infection──▶ Infected ──recovery──▶ Recovered
     ▲                        │
  control (Message)      observe (non-draining level read)
     └──── I-Sensor ◀─────────┘
```

The I-Sensor reads the Infected level through an **observation flow** and feeds a
Message control into Susceptible; S's regulated release is the infection transfer S→I
(gated by sensed I, drawn from S), and I releases to R at a constant recovery rate.
Seed storages via `agent.initial_state` (S=100, I=5, R=0).

**What it proves**: an epidemic curve — S monotone down, I a single peak, R monotone up
— emerges entirely from primitive wiring with **mass conserved to 1e-9**, no custom
agent code. A brief two-tick onset transient reflects the synchronous sensor→control→
infection signal path. **Test**: `test_sir_epidemic`.

**Emergent epidemic threshold**: sweeping the infection gain `k` (the I-Sensor gain, a β
proxy) with `python/sweep_sir.py` reveals a sharp R₀=1 bifurcation at `k ≈ 0.0077` — below
it the seed infecteds recover before spreading (attack rate a few %), above it the epidemic
burns through the whole pool (100%). The transition is a <7% change in `k`. Figure:
`docs/sir-threshold.png`. The threshold is *emergent from the composition*, not coded — the
same wiring contains or explodes depending only on one gain crossing a critical value.

### On Two-Buffer Lotka-Volterra (honest result)

A predator-prey pair was attempted as a conservative two-buffer composition (predation =
Prey's release gated by an observation read of Predator level; predator death to a sink).
It does **not** sustain a limit cycle: in a strictly mass-conserving discrete engine the
Buffering primitive supplies no self-proportional prey-growth (αPrey) term, so once
predation crashes the prey it cannot rebound — the system collapses to prey=0 rather than
cycling (consistent with discrete-time predator-prey instability). This is a real finding
about conservative discretization, not a bug. The **canonical sustained-oscillation
exemplar remains the Oscillator limit cycle above** (`test_composition_oscillator` /
`test_oscillator`). Together, SIR (epidemic threshold) and the Oscillator (limit cycle)
give two qualitatively different emergent behaviors from the same primitive toolkit —
the #76 goal.

### Validated Composition: Energy Processing Chain

Mobus Ch. 3 production pipeline at `assets/models/local/test-primitives/energy-chain.json`:

```
Source A → Combiner → Propeller (η=0.7) → Splitter → Output A
Source B →                                          → Output B
```

**What it proves**: Conservation through a chain with thermodynamic loss. Total output < total input by the Propeller's efficiency factor. Perturbation propagates proportionally (2× input → 2× output).

**Test**: `test_energy_chain` — loads JSON, runs 200 steps with perturbation, asserts output increases proportionally.

### Validated Composition: Information Broadcast

Information processing pattern at `assets/models/local/test-primitives/info-broadcast.json`:

```
Physical Stimulus → Sensor → Copier → Modulator A → Output A
                                    → Modulator B → Output B
Independent Energy A → Modulator A (primary)
Independent Energy B → Modulator B (primary)
```

**What it proves**: Information replicates without conservation. One sensed signal controls two independent energy flows simultaneously. Copying is non-conservative (outputs > input) — this is valid for Message substance only.

**Test**: `test_info_broadcast` — loads JSON, runs 50 steps, asserts both modulators have identical activity (synchronized control).

### H-Conditioning by Agent Kind

| Agent Kind | H Usage | Observable | What It Proves |
|---|---|---|---|
| Reactive | None (stateless) | agency_capacity unchanged | Pure T(input) — no memory |
| Anticipatory | Predicts activity trend | prediction_factor ≠ 1.0 after input change | T conditioned by H prediction |
| Intentional | Tracks goal-relative performance | effort_factor adjusts when below mean | T modified to optimize toward goal |

### Substance Type Enforcement

Runtime enforcement in `PRIMITIVE_SUBSTANCE_VALID` (`model.py`). Compile-time enforcement in `SubstanceType` enum (`system_elements.rs`). Schema enforcement in TypeDB (`@values`).

| Primitive | Energy | Material | Message | Rationale |
|---|---|---|---|---|
| Buffering | ✓ | ✓ | ✓ | Stores any substance |
| Combining | ✓ | ✓ | ✗ | Physical confluence (mass/energy conservation) |
| Splitting | ✓ | ✓ | ✗ | Physical distribution (conservation). Message splitting = Copying |
| Propelling | ✓ | ✓ | ✓ | Transport with work (any substance) |
| Impeding | ✓ | ✓ | ✓ | Resistance (any substance) |
| Sensing | ✓ in | ✓ in | ✗ in / ✓ out | Transduction: physical → informational |
| Modulating | ✓ primary | ✓ primary | ✓ control | Two-port: physical primary + Message control |
| Inverting | ✗ | ✗ | ✓ | Informational complement only |
| Copying | ✗ | ✗ | ✓ | Information replicates freely; physical cannot |

## Implementation Notes

- **Primitive type selects T-function**: `PRIMITIVE_T` dict in `agents.py` dispatches `_t_buffering`, `_t_combining`, etc.
- **Composite subsystems**: dispatch each primitive in sequence via `_act_by_primitive()`
- **`_produce_outputs()` two-path design**: Splitting/Copying write outputs in their T-functions; all others propagate activity to outgoing flows generically. Early return prevents double-writing. Observation taps are skipped; conservative buffers (synchronous mode) emit exactly `activity` with no capacity clamp.
- **Conservation**: `_enforce_conservation()` clamps Energy/Material outflows to inflow budget; Message and observation flows exempt. Conservative buffers are exempt entirely (they self-limit and emit their exact debit). Closed-system exactness is asserted via `BertModel.total_conserved_mass()`.
- **Update modes**: `BertAgent.step()` = `compute()` + `commit()`. Async runs them inline per agent (shuffled); synchronous (`BertModel._step_synchronous`) snapshots levels, runs all computes, then all commits. See "Update Modes" above.
