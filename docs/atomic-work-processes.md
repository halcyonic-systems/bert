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
- **`_produce_outputs()` two-path design**: Splitting/Copying write outputs in their T-functions; all others propagate activity to outgoing flows generically. Early return prevents double-writing.
- **Conservation**: `_enforce_conservation()` clamps Energy/Material outflows to inflow budget. Message flows exempt (non-conservative by design).
