# BERT System Language Specification v0.1

## Preamble

### Purpose

This document specifies the **System Language** (SL) implemented by BERT — the Bounded Entity Reasoning Toolkit. The SL is a typed, hierarchical modeling language for describing complex systems following George Mobus's systems ontology, particularly the Deep Systems Analysis (DSA) framework presented in *Systems Science: Theory, Analysis, Modeling, and Design* (2022).

The SL enables systems scientists to construct formal, machine-readable models that capture a system's composition, boundaries, flows, behavioral roles, and hierarchical decomposition. These models serve as both analytical artifacts and simulation blueprints.

### Audience

Systems scientists authoring models in BERT. This specification assumes fluency in Mobus's systems science vocabulary. Computational details (JSON fields, Rust types, ECS components) appear as secondary annotations — the primary language is systems science.

### Running Example

The **Bitcoin network** serves as the reference model throughout this specification. Bitcoin is modeled as a 3-level decomposition:
- **Level 0**: Bitcoin as a unified system with 4 major subsystems and 3 interface subsystems
- **Level 1**: Mining, Validating, Development, Protocol — each decomposed into functional sub-subsystems
- **Level 2**: Hash Production, Block Assembly, Mempool, Block Processor, Consensus Rules, Network Layer, Chain State, Protocol Research, Code Implementation, Review & Governance

The reference implementation is at `assets/models/examples/bitcoin.json`.

### Theoretical Grounding

The SL is grounded in three formal sources:

1. **Mobus 8-tuple**: Every system is `S = ⟨C, N, E, G, B, T, H, Δt⟩` — components, internal network, environment, external flows, boundary, transformation, history, and timescale.

2. **Lean 4 formalization**: Machine-verified proofs establish 4 coherence constraints that every valid system must satisfy, plus a bridge theorem showing that every Mobus system projects to a Bunge CES (Composition-Environment-Structure) triple with formally characterized information loss.

3. **OWL/RDF ontology**: 40 entities in a formal ontology that bridges Mobus's mathematical framework to BERT's computational representation.

### Scope

This specification covers the 40 concepts currently implemented in BERT. Unimplemented Mobus concepts are listed by name in §6 (Known Gaps) but are not defined here. The specification version is `0.1` — it establishes the structural framework. Future versions will add formal grammar notation, validation pseudocode, and simulation templates.

---

## 1. Lexicon — Typed Primitive Elements

Every element the System Language can express. Each entry provides a definition in systems science terms, its Mobus grounding, and a concrete reference to the Bitcoin model.

### 1.1 Environment

