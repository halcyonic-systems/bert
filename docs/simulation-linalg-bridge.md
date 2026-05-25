# Simulation ↔ Linear Algebra Bridge

**Status**: Living spec. Primitive T functions (§ Process Primitives as Linear Maps) implemented in v0.4.0. State Panel dashboard shipped. Matrix construction, eigenvalue analysis, hierarchical stepping, and remaining dashboard panels are planned.

**Ties to**: `process-primitives.md` (primitive definitions), `h-element-theory.md` (history conditioning), `system-language-spec.md` §4 (execution mapping)

## Purpose

Every implementation decision in BERT's simulation layer is simultaneously a linear algebra decision. Code written during simulation development should be documented in the vocabulary of both Mobus's systems science AND linear algebra — so that building BERT teaches the math, and learning the math improves BERT.

**Rule**: When writing or reviewing any `_act()`, step logic, or flow computation, name the linear algebra object alongside the systems science object.

---

## The Fundamental Distinction

BERT has two layers, and conflating them is the source of most confusion:

**Layer 1 — The System Definition S (structural, static, what BERT has been for 3 years):**
`S = ⟨C, N, E, G, B, T, H, Δt⟩` is a tuple of sets, relations, functions, and parameters. It specifies *what the system is* — its components, their connections, its boundary, its transformation rule, its memory structure, its timescale. This is the ontological specification. It does not change at each tick. It is not a vector. It is the *blueprint*.

**Layer 2 — The State Vector v(t) (dynamic, changes every tick, what BERT is becoming):**
`v(t) ∈ ℝⁿ` is the instantaneous snapshot of all internal variable values at time t. It IS a vector — a column of numbers, one per internal variable. It moves through the state space as the simulation runs. Mobus does not give it a single letter in the 8-tuple because it is implicit: T maps states to states, H records past states, but "the current state" is not one of the eight named elements.

**The relationship:** S *defines* the state space. v(t) is a *point* in that space. T *moves* the point. H *records* where the point has been.

Three years of structural decomposition have been building S — the container. The simulation fills it with a moving point v(t).

---

## How S Defines the State Space

Each element of the 8-tuple contributes to defining the vector space that v(t) lives in, but none of them IS v(t).

| 8-tuple element | What it is (Mobus) | What it defines for the state space | What it is NOT |
|---|---|---|---|
| **C** (components) | Set of subsystems | **The dimensions.** Each subsystem with internal variables contributes coordinates to v(t). 3 subsystems with 1 variable each → v(t) ∈ ℝ³ | C is not a vector. It is a set of parts whose variables become the coordinate axes |
| **N** (internal network) | Relations between components | **The coupling structure.** Which dimensions of v(t) influence which other dimensions. Determines which entries of the transformation matrix are nonzero | N is not a matrix. It is a graph whose edges tell you where the matrix has nonzero off-diagonal entries |
| **E** (environment) | Objects and milieu outside B | **The input/output interface.** Determines which external vectors u(t) are added to v(t) and which projections y(t) are read from v(t) | E is not part of v(t). It defines what crosses the boundary |
| **G** (external flows) | Flows crossing B | **The I/O channels.** Each external flow adds a dimension to the input vector u(t) or the output vector y(t) | G defines how outside connects to inside |
| **B** (boundary) | Membrane with porosity, interfaces | **Projection and scaling operators.** Porosity scales flow magnitudes. Interfaces are projections from the full state space to the subspace visible externally | B is a filter, not a state |
| **T** (transformation) | What the system does to inputs | **The linear map itself.** T : ℝⁿ → ℝⁿ. Given v(t) and u(t), produces v(t+1). Each process primitive defines a different T | T is the function, not the argument |
| **H** (history) | What the system has learned | **The trajectory record.** H(t) = [v(t-k), ..., v(t-1), v(t)]. May condition which T is applied at the next step | H is a sequence of past vectors, not the current one |
| **Δt** (timescale) | How fast the system operates | **The sampling rate.** Determines how many times T is applied per parent step. Controls block iteration structure | Δt is a clock parameter, not a spatial dimension |

