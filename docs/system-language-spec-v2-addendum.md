# SL v2.0 Addendum: Kernel and Mode Architecture

**Status**: Draft (2026-06-11). Extends `system-language-spec.md` v0.1. Nothing here invalidates v0.1; every v0.1 model remains valid as a `Full`-mode model.

**Formal grounding**: `systems-science-foundations` (SSF), in particular `Systems/Klir/ViewGeneration.lean` (kernel and generated views, commit `abee84d`), `Systems/Klir/KlirSystem.lean` (projections and the commuting triangle), `Systems/Mobus/Tuple.lean` (coherence constraints, already cited by v0.1 section 2.6). This addendum is implementation-facing: it states what the proofs license BERT to build, and cites declarations by file. Theorem exposition lives in SSF.

---

## A1. What changed since v0.1

v0.1 grounds the SL in the Mobus 8-tuple as the primary object. Three results since then change the architecture:

1. **The kernel is proven and is data, not philosophy.** SSF defines `Kernel`: a set of things, a dependency relation, and the constraint that the relation lives on the things. The Klir, Bunge, and Mobus presentations are *generated* from it as views, each with an identity round trip. The 8-tuple is therefore a derived presentation, not the foundation. BERT keeps the 8-tuple as its working surface; the spec re-founds it.

2. **View preconditions are machine-checked.** Generating each view costs a named hypothesis (section A4). These hypotheses become SL validation rules with citable rationale.

3. **The 8-tuple splits into two faces.** Structure (C, N, E, G, B: components, networks, environment, external flows, boundary) is BERT's wheelhouse: typed decomposition, boundary accountability, TypeDB. Dynamics (T, H, Δt: transforms, history, timescale) is bert-compose's wheelhouse: conserved flows, rates, the five base-layer parameters (capacity, setpoint, time-constant, maintenance, back-pressure). The mode ladder below respects this split.

---

## A2. The kernel

The minimal object every SL model contains, mirroring SSF `Kernel`:

| Field | Meaning | SL realization |
|---|---|---|
| `things` | The relata (T) | Systems + external entities, by `Id` |
| `dep` | The dependency relation (R), directed pairs | Interactions, projected to (source, sink) pairs |
| on-ness | R is defined on T | Every interaction endpoint resolves to a declared entity |

Two consequences:

- **The on-ness constraint is what makes a model a system.** A bare (things, relations) pair where relations dangle off undeclared entities is not a system in any tradition. This is already enforced by BERT's ID resolution; the kernel names it as the load-bearing rule rather than a bookkeeping detail.
- **The kernel stores no slots.** Environment, boundary, interfaces, transforms, history, timescale are all view-level elaborations. A model's kernel is extracted by projection (v0.1 section 3.8 is the Bunge step of this; the full projection is SSF `MobusSystem.toKlir`).

## A3. The mode ladder

The v2.0 mode selector, re-derived from the proven views. A **lens** is a proven generated view (round trip and faithfulness machine-checked in SSF). A **mode** is an SL commitment level. Three modes are lenses; two are engineering levels that the lens results bracket.

| Mode | Lens? | Adds | Precondition (entry rule) | Source |
|---|---|---|---|---|
| `Core` | Yes (= Klir) | Nothing: things + dependencies | on-ness only | `Kernel`, `Kernel.toKlir` |
| `Structural` | Yes (= Bunge CES) | Environment, inside/outside | At least one bond between two distinct components | `Kernel.toBunge`, hypothesis `HasBond` |
| `Operational` | Yes (= Mobus, structural face) | Typed flow networks (N, G), boundary, interfaces | No self-dependencies, plus the four v0.1 section 2.6 coherence constraints | `Kernel.toMobus`, hypothesis `Irreflexive` |
| `Full` | Mode | The dynamical face: T, H, Δt populated | `Operational` plus dynamical slots non-empty | Two-faces split; realized by bert-compose |
| `Cybernetic` | Mode (future) | Feedback as first-class cycle | Open: the cyclic shape has no faithful finite acyclic comparison yet | SSF open question (Joslyn); homeostat/lens results (`Systems/Core/Lens.lean`) are the partial grounding |

Notes:

- **`Core` and a hypothetical `Klir` mode are the same mode, by theorem.** SSF proves the kernel and the dependency-respecting Klir system are interchangeable with definitional round trips. The ladder therefore has no separate Klir rung.
- **`Operational` vs `Full` is the two-faces split, not an ontology change.** Both are the Mobus view; `Operational` is its structural face (what BERT's editor and validator govern today), `Full` adds the dynamical face (what bert-compose executes). v0.1's section 2.6 constraints are exactly the `Operational` entry rules and need no change.
- **`mode: "Full"` is the backward-compatible default.** Every existing model is a `Full` model. Absence of the field means `Full`.
- **Slot count is a mode property.** "Why 8 slots" has no kernel-level answer because the kernel has no slots; the 8 are what the Mobus view fills. Mode declarations are presentation commitments, not ontological claims.

## A4. Mode declaration and validation

Schema addition (backward compatible):

```json
{
  "version": 5,
  "mode": "Full",
  "environment": { ... },
  "systems": [ ... ],
  "interactions": [ ... ]
}
```

Validation is mode-aware. Each mode's entry rule is the proven precondition, and the error message cites the tradition that charges it:

| Mode | Validator checks | On failure, the message can say |
|---|---|---|
| `Core` | Every interaction endpoint resolves | "Relations must be on things (the kernel constraint)" |
| `Structural` | Some interaction connects two distinct systems | "Bunge Def 1.1: a system requires at least one bond between distinct components; an unbonded collection is an aggregate" |
| `Operational` | No interaction has the same system as source and sink; section 2.6 constraints | "Mobus section 4.3: flow edges require k ≠ o; self-dependency is not representable in the 8-tuple" |
| `Full` | `Operational` plus `transformation`, `history`, `time_constant` populated where required | "Full mode requires the dynamical face" |

Relaxation direction: a lower mode requires strictly less. `Core` accepts models that `Structural` rejects (an unbonded aggregate is a valid Core object: things and relations without systemhood claims). This is deliberate and useful for early-stage modeling.

## A5. Mode transitions

**Downgrade (project)**: always total, always succeeds, loses information. The v0.1 section 3.8 information-loss catalogue (milieu, capacity, boundary properties, transforms, history, timescale) becomes the downgrade warning text. Downgrades correspond to SSF's projection maps (`toBunge`, `toKlir`) and compose along the commuting triangle.

**Upgrade (generate)**: conditional on the target mode's precondition, then fills new slots with minimal witnesses (empty environment, no external flows, empty parametric slots). This is SSF's generation direction, and the minimal witnesses are exactly the ones the proofs use. The UI flow: check precondition, report the named failure if unmet, otherwise generate and let the modeler elaborate the new slots.

**The read-only guarantee**: switching the displayed mode never mutates the model. The round trips are identities, so a model viewed in any mode and projected back is unchanged. This is the same invariant bert-compose's lens packs enforce empirically at the domain level (the lens-invariance test: one homeostat, four domain lenses, byte-identical CSVs). Tradition lenses over the kernel and domain lenses over the instrument are one pattern at two levels: rename and re-skin, never rewrite.

## A6. bert-core alignment (audit, 2026-06-11)

`bert-core` (`WorldModel`, `System`, `Interaction`) against the spec above. Divergences are filed, not fixed here:

1. **bert-core is the `Full` view with no kernel object.** Expected: the kernel is extracted by projection, so no struct is required yet. The bert#88 refactor decides whether the kernel becomes a first-class struct or stays a derived projection. Either satisfies this spec; the projection must exist as a function in any case.
2. **Semantic and presentation data are mixed.** `System.radius`, `System.transform`, `Interaction.endpoint_offset` are rendering concerns living beside ontological fields. The kernel/view separation suggests the same split inside bert-core (kernel data, view elaborations, presentation). Filed under bert#88 scope.
3. **The dynamical face is stringly-typed in bert-core** (`transformation: String`, `history: String`, `time_constant: String`) while bert-compose carries the real typed dynamical parameters. Consistent with the two-faces division of labor, but `Full`-mode validation ("dynamical slots populated") can only check non-emptiness until the faces share types. Acceptable for v2.0; noted for the compose/BERT convergence discussion.
4. **Self-loop check**: nothing in bert-core forbids an interaction whose source and sink resolve to the same system. Under this spec that is legal in `Core`/`Structural` and illegal from `Operational` up. New validator rule, mode-gated.

## A7. Out of scope

- The Rust refactor itself (bert#88, sequenced after this spec).
- Mesa bridge, BNF grammar, and v0.1 roadmap remainders.
- The `Cybernetic` mode's formal grounding (SSF open question; do not block v2.0 on it).
- Theorem exposition and paper framing (SSF and the CCT paper; this document cites, it does not restate).