The **environment** is everything outside the system boundary that the system interacts with. Following Mobus, the environment has two parts: discrete **objects** (entities that participate in flows) and an ambient **milieu** (background conditions that influence but don't directly connect). In Bitcoin, the environment contains Users (who submit transactions and receive confirmations), the Power Grid (which supplies electricity), and GitHub (which receives code contributions).

Every model has exactly one environment.

- **Mobus**: E in the 8-tuple — `E_{i,l} = ⟨O_{i,l}, M_{i,l}⟩`
- **JSON**: `WorldModel.environment` → `Environment { info, sources, sinks }`
- **OWL**: `bert:Environment`
- **ID**: Always `E-1` (the `-1` sentinel distinguishes environment from system indices)

### 1.2 External Entity

An **external entity** is a discrete object in the environment that participates in flows across the system boundary. External entities are not typed as Source or Sink — their role is **derived from flow direction**. An entity is acting as a source when flows originate from it, and as a sink when flows terminate at it. The same entity may play both roles.

In Bitcoin, "Users" appears twice in the environment: once with outgoing flows (transaction requests — acting as source) and once with incoming flows (confirmed transactions — acting as sink). These two appearances are linked by `is_same_as_id`, declaring they represent the same real-world entity.

**Aliasing**: The `is_same_as_id` field declares that two external entity entries represent the same real-world entity. This is the general aliasing mechanism in the SL — it also extends to interface pairs (see §1.5).

**Equivalence class**: The `equivalence` field (Mobus's `e_{i,k,l}`) assigns an equivalence class label. Entities sharing the same label are instances of the same type. This enables compact representation — rather than enumerating 10,000 Bitcoin full nodes individually, assign equivalence class `"FullNode"` and use Multiset complexity (see §1.8).

- **Mobus**: Members of O in E — discrete environmental objects
- **JSON**: `ExternalEntity { info, transform, equivalence, model, is_same_as_id }`
- **OWL**: `bert:Source`, `bert:Sink` (note: OWL retains type distinction for query convenience; the spec derives role from flow direction)
- **Bitcoin example**: "Users" (source-role: `Src-1.0`), "Power Grid" (source-role: `Src-1.1`), "GitHub" (source-role: `Src-1.2`), "Users" (sink-role: `Snk-1.0`), "Environment" (sink-role: `Snk-1.1`)

> **Migration note**: Prior BERT versions stored Source/Sink as an explicit `ExternalEntityType` field. The Lean formalization proves this is equivalent to edge direction in the external flow graph. The v1 spec canonizes the edge-derived representation. The `ty` field is removed from the semantic model; implementations may retain it for backward compatibility but must validate consistency with actual flow directions.

### 1.3 System

The **system** is the primary compositional unit of the SL. It represents a bounded entity with internal structure, behavior, and environmental coupling. Every system maps to Mobus's 8-tuple:

| Mobus element | SL field | Meaning |
|---------------|----------|---------|
| C (Components) | Child systems with matching `parent` | The parts |
| N (Internal network) | Interactions between subsystems | How parts connect |
| E (Environment) | `WorldModel.environment` (top-level) or parent's interior (subsystems) | What's outside |
| G (External flows) | Interactions crossing the boundary | How inside connects to outside |
| B (Boundary) | `boundary` | The membrane |
| T (Transformation) | `transformation` | What the system does to inputs |
| H (History) | `history` | What the system has learned |
| Δt (Timescale) | `time_constant` | How fast the system operates |

**Membership function**: The `member_autonomy` field implements Mobus's `m_{i,k,l}` — the degree to which a component is currently a member of this system, in [0, 1]. A value of 1.0 means the component is always a member (Bitcoin's Protocol subsystem is always part of Bitcoin). A value less than 1.0 indicates partial or conditional membership — a developer contributing to multiple open-source projects, or a mining node that intermittently joins and leaves the network.

> **Nomenclature note**: Despite the field name `member_autonomy` in the JSON schema, this is Mobus's membership function, not a measure of autonomy. The name will be corrected in a future schema version. See Mobus §4.3.3.1.

**Equivalence class**: The `equivalence` field (Mobus's `e_{i,k,l}`) assigns an equivalence class label, as with external entities. Systems sharing the same label are structurally equivalent instances.

- **Mobus**: S — the central object of analysis
- **JSON**: `System { info, sources, sinks, parent, complexity, boundary, radius, transform, equivalence, history, transformation, member_autonomy, time_constant, archetype, agent }`
- **OWL**: `bert:System`, `bert:Subsystem`
- **Bitcoin example**: Bitcoin network is the Level 0 system (`S0`). Mining (`C0.1`), Validating (`C0.2`), Development (`C0.4`), and Protocol (`C0.5`) are Level 1 subsystems.

### 1.4 Boundary

The **boundary** is the membrane separating a system's interior from its environment. It has two aspects:

1. **Properties**: Continuous parameters characterizing the boundary's behavior
   - `porosity` ∈ [0, 1] — how permeable the boundary is to flows. 0.0 = impermeable (nothing crosses), 1.0 = fully open (no restriction on throughput).
   - `perceptive_fuzziness` ∈ [0, 1] — uncertainty in where the boundary lies. 0.0 = crisp boundary (clear inside/outside distinction), 1.0 = diffuse boundary (ambiguous membership).

2. **Interfaces**: The set of ports through which all flows must pass. The boundary **shields** internal components from direct environmental contact — this is a theorem, not just a convention (see §2.6).

The `parent_interface` field on a subsystem's boundary indicates that this subsystem mediates a parent-level interface (see §1.6 InterfaceSubsystem).

- **Mobus**: B in the 8-tuple — `B_{i,l} = ⟨P_{i,l}, I_{i,l}⟩` (properties + interfaces)
- **Lean**: `MobusBoundary { properties, interfaces }` with `BoundaryComplete` theorem
- **JSON**: `Boundary { info, porosity, perceptive_fuzziness, interfaces, parent_interface }`
- **OWL**: `bert:Boundary`
- **Bitcoin example**: The Bitcoin system boundary has porosity 0.7 (relatively open — anyone can submit transactions) and 5 interfaces: Heat Dissipation (Export), Transformers (Import), GitHub (Import), Software Wallet (Export), Node RPC (Import).

### 1.5 Interface

An **interface** is a typed port on a system's boundary through which flows enter or exit. Interfaces are directional:

- **Import**: Receives flows from outside the boundary into the system
- **Export**: Sends flows from inside the system out through the boundary

There is no `Hybrid` type. Bidirectional interfaces are modeled as an **Import + Export pair** at the same boundary location, linked by `is_same_as_id`. This extends the aliasing mechanism to interfaces: the two ports represent one real-world connection point with bidirectional capability.

**Protocol**: Each interface declares its interaction protocol from a controlled vocabulary:
- `JSON-RPC` — structured remote procedure call (e.g., Bitcoin's Node RPC interface)
- `P2P-gossip` — peer-to-peer broadcast protocol (e.g., Bitcoin's network layer)
- `Git-PR` — version control pull request workflow (e.g., Bitcoin's GitHub interface)
- `REST-HTTP` — RESTful HTTP API
- `Stratum` — mining pool communication protocol
- `SMTP` — email-based communication

This vocabulary is extensible — modelers may add domain-specific values. An empty protocol field indicates a structurally incomplete model.

- **Mobus**: Members of I in B — the interface set
- **JSON**: `Interface { info, protocol, ty, exports_to, receives_from, angle }`
- **OWL**: `bert:Interface`
- **Bitcoin example**: "Software Wallet" is an Export interface with protocol describing the user-facing transaction submission mechanism. "Transformers" is an Import interface receiving electricity from the Power Grid.

> **Bitcoin update needed**: The Node RPC interface is currently modeled as a single interface but should be an Import + Export pair linked by `is_same_as_id`, since it both receives queries and returns responses.

### 1.6 InterfaceSubsystem

An **InterfaceSubsystem** is a subsystem that exists at both levels of the hierarchy simultaneously — it is a component of the parent system AND mediates flows through a parent-level interface. This is the non-obvious concept in the SL: it resolves how flows that cross a parent's boundary connect to the internal network at the child level.

In Bitcoin, the GitHub InterfaceSubsystem (`C0.51`) mediates the GitHub Import interface. At the parent level, code contributions flow from GitHub (external entity) through the GitHub interface into the Bitcoin system. At the child level, the GitHub InterfaceSubsystem receives those contributions and routes them to the Development subsystem via internal flows.

**Reconstruction rule**: A system S is an InterfaceSubsystem if its boundary's `parent_interface` field references a valid interface on the parent system's boundary. The reconstruction proceeds as follows:

1. During model loading, if `system.boundary.parent_interface` is set, the system is flagged as an InterfaceSubsystem
2. The referenced interface ID must resolve to an existing interface on the parent boundary — if it does not, the model is invalid
3. The InterfaceSubsystem's index path matches the referenced interface's index path (they share identity in the hierarchy)
4. The system is classified as Import or Export based on whether the parent interface has incoming or outgoing connections (or both)
5. At the parent level, the InterfaceSubsystem appears as the internal endpoint of cross-boundary flows

**Validation requirements**:
- `parent_interface` must reference a valid interface ID on the parent system's boundary
- The InterfaceSubsystem's own boundary interfaces handle the internal routing
- An InterfaceSubsystem without a valid parent_interface reference is a model error

- **Mobus**: Interface components simultaneously in C and I — the interface subset property
- **Lean**: The `iface_sub` constraint: `I ⊆ C`
- **JSON**: No dedicated type — reconstructed from `Boundary.parent_interface` presence
- **ECS runtime**: `InterfaceSubsystem` component + `ImportSubsystem`/`ExportSubsystem` markers

### 1.7 Interaction

An **interaction** is a directed connection between two entities carrying a typed substance or exerting influence. Interactions are the edges that constitute the internal network N and external flow graph G.

**Interaction types**:
- **Flow**: Carries substance across a connection. Substance crosses from source to sink. This is the standard interaction — material, energy, or messages physically transfer between entities.
- **Force**: Exerts influence without substance transfer. The source shapes the sink's behavior without anything crossing the boundary. Forces are required for governance and regulatory interactions where policy constrains behavior without material exchange.

  In Bitcoin, the interaction "Protocol Rules & Parameters" from Protocol to Mining may be better classified as a Force — the protocol doesn't transfer material to miners, it constrains their behavior. Contrast with "Mined Blocks" (Mining → Validating), which is a Flow — actual block data transfers between subsystems.

**Substance** (for Flows):
- `ty` — the primary substance type: `Energy`, `Material`, or `Message` (Mobus's three substance categories)
- `sub_type` — a controlled qualifier from a per-type vocabulary:

  | Primary type | Valid sub_types |
  |-------------|-----------------|
  | Energy | Electricity, Thermal, Kinetic, Chemical, Radiant, Bandwidth |
  | Material | Liquid, Gas, Solid, Biological |
  | Message | Data, Signal, Document, Code, Currency, Transaction, Consensus-Rules, Contribution |

  This vocabulary is extensible. Domain extensions require expert formalization against Mobus's substance taxonomy before entering the vocabulary. In v2, `sub_type` becomes an OWL subclass hierarchy of `ty`.

**Usability** (quadrant classification):

|  | Useful | Harmful |
|--|--------|---------|
| **Input** | Resource | Disruption |
| **Output** | Product | Waste |

- **Resource**: Useful input the system needs to function. Bitcoin example: Electricity from Power Grid, Transaction Requests from Users.
- **Product**: Useful output the system produces. Bitcoin example: Confirmed Transactions to Users.
- **Waste**: Harmful or unwanted output. Bitcoin example: Heat Dissipation to Environment.
- **Disruption**: Harmful input that degrades system function. Bitcoin examples: a 51% attack attempt is a Disruption to the Validating subsystem (harmful energy/computational input intended to subvert consensus). A DDoS flood is a Disruption to the Protocol subsystem (harmful message input overwhelming the network layer). A hostile fork proposal is a Disruption to the Development subsystem (harmful message input intended to fracture governance).

**Flow routing**: Every interaction specifies a 4-point path: `source → source_interface → sink_interface → sink`. For external flows, the source or sink is an external entity and the corresponding interface is on the system boundary. For internal flows, both source and sink are subsystems with interfaces on their respective boundaries.

**Quantification**: The `parameters` array is the canonical mechanism for attaching quantitative data to interactions. Each parameter has a name, value, and unit. The scalar `amount` and `unit` fields are deprecated in v1 (see §6 Known Gaps).

- **Mobus**: Edges in N (internal) and G (external)
- **JSON**: `Interaction { info, substance, ty, usability, source, source_interface, sink, sink_interface, parameters, endpoint_offset }`
- **OWL**: `bert:Flow` (+ `bert:EnergyFlow`, `bert:MaterialFlow`, `bert:MessageFlow`)
- **Bitcoin example**: "Confirmed Transactions" — Flow, Message/Transaction, Product, from Validating through Software Wallet to Users.

### 1.8 Complexity

**Complexity** classifies a system's internal structural richness. In v1, two classifications are available:

- **Complex**: The system has internal structure — subsystems, internal flows, decomposition. This is the default classification for any system that has been or will be decomposed.

- **Multiset(n)**: A collection of `n` structurally equivalent components. Rather than modeling each instance individually, a Multiset declares "there are n of these, and they are interchangeable." The `equivalence` field (see §1.3) is **required** for Multiset — it provides the class label that defines what "equivalent" means. A Multiset without an equivalence label is an invalid model.

  Bitcoin example: The full node network could be modeled as `Multiset(15000)` with equivalence class `"FullNode"` — 15,000 instances of a structurally identical validating node.

> **Known Gap**: The `Atomic` complexity type (indivisible, no internal structure) is deferred to v2. See §6.
>
> **Known Gap**: The `adaptable` and `evolveable` boolean flags on Complex carry real Mobus meaning (adaptable = can modify behavior without structural change; evolveable = can modify own structure) but are deprecated in v1. Bitcoin's Development subsystem should be `evolveable: true` as a reference case. See §6.

- **Mobus**: Complexity classification from DSA
- **JSON**: `Complexity` — tagged enum: `Complex { }` or `Multiset(u64)`
- **OWL**: `bert:SimpleSystem`, `bert:ComplexSystem`

### 1.9 Archetype (HCGS)

An **archetype** classifies a system's organizational role within its parent composition. Archetypes are descriptive — they aid understanding and guide simulation template selection, but do not enforce behavior.

- **Governance**: Sets policy, makes rules, coordinates other subsystems. In Bitcoin: "Review & Governance" subsystem controls protocol change decisions.
- **Economy**: Transforms resources into products — the value-creating core. In Bitcoin: "Mining" subsystem converts electricity into block production.
- **Agent**: Exercises autonomous decision-making — senses, decides, acts. In Bitcoin: "Development" subsystem exercises judgment about protocol direction.
- **Unspecified**: Role not yet classified. Default for new systems.

- **Mobus**: HCGS (Governance, Economy, Agent as organizational roles)
- **JSON**: `HcgsArchetype` enum: `Unspecified | Governance | Economy | Agent`

### 1.10 AgentModel

An **AgentModel** specifies the behavioral architecture of a system capable of autonomous action. It describes what kind of agent the system is, what processes it can perform, and how it connects to other agents.

**Kind hierarchy** (increasing cognitive sophistication):
- **Reactive**: Responds to stimuli with fixed rules. No internal model of the world.
- **Anticipatory**: Maintains a model of expected future states. Can act preemptively.
- **Intentional**: Has goals and plans. Can reason about actions and their consequences.

**Agency capacity**: A scalar in [0, 1] measuring the degree of autonomous action. Distinct from `member_autonomy` (membership function) — agency capacity measures what the agent *can do*, membership measures how *bound* it is to its parent system.

**9 Process Primitives** (Mobus's process taxonomy):
`Combining`, `Splitting`, `Buffering`, `Impeding`, `Propelling`, `Copying`, `Sensing`, `Modulating`, `Inverting`

These map to the atomic operations an agent can perform. A Mining subsystem might use Combining (assembling transactions into blocks), Buffering (mempool), and Sensing (monitoring network state).

- **Mobus**: Agent as a system with decision-making capacity
- **JSON**: `AgentModel { kind, agency_capacity, primitives, cognitive_params, process_configs, initial_state, network_config }`

### 1.11 Timescale

The **time constant** (`time_constant`) is Mobus's Δt — the characteristic timescale at which a system operates. This is a controlled enumeration:

`Millisecond` | `Second` | `Minute` | `Hour` | `Day` | `Week` | `Month` | `Year` | `Decade` | `Epoch`

In Bitcoin, the Protocol subsystem operates at `Minute` scale (~10 minute block time), while the Development subsystem operates at `Month` or `Year` scale (BIP process, release cycles). The Mining subsystem's Hash Production operates at `Millisecond` scale (individual hash computations).

In simulation mapping, `time_constant` drives the step rate for hierarchical agent composition — subsystems with faster timescales execute more steps per parent step (see §4).

- **Mobus**: Δt in the 8-tuple
- **JSON**: `time_constant: String` (one of the enumerated values)

---

## 2. Grammar — Legal Compositions

Rules for combining Lexicon elements into valid models. Every rule stated here is checkable against the Bitcoin reference model.

### 2.1 ID Encoding

Every element in the SL has a unique **identifier** encoding its type and position in the hierarchy.

**Serialization format**: `<TypePrefix><index.index.index...>`

| Type | Prefix | Example | Meaning |
|------|--------|---------|---------|
| System | `S` | `S0` | Root system |
| Subsystem | `C` | `C0.1` | 2nd child of root |
| Interface | `I` | `I0.0` | 1st interface on root's boundary |
| Source | `Src` | `Src-1.0` | 1st source-role entity in environment |
| Sink | `Snk` | `Snk-1.0` | 1st sink-role entity in environment |
| Environment | `E` | `E-1` | Environment (singleton) |
| Flow | `F` | `F0.0` | 1st flow at root level |
| Boundary | `B` | `B0` | Root system's boundary |

**Index semantics**:
- The index vector encodes a path from root to entity: `[0]` = root, `[0, 1]` = second child of root, `[0, 1, 3]` = fourth child of second child of root
- Environment uses `-1` as a sentinel: `[-1]` = environment itself, `[-1, 0]` = first entity in environment
- Indices are zero-based and sequential within each parent
- **Level** = length of indices - 1 (root is level 0, first decomposition is level 1)
- Environment and its children have level -1

**Invariants**:
- All indices are integers (negative only for the environment sentinel `-1`)
- Sibling indices are unique within their parent
- InterfaceSubsystem indices match their parent interface's indices

**Bitcoin example**: Mining subsystem is `C0.1` (subsystem, child 1 of root 0). Its Hash Production sub-subsystem is `C0.1.0` (child 0 of child 1 of root). The "Users" source is `Src-1.0` (first source in environment).

### 2.2 Hierarchy Rules

- Systems are stored in a flat array with a `parent` field linking children to parents. The hierarchy is reconstructed at load time.
- The root system has no parent. All other systems must reference a valid parent.
- `info.level` must equal the depth implied by the index vector (level 0 = root, level 1 = first decomposition, etc.)
- A system may contain subsystems only if its complexity is `Complex` or `Multiset`.

### 2.3 Flow Routing

Every interaction must specify the full 4-point path:

```
source → source_interface → sink_interface → sink
```

**External flows** (crossing the boundary):
- One endpoint is an external entity, the other is a system
- The interface endpoints are on the system's boundary
- Example: Users (`Src-1.0`) → Software Wallet interface → ... → Bitcoin system

**Internal flows** (within a system):
- Both endpoints are subsystems of the same parent
- Interface endpoints are on the respective subsystem boundaries
- Example: Mining → [Mining export interface] → [Validating import interface] → Validating

### 2.4 InterfaceSubsystem Instantiation

When a subsystem's `boundary.parent_interface` references a parent-level interface:
1. That subsystem becomes an InterfaceSubsystem
2. The referenced interface ID must exist on the parent system's boundary
3. The subsystem's own boundary interfaces handle internal-side routing
4. External flows terminate/originate at the InterfaceSubsystem rather than at the raw interface
5. The subsystem is classified as Import, Export, or both based on the parent interface's connection directions

### 2.5 Boundary Containment

- Every interface must be a child of a boundary (stored in `boundary.interfaces`)
- All cross-boundary flows must pass through an interface — no direct access to internal components from outside
- A boundary with no interfaces is valid but semantically implies an isolated system (no environmental interaction)

### 2.6 Coherence Constraints (Lean-Verified)

The Lean formalization (`Systems/Mobus/Tuple.lean`) proves 4 machine-checked invariants. Any valid SL model must satisfy all four:

**Constraint 1 — Component-Network Consistency** (`net_on`):
Internal network edges only connect components.
∀ (a, b) ∈ N: a ∈ C ∧ b ∈ C

*Practical meaning*: You cannot draw an internal flow between an external entity and a subsystem. All internal flows connect subsystems to subsystems.

**Constraint 2 — Composition-Environment Disjointness** (`comp_env_disjoint`):
No entity is simultaneously a component and an environmental object.
C ∩ O = ∅

*Practical meaning*: A subsystem cannot also be an external entity. If "Users" is in the environment, there cannot be a "Users" subsystem.

**Constraint 3 — Interface Subset** (`iface_sub`):
All interface components are components.
I ⊆ C

*Practical meaning*: InterfaceSubsystems are full members of the system's composition — they appear in C, not just in B. This is the formal basis for the InterfaceSubsystem concept.

**Constraint 4 — Bipartite External Flows** (`ext_bipartite`):
External flows connect environment objects to interface components, or vice versa — never environment-to-environment or interface-to-interface.
∀ (a, b) ∈ G: (a ∈ O ∧ b ∈ I) ∨ (a ∈ I ∧ b ∈ O)

*Practical meaning*: Cross-boundary flows always have one foot inside (interface) and one foot outside (environment object). Two external entities cannot be directly connected through the system.

**Derived theorem — Boundary Completeness** (`boundaryComplete`):
Every external flow passes through an interface. Non-interface components are **shielded** from direct environmental contact. This follows from Constraint 4 — it does not need separate axiomatization.

*Practical meaning*: If a subsystem is not an InterfaceSubsystem, it cannot have flows directly connecting it to external entities. All environmental interaction is mediated by the boundary.

### 2.7 Substance Compatibility

- Source and sink interfaces on the same flow should handle compatible substance types
- The primary compatibility axis is `substance.ty` (Energy/Material/Message) — a flow cannot carry Energy through a Message-typed interface
- `substance.sub_type` provides finer-grained qualification within the controlled vocabulary

### 2.8 Multiset Validity

- A system with `Multiset(n)` complexity **must** have a non-empty `equivalence` field
- The equivalence label defines what "structurally equivalent" means for this collection
- `n` must be a positive integer representing the count of equivalent instances

---

## 3. Semantics — What Valid Models Mean

How to interpret a structurally valid model. What claims it makes about the real system.

### 3.1 Purpose-Driven Reading

Following Mobus DSA: **read outputs first**. A system's Products and Waste define its purpose before its Resources and internal structure are considered. This is not mere convention — it reflects the analytical principle that a system exists to produce its outputs.

Bitcoin read in purpose-driven order:
1. **Products**: Confirmed Transactions (the system exists to provide trustless value transfer)
2. **Waste**: Heat Dissipation (unavoidable byproduct of proof-of-work)
3. **Resources**: Electricity, Transaction Requests, Code Contributions (inputs that enable the purpose)
4. **Internal structure**: Mining, Validating, Development, Protocol (how the purpose is achieved)

### 3.2 Flow Conservation

The aspiration that total substance entering a system equals total substance leaving (accounting for transformation). In v1 this is a **SHOULD**, not a MUST — the `parameters` mechanism provides quantitative capability, but no validator enforces conservation. Models should strive toward quantitative balance as they mature from structural sketches to simulation-ready specifications.

### 3.3 Boundary Semantics

- `porosity` governs flow volume: low porosity means the boundary restricts throughput, high porosity means flows pass freely. Bitcoin's relatively high porosity (0.7) reflects that anyone can submit transactions.
- `perceptive_fuzziness` governs boundary clarity: low fuzziness means clear inside/outside distinction, high fuzziness means membership is ambiguous. A corporate department has low fuzziness; a community of practice has high fuzziness.
- These parameters are advisory in v1 — they inform analysis and simulation design but are not computationally enforced by BERT.

### 3.4 Completeness Criteria

A model is **structurally valid** if it satisfies all Grammar rules (§2). A model is **semantically complete** if additionally:
- All interfaces have at least one connected flow
- All subsystems have at least one input and one output flow
- Boundary porosity and fuzziness are explicitly set (not default)
- Complexity classification matches actual structure (Complex if has subsystems, Multiset if equivalence-labeled collection)
- `time_constant` is set to a value from the controlled enumeration
- `protocol` is set on all interfaces
- `sub_type` is set on all substance declarations
- All Multiset systems have non-empty `equivalence` labels

**Stub models**: Models that are structurally valid but semantically incomplete. Stub indicators include empty `protocol` fields, missing `sub_type` values, or `time_constant` not yet specified. Stubs are useful for iterative modeling — capture structure first, refine semantics later.

### 3.5 Archetype Semantics

Archetypes describe organizational roles, not behaviors. A Governance subsystem is not *implemented* as a governor — it is *understood* as serving the governance function. The same subsystem might be classified differently by different analysts depending on their analytical frame.

In Bitcoin:
- **Governance**: Review & Governance — this subsystem's purpose is to evaluate and approve protocol changes
- **Economy**: Mining — this subsystem's purpose is to transform electricity into block production (the value-creating transformation)
- **Agent**: Development — this subsystem exercises autonomous judgment about protocol direction

### 3.6 Equivalence and Aliasing

**`is_same_as_id`**: Declares two model entities represent one real-world entity. Used when the same entity appears in multiple model roles:
- External entity in both source and sink positions (Bitcoin's "Users")
- Import and Export interface pair representing a bidirectional connection point

**`equivalence`** (Mobus's `e_{i,k,l}`): Declares type equivalence — entities sharing a label are instances of the same structural type. This is distinct from `is_same_as_id` (identity aliasing). Equivalence enables Multiset compression and supports analysis queries like "how many components of type X?"

### 3.7 Force Semantics

A **Force** interaction shapes behavior without substance transfer. The key distinction:
- **Flow**: Something moves from source to sink. The sink *receives* material, energy, or information.
- **Force**: The source *constrains or influences* the sink. Nothing transfers. The sink's behavior changes because of the source's existence or state.

Forces are essential for modeling governance and regulation. In Bitcoin, consensus rules constrain mining behavior — miners don't *receive* the rules as a message (they already have them); the rules *shape* what miners can do. This is a Force, not a Flow.

In simulation, Forces map to parameter constraints or rule injection rather than substance transfer in the agent step logic.

### 3.8 Bridge to Bunge

The Lean `toBunge` function maps every SL model (via its Mobus 8-tuple) to a Bunge CES triple:
- Composition = C (components)
- Environment = O (environmental objects — milieu M is lost)
- Structure = N ∪ G (all relations — flow capacities lost)

This projection is many-to-one. Six categories of information have no Bunge counterpart:
1. Milieu M (ambient variables)
2. Flow capacity κ (substance labels)
3. Boundary properties π (porosity, fuzziness)
4. Transforms τ (what the system does)
5. History η (what the system knows)
6. Timescale δ (how fast it operates)

**Practical implication**: SL models contain strictly more information than Bunge-style system descriptions. The internal/external flow separation (N vs G) = Mobus's two-element structure family; flattening them recovers Bunge's single "structure" relation.

---

## 4. Execution Mapping — Model → Simulation

How SL models translate to executable simulations.

### 4.1 JSON Schema

The canonical serialization is JSON via the `WorldModel` structure:

```
WorldModel {
  version: u32,           // Schema version (currently 1)
  environment: Environment,
  systems: Vec<System>,   // Flat array; hierarchy via parent field
  interactions: Vec<Interaction>,
  hidden_entities: Vec<Id> // UI extension — see §4.6
}
```

Serialization uses serde_json with externally-tagged enums. Complexity serializes as `{ "Complex": { } }` or `{ "Multiset": 1000 }`.

### 4.2 System → Mesa Agent (Primary Target)

| SL concept | Mesa mapping |
|------------|-------------|
| System | Agent class |
| System hierarchy | Model composition (MetaAgent) |
| Interface | Agent interaction port |
| Archetype | Behavior template (Governance → coordinator, Economy → transformer, Agent → autonomous) |
| `time_constant` | MetaAgent internal scheduler step rate |
| `member_autonomy` (membership) | Agent activation probability per step |

Subsystems with faster `time_constant` values execute proportionally more steps per parent step. A `Millisecond` subsystem inside a `Minute` parent executes ~60,000 sub-steps per parent step.

### 4.3 Interaction → Step Logic

| SL concept | Mesa mapping |
|------------|-------------|
| Flow | Substance transfer in `step()` method |
| Force | Parameter constraint / rule injection (no transfer) |
| Substance type | Channel selection (Energy → energy pool, Material → inventory, Message → message queue) |
| Usability | Step logic role (Resource → consumed, Product → produced, Waste → emitted, Disruption → degradation) |
| `parameters` | Quantitative step values |

### 4.4 AgentModel → Behavioral Architecture

| SL concept | Mesa mapping |
|------------|-------------|
| `kind: Reactive` | Fixed rule-based step logic |
| `kind: Anticipatory` | Step logic with internal world model and prediction |
| `kind: Intentional` | Goal-directed planning with action selection |
| `agency_capacity` | Degree of autonomous variation in step behavior |
| ProcessPrimitives | Atomic operations composed into step logic |

### 4.5 Bevy ECS: Visualization and Exploration

The BERT implementation uses Bevy ECS for model visualization. The ECS architecture has natural correspondence to agent systems:
- Bevy Entities = SL systems, interfaces, flows
- Bevy Components = SL properties and state
- Bevy Systems = update logic and rendering

This mapping is under exploration as a **native Rust simulation target** — ECS could run agent-based models directly without Python/Mesa translation. The natural alignment between ECS (entities with components processed by systems) and ABM (agents with state updated by rules) makes this a promising path. The spec leaves this door open.

### 4.6 UI Extension Fields

The following fields are **excluded from the semantic model** — they are visualization artifacts with no analytical meaning:

- `hidden_entities: Vec<Id>` — entities hidden in the visual editor
- `endpoint_offset` — visual placement of flow endpoints
- `angle` on Interface — visual placement on boundary
- `radius` on System — visual display size
- `transform` / `Transform2d` — visual position and rotation

Implementations may include additional UI extension fields. The semantic spec makes no claims about them.

### 4.7 Version and Migration

- `WorldModel.version` is currently `1`
- Breaking schema changes increment the version
- Migration from v0 (pre-spec models): remove `ExternalEntityType` field, validate flow-direction consistency, populate controlled vocabularies for `protocol` and `sub_type`

---

## 5. Known Gaps

### 5.1 Deprecated Fields (Removed from v1)

| Field | Reason | Future |
|-------|--------|--------|
| `amount` (on Interaction) | Always "1" — placeholder with no semantic value. `parameters` is the canonical quantification mechanism. | Reintroduce in v2 when simulation integration defines clear semantics. |
| `unit` (on Interaction) | Inconsistent usage, coupled to `amount`. | Reintroduce with `amount` in v2. |
| `smart_parameters` (on Interaction) | Experimental AI-assisted parameterization — never populated. | Revisit in v2 as extension of `parameters`. |
| `adaptable` (on Complex) | Carries real Mobus meaning (can modify behavior without structural change) but never set to `true` in any model. | Restore in v2 with simulation integration. Bitcoin's Development should be `adaptable: true`. |
| `evolveable` (on Complex) | Carries real Mobus meaning (can modify own structure) but never set to `true`. | Restore in v2. Bitcoin's Development should be `evolveable: true`. |
| `ExternalEntityType` (on ExternalEntity) | Redundant with flow direction per Lean proof. | Edge-derived role is canonical. Implementations may retain for backward compatibility with validation. |

### 5.2 Deferred Concepts

| Concept | Reason | Future |
|---------|--------|--------|
| `Atomic` complexity | No examples exist; no enforced validator rule. | Restore in v2 with examples and validation. |
| `Hybrid` interface type | Superseded by Import+Export pair pattern with `is_same_as_id`. | No plan to restore — pattern is superior. |

### 5.3 Unimplemented Mobus Concepts

The following Mobus concepts have no SL representation. They are listed by name only — definitions belong in the full ontology, not this spec. This list is derived from the onto-viz coverage analysis (BERT covers 29% of the Mobus ontology, approximately 40 of 101 concepts).

**Core system dynamics**: negative feedback, positive feedback, homeostasis, self-organization, emergence, causal chain, causal loop

**System classification**: anticipatory system, autopoietic system, viable system, dissipative system, complex adaptive system

**Fitness and adaptation**: fitness landscape, viability, adaptation, learning, evolution (as process)

**Information and knowledge**: information processing, signal, noise, entropy (information-theoretic), knowledge acquisition

**Process and function**: function (Mobus sense), process (as distinct from flow), transformation (as operator, beyond string description)

**Network properties**: connectivity, modularity, hierarchy (as measurable property), centrality

**Temporal dynamics**: steady state, transient, oscillation, attractor, basin of attraction

This list is not exhaustive — the full gap analysis is available via the onto-viz coverage tool.

---

## 6. Appendix: Grounding Sources

**Rust data model** — `src/bevy_app/data_model/mod.rs`
: JSON schema definition (1,433 lines, 16 structs, 9 enums)

**ECS components** — `src/bevy_app/components/system_elements.rs`
: Runtime semantics (1,090 lines)

**Load logic** — `src/bevy_app/data_model/load.rs`
: ID reconstruction, InterfaceSubsystem detection

**Save logic** — `src/bevy_app/data_model/save.rs`
: ID generation, serialization

**OWL/RDF ontology** — `gitbook/for-researchers/bert-systems-ontology.rdf`
: Formal ontology (40 entities, 10 object properties)

**Lean 8-tuple** — `systems-ontology/Systems/Mobus/Tuple.lean`
: Machine-verified coherence constraints

**Lean bridge** — `systems-ontology/Systems/Mobus/Bridge.lean`
: Mobus to Bunge projection with information loss characterization

**Lean boundary** — `systems-ontology/Systems/Mobus/Boundary.lean`
: Boundary completeness and shielding theorems

**Bitcoin model** — `assets/models/examples/bitcoin.json`
: Reference implementation

**System Language theory** — `gitbook/for-researchers/system-language.md`
: Theoretical grounding

**Mobus reference** — `docs/MOBUS_REFERENCE.md`
: DSA quick reference

**Coverage analysis** — `onto-viz/ARCHITECTURE.md`
: 29% coverage metric, gap identification