---

## What v(t) Actually Contains

The state vector v(t) is a column of numbers. Each number is the current value of one internal variable. The variables come from process primitives:

```
Subsystem          Primitive      Variable(s) contributed to v(t)          Persistent?
──────────────────────────────────────────────────────────────────────────────────────
Hash Production    Buffering      storage_level                            YES — accumulates
Block Assembly     Combining      current_activity (= Σ inflows)           NO — recomputed
Mempool            Buffering      storage_level                            YES — accumulates
Block Processor    Propelling     current_throughput (= in × efficiency)   NO — recomputed
Consensus Rules    Modulating     modulation_factor                        NO — recomputed
Network Layer      Propelling     current_throughput                       NO — recomputed
Chain State        Buffering      storage_level                            YES — accumulates
Protocol Research  Sensing        current_signal                           NO — recomputed
Code Impl.         Propelling     current_throughput                       NO — recomputed
Review & Gov.      Modulating     modulation_factor                        NO — recomputed
```

For the Bitcoin Level 2 decomposition, v(t) might look like:

```
v(t) = ┌ hash_storage_level      ┐    ← Hash Production (Buffering)
       │ block_assembly_activity  │    ← Block Assembly (Combining)
       │ mempool_level            │    ← Mempool (Buffering)
       │ block_throughput         │    ← Block Processor (Propelling)
       │ consensus_mod_factor     │    ← Consensus Rules (Modulating)
       │ network_throughput       │    ← Network Layer (Propelling)
       │ chain_state_level        │    ← Chain State (Buffering)
       │ research_signal          │    ← Protocol Research (Sensing)
       │ code_throughput          │    ← Code Implementation (Propelling)
       └ governance_mod_factor    ┘    ← Review & Governance (Modulating)
```

This is a vector in ℝ¹⁰. The structural decomposition (S) defined these 10 dimensions. The simulation fills in the 10 numbers and updates them each tick.

---

## Process Primitives as Linear Maps on v(t)

Each primitive defines how its slice of v(t) updates. These are the entries of T.

```
Primitive     Update rule                            Matrix form (1D)          Nature
─────────────────────────────────────────────────────────────────────────────────────────
Buffering     s(t+1) = s(t) + in - out               [1]·s + (in - out)       Affine (has constant term)
Combining     a = Σ(all_inflows)                      [1 1 1 ...]·inflows      Linear (weighted sum)
Splitting     each_out = a / n                        (1/n)·a                  Linear (scalar multiply)
Propelling    out = in × efficiency                   [η]·in                   Linear (scalar multiply)
Impeding      out = in × (1 - impedance)              [1-r]·in                 Linear (scalar multiply)
Sensing       signal = f(physical_in)                 [k]·in (linear approx)   Linear (transduction)
Modulating    out = primary × f(control)              diag multiply            Bilinear (two inputs)
Inverting     out = max - in                          [-1]·in + max            Affine (has constant term)
Copying       each_out = in  (Message only)           [1; 1; 1; ...]·in        Linear (fan-out)
```

**Key observations:**
- **Buffering** is the only primitive with persistent state — its storage level carries across ticks. All others recompute from current inputs. This determines which primitives need H and which are Markovian
- **Buffering** and **Inverting** are affine, not linear — they have constant terms. Axler Ch 3 distinguishes these: a linear map sends 0 to 0; an affine map may not
- **Modulating** is bilinear (product of two inputs) — not linear in either input alone. This is where the system becomes genuinely nonlinear
- **Splitting** conserves (valid for Material/Energy). **Copying** replicates (valid for Message only). Substance type constrains which maps are legal in which dimensions

---

## Composite Subsystem State: Direct Sum

Most subsystems aren't a single primitive. Mining = Combining + Propelling. The state is the **direct sum**:

```
State(Mining) = State(Combining) ⊕ State(Propelling)
             = (activity) ⊕ (throughput)
             = vector in ℝ²
```

