# BERT Compose — Positioning & Competitive Landscape

*Where bert-compose sits in systems-modeling software, foundationally and practically. Companion to [README.md](README.md), [ARCHITECTURE.md](ARCHITECTURE.md), and the vault's `strategy/bert-world-models-positioning.md`. Sourced June 2026.*

## What bert-compose is

A conservation-faithful systems simulator built on Mobus's atomic work processes: drag primitives, wire them, watch matter / energy / information flow and stay accounted every tick, read the same model across domains via lenses, and save it as typed BERT JSON. It is the tactile front of the convergence thesis (K ≅ **2**) — the claim that the common core of systems science is small and behaves the same across domains, made something you can build and run.

## The five-way distinctiveness

No single competitor is **all** of these at once; that combination is the position:

1. **GST/ontology-grounded** — primitives are Mobus work processes, not equation glyphs; the model is a typed entity structure, not a rate-equation soup.
2. **Conservation-faithful by construction** — a per-tick ledger makes mass accounting hold structurally, so a curve is evidence, not decoration. (Other tools leave conservation to the modeler.)
3. **Tactile for non-engineers** — plain-English cards, relatable substances, drag-and-drop; a governance researcher can enjoy the first five minutes.
4. **Cross-domain via lenses** — the same model reads as governance / neuro / crypto / ecology, the isomorphism displayed and machine-checked (identical CSV under every lens).
5. **Exports to a typed knowledge graph** — BERT JSON → TypeDB → Mesa, so a model is a knowledge artifact, not just a simulation file.

## The landscape, by category

### A. System Dynamics — the closest *practical* substitute

**Stella / iThink** (isee systems, since 1985, Barry Richmond), **Vensim** (Ventana, v10.4 Oct 2025, Monte Carlo + calibration), **InsightMaker** (2010, free/web, SD + ABM). These are what someone reaches for today to "build a thermostat and watch it settle" — your flow circuit *is* a stock-and-flow simulator.

**How bert-compose differs:** SD tools are **equation-first** (differential equations under the hood), not **ontology-first** — a flow is a rate equation, not a conserved transfer between bounded entities. **Conservation is on the modeler, not the tool** — nothing stops an SD model from leaking mass; the ledger makes it hold by construction. And SD has no cross-domain lens layer and no typed-graph export: SD models are simulation artifacts, not knowledge artifacts.

### B. Compositional / categorical — the closest *foundational* kin

**AlgebraicJulia / AlgebraicDynamics.jl** (built on Catlab.jl): builds dynamical systems *compositionally* from primitive systems via **operads and wiring diagrams** — "systems are morphisms, composition is the operation, the wiring diagram is the syntax." That is the K ≅ **2** thesis in someone else's hands. Closest on the deep idea.

**CatColab (Topos Institute)** — "a collaborative environment for formal, interoperable, conceptual modeling" (v0.5 *Sandpiper*, March 2026, alpha). Its stated aim: *"anyone, from citizen to scientist, can contribute their piece of understanding of the world in a language in which they're comfortable... combine their world models."* That is **almost word-for-word the cross-domain/lens thesis**, from a category-theory-native foundation.

**How bert-compose differs:** these are **research-grade, code-first / CT-native, for mathematicians** — Julia operads, double categories, abstract logics. bert-compose is **GST/Mobus-native, conservation-faithful, tactile**, with a *specific* convergence theorem (K ≅ **2**, machine-verified in Lean) and a working typed-graph export. Convergent on *vision*, divergent on *foundation* (CT-first vs GST-first) and on *tactility/simulation*. See "The Topos relationship" below — they are as much a potential ally as a competitor.

### C. Formal systems-process / behavior — the Troncale lineage

**Monterey Phoenix** (Naval Postgraduate School, behavior-modeling language that exposes emergent behaviors). Directly relevant: Giammarco & Troncale, *"Modeling Isomorphic Systems Processes Using Monterey Phoenix"* (Systems, 2018) modeled the **Cycles** systems process — oscillation, lifecycle, recycling, reinforcement — from a six-line MP model. This is the *prior* formalization of Troncale's isomorphic processes, the exact territory the `troncale-sweep/` addresses.