The composite T dispatches to each primitive's T in sequence, with internal wiring connecting outputs to inputs. This is **function composition**: T_Mining = T_Propelling ∘ T_Combining.

For the parent system, child states combine the same way:

```
State(Bitcoin) = State(Mining) ⊕ State(Validating) ⊕ State(Development) ⊕ State(Protocol)
```

The parent's T is **block-structured**:

```
T_Bitcoin = │ T_Mining      F_M→V        0            0         │
            │ 0             T_Validating  0            F_V→P     │
            │ 0             0            T_Development F_D→P     │
            │ F_P→M         0            0            T_Protocol │
```

- **Diagonal blocks** (T_Mining, T_Validating, ...): each child's own transformation
- **Off-diagonal blocks** (F_M→V, F_V→P, ...): coupling matrices derived from interaction graph N
- **Zero blocks**: no direct flow between those children

N (the internal network from the 8-tuple) tells you which off-diagonal blocks are nonzero. The interaction graph IS the sparsity pattern of the parent's T matrix.

---

## Hierarchical Δt as Iterated Maps

When Mining (Minute) lives inside Bitcoin (Year), one parent step produces 525,600 child steps.

The child's T is applied 525,600 times. The *result* of those iterations is what the parent sees as one update to the Mining subspace of v(t).

```python
# Current (flat, wrong):
if tick % step_interval == 0: act()

# Correct (hierarchical):
for child_step in range(parent_interval // child_interval):
    child_state = child_T(child_state, child_inputs)
parent_state[child_subspace] = child_state
```

This is **iterated linear maps** — applying T many times. The eigenvalues of T determine behavior:
- |λ| < 1 for all eigenvalues → child state converges (stable)
- |λ| = 1 → child state oscillates (marginally stable)
- |λ| > 1 for any eigenvalue → child state diverges (unstable)

This is why spectral theory (Axler Ch 5) matters for simulation stability — the eigenvalues of your primitive T functions determine whether the simulation blows up or settles.

---

## Seven Mechanisms for Boundedness

The eigenvalue section above asks: does v(t) diverge? The answer: Mobus's framework has seven independent mechanisms that prevent unbounded growth. A simulation that implements none of them will diverge — this is expected, not a bug in the primitives.

These mechanisms are ordered from most fundamental (thermodynamic) to most compositional (evolutionary). A well-grounded sim layers them in progressively.

| # | Mechanism | Source | What it constrains | Implementation status |
|---|-----------|--------|-------------------|----------------------|
| 1 | **Conservation** | 1st/2nd Laws (Ch. 3 MEIK) | Material/Energy flows can't create substance. Every transform has η < 1, so every cycle loses energy. Message is the exception — Copying replicates without loss | Not implemented |
| 2 | **Lawful state space S_L(K)** | Bunge Vol. 4 §2.2, `State.lean` line 47 | v(t) confined to physically realizable states. S_L(K) ⊂ S(K). A state where output >> input violates the law predicate | Not implemented |
| 3 | **Boundary interfaces** | Eq. 4.6 B = {P, I}, `Boundary.lean` | Every external flow crosses an interface with its own T function. The boundary IS a bottleneck — nothing enters/exits except through a capacity-limited interface | Not implemented (interfaces are pass-through) |
| 4 | **Edge capacity κ** | Eq. 4.4 N = {C, L}, `FlowNetwork.lean` line 34 | Every flow edge has a hard throughput ceiling. Property of the CONNECTION, not the agent | **Implemented v0.4.0** — `capacity` field on flows, clamped in `_produce_outputs` |
| 5 | **Hierarchical governance** | Ch. 7-8 HCGS | Negative feedback via Force interactions (parameter injection). Governance doesn't add flow — it modulates parameters of work processes | **Implemented v0.4.0** — `force_polarity` field, positive/negative modulation of `agency_capacity` |
| 6 | **Multi-timescale damping** | Eq. 4.1 Δt hierarchy | Slow governance corrects fast operational loops. A fast positive feedback is counteracted by slower negative feedback at the coordination level | Partially implemented — `should_step()` supports different intervals, but models use uniform time constants |
| 7 | **Environmental selection** | Bunge Postulate 1.6, `Selection.lean` line 124 | All systems subject to environmental selection. Unbounded systems are not viable over evolutionary timescales. Structural guarantee, not per-step mechanism | Not implemented (evolutionary timescale) |

**Key insight (from 2026-05-13 research session):** `agency_capacity` is transformation efficiency — how well T converts inputs to outputs. It is NOT carrying capacity. Carrying capacity κ is a property of the flow edge (Mechanism 4) or the boundary interface (Mechanism 3). Using `agency_capacity` as κ conflates two distinct concepts.

**Why the Bitcoin cycle diverges:** The Validating→Mining→Protocol→Validating cycle grows because (a) PassiveSystem relays set activity = throughput with no damping, (b) no flow edges have capacity values, (c) the Protocol→Mining Force was positive feedback instead of negative, and (d) no conservation check prevents outputs from exceeding inputs. Adding edge capacities and negative force polarity (Mechanisms 4+5) is the theoretically grounded first fix. Conservation enforcement (Mechanism 1) is the next layer.

---

## The H Element: When T Itself Changes

**Stateless (most primitives):**
```
v(t+1) = T · v(t) + u(t)
```
Same T every step. Linear dynamics.

**History-conditioned (anticipatory/intentional agents):**
```
v(t+1) = T(H(t)) · v(t) + u(t)
```
T depends on past states. The system is **nonlinear** even if each individual T is linear, because the choice of which T to apply depends on the trajectory.

**For v1 implementation, keep it simple:**
- Buffering's storage level IS its H — the accumulator's current value embodies its history
- All other primitives: Markovian (no H needed for v1)
- Agent-level H (anticipatory/intentional): deferred to v2

---

## Dashboard Design: The Grassmann View

Grassmann's insight: directed magnitudes — things with both size and direction — deserve a unified visual language. The dashboard should make v(t), the space S defines, and T's action on v(t) **directly visible**.

### Core principle

Grassmann treated force, velocity, displacement, and area with the same algebraic formalism because they are all directed magnitudes. The dashboard should treat storage levels, throughputs, signals, and modulation factors with the same visual formalism because they are all components of v(t). A storage level and a throughput are both numbers that can grow or shrink. The dashboard makes them look like instances of one thing — entries in a vector — not unrelated numbers.

This is not decoration. It is the visual manifestation of the mathematical claim that all these quantities live in the same vector space.

### Five panels

**State Panel — v(t) as a single visible object:**
- One row per subsystem, showing current values of all its state variables
- Magnitude as bar width. Direction of change (from last tick) as color shift: brightening = growing, dimming = shrinking
- The entire column IS the state vector. Draw it as a unified vertical stack — a single mathematical object, not isolated numbers
- Annotate each row with its primitive type: [B] Buffering, [P] Propelling, [M] Modulating, etc.

**Flow Panel — the off-diagonal entries of T:**
- Every active flow as a directed edge with thickness ∝ current magnitude
- Substance hue: Energy = amber, Material = teal, Message = violet
- Usability line style: Resource = solid, Product = solid bright, Waste = dashed, Disruption = jagged
- This IS the coupling structure. The visual graph IS the sparsity pattern of the block matrix

**Transformation Panel — what T did this step:**
- For each subsystem: show v_before → T → v_after as input/output pair
- Highlight which primitive's T was applied
- If composite: show T₃ ∘ T₂ ∘ T₁ as sequential stages with intermediate vectors

**History Panel — H as trajectory:**
- Sparkline per state variable showing last N values of that component of v(t)
- For Buffering: this IS the stock trajectory — the accumulation over time
- Long horizontal runs = persistent features. Spikes = transients
- This panel answers: "where has v(t) been?"

**Hierarchy Panel — block structure of T:**
- Nested rectangles: parent contains children
- Pulse animation showing child steps within parent step
- The 525,600:1 ratio should be *felt* — fast children visibly iterate while parent holds still
- When a child block updates, its rectangle briefly highlights in its primitive color