**How bert-compose differs:** MP formalizes process *behaviors* as event traces for verification; bert-compose *runs* the processes as conservation-faithful flow circuits and demonstrates the Linkage Propositions empirically (a process's CSV shows its signature). Complementary lenses on the same Troncale corpus — MP for behavior-trace V&V, bert-compose for runnable, mass-accounted dynamics.

### D. MBSE / SysML — established formal systems modeling (engineering-flavored)

**SysML** tooling — **Capella/Arcadia** (open, Eclipse), **Cameo/MagicDraw** (commercial). The established standard for *model-based systems engineering*.

**How bert-compose differs:** MBSE is engineering-document-centric (requirements, blocks, allocations), not grounded in **general** systems theory; it has no live conservation simulation and no cross-domain isomorphism claim. It models *a* system to build it, not *systemness* to understand it.

### E. Typed world-model / ontology — the operational-data axis

**Palantir Foundry** — ontology-driven operational modeling. Covered in the vault positioning doc: proves the market wants formal ontology, but proprietary, seven-figure, and not systems-theoretic.

## Closest competitor — the single best answer

- **Practically: Stella / InsightMaker** — the stock-and-flow simulator a user would literally substitute.
- **Foundationally: CatColab (Topos) and AlgebraicJulia** — the same compositional-systems thesis, expressed as category theory.

## The whitespace

Nothing else is simultaneously GST/Mobus-grounded **and** conservation-faithful-by-construction **and** tactile-for-non-engineers **and** cross-domain-via-lenses **and** typed-graph-exporting. SD tools have the simulation but no ontology; AlgebraicJulia/CatColab have the deep compositionality but not the tactility or the conservation simulation; MBSE has formal structure but no GST or live sim; Palantir has typed world models but no systems theory and is closed.

**BERT is, in effect, the first software realization of the Mobus/Klir–Bunge systems ontology** — and there is no competing software implementation of *that* tradition. The competition is all adjacent.

## The Topos relationship (ally as much as competitor)

CatColab is the most serious neighbor and the most strategically interesting. Same vision (formal + conceptual + accessible + combine-world-models), different substrate (CT logics vs GST/Mobus ontology). Topos is already in Halcyonic's orbit (Stanford/Topos parallel PhD track; ct-sandbox uses Catlab.jl). The healthy framing: **bert-compose is the GST-grounded, conservation-faithful, tactile instantiation of the same dream CatColab pursues category-theoretically** — and the two could interoperate (a BERT model as a CatColab logic; a lens as a functor) rather than merely compete.

## The real risk: mis-categorization, not being out-competed

The danger isn't a better tool; it's being filed wrong. Seen as "another stock-and-flow tool," it loses the ontology and the theorem. Seen as "a category-theory toy," it loses the tactility and the social-scientist audience. The positioning job is to hold both: *the rigor of the categorical crowd, the touchability of NetLogo, the conservation no SD tool guarantees — the convergence thesis you can run.*

## Sources

- System Dynamics Society, core software overview — https://systemdynamics.org/tools/core-software/
- Vensim (Ventana Systems) — https://vensim.com/ ; comparison of SD software — https://grokipedia.com/page/Comparison_of_system_dynamics_software
- isee systems / Stella — https://www.iseesystems.com/
- Insight Maker — https://insightmaker.com/
- AlgebraicDynamics.jl — https://github.com/AlgebraicJulia/AlgebraicDynamics.jl ; docs — https://algebraicjulia.github.io/AlgebraicDynamics.jl/dev/
- CatColab (Topos Institute) — https://github.com/ToposInstitute/CatColab ; v0.5 Sandpiper — https://topos.institute/blog/2026-03-23-catcolab-0-5-sandpiper/ ; project page — https://topos.institute/work/catcolab/
- Giammarco & Troncale, "Modeling Isomorphic Systems Processes Using Monterey Phoenix," *Systems* 6(2):18, 2018 — https://www.mdpi.com/2079-8954/6/2/18 ; Monterey Phoenix (NPS) — https://nps.edu/web/monterey-phoenix/
- SysML/MBSE: Capella/Arcadia — https://www.eclipse.org/capella/