---

## Learning Checkpoints

After each implementation milestone, verify you can answer the corresponding math question:

| Implementation milestone | Math question | Axler chapter |
|---|---|---|
| Write `_act_buffering()` | What is an affine map? Why does Buffering's constant term (net inflow) make it affine rather than linear? | Ch 3A (Linear Maps) |
| Write `_act_propelling()` | What is scalar multiplication? Why is efficiency a scalar acting on the input vector? | Ch 1A (ℝⁿ and ℂⁿ) |
| Combine two primitives in one subsystem | What is composition of linear maps? What is the matrix of a composition? | Ch 3D (Invertibility) |
| Build parent v(t) from children | What is a direct sum of vector spaces? What is a subspace? Can you identify each child's subspace within the parent space? | Ch 1C (Subspaces), Ch 5 |
| Implement hierarchical Δt | What happens when you apply a linear map repeatedly (Tⁿ)? What do eigenvalues predict about convergence? | Ch 5 (Eigenvalues) |
| Build flow-coupling matrix from N | What is a block matrix? What do the off-diagonal blocks represent? | Ch 3C (Matrices) |
| Implement conservation check | What does it mean for a linear map to preserve a quantity? What are the row sums of a stochastic matrix? | (Strang Ch 8) |
| Notice Modulating is bilinear | Why is the product of two inputs not a linear map? What breaks? | Ch 3A (definition) |

---

## Session Protocol

When working on BERT simulation with Claude:

1. **Name the space**: Before writing any code, state what the state space is — its dimensions, what each coordinate means, which element of C contributes which axes
2. **Name the point**: State what v(t) contains right now — the actual numbers
3. **Name the map**: State which T (primitive) operates on which slice of v(t), in both Python and matrix notation
4. **Name the coupling**: For any flow in N, identify which off-diagonal block of the parent's T matrix it populates
5. **Document both ways**: Every code comment that says "update storage" also says "apply affine map s(t+1) = s(t) + net_inflow." Every docstring names the Mobus concept AND the linear algebra concept
6. **Test with tiny vectors**: Before running full Bitcoin, test each primitive with 1D or 2D vectors where you can verify the matrix multiplication by hand
7. **Draw it**: When confused, draw v(t) as a column of numbers, T as a matrix, multiply them, see where the outputs land

---

## Files This Context Touches

- `bert/python/agents.py` — `_act()` methods: currently dispatched by archetype (Economy, Governance, Agent, Passive), not by primitive. Target: primitive-based dispatch defining T per primitive type
- `bert/python/model.py` — `step()`: currently tick-increment + shuffled agent stepping. Target: parent-level block-structured T application with hierarchical Δt
- `bert/docs/system-language-spec.md` §4 — Execution Mapping (10 subsections, v0.4.0)
- `bert/docs/process-primitives.md` — primitive definitions (T functions). 9 from Mobus Ch. 3 + Inverting from Ch. 4 = 10 total; Rust enum encodes 9 (Amplifying absent)
- `bert/docs/h-element-theory.md` — H as computational state. Status: theoretical — H is currently a string field, never read during `_act()`
- `bert/src/bevy_app/data_model/mod.rs` — ProcessPrimitive enum (9 variants), AgentModel.primitives field

## Key Textbook References

- **Axler, *Linear Algebra Done Right* (4th ed, 2024)** — free PDF from Springer. THE text.
- **Strang, *Introduction to Linear Algebra* (6th ed)** — for computational/matrix emphasis when Axler is too abstract
- **3Blue1Brown, *Essence of Linear Algebra*** — geometric intuition, watch alongside Axler
- **Grassmann, *Die Lineale Ausdehnungslehre* (1844)** — trans. Kannenberg, Open Court 1995. The philosophical ancestor. Read the preface.

---

*Update this file as implementation proceeds. Each completed checkpoint gets a date stamp and a one-line note on what was learned.*
