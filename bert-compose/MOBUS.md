# Grounding in Mobus

What in bert-compose comes straight from Mobus, what's our extension, and what's a known gap — sourced from the actual text, not general knowledge.

**Sources.** The vault `operations/systems-science/mobus/` is **Mobus 2022** (*Systems Science: Theory, Analysis, Modeling, and Design*). Mobus & Kalton 2015 (*Principles of Systems Science*) is the earlier, separate work (Zotero). The **8-tuple is Mobus's post-2022 "book-revisions,"** formalized and machine-checked in `systems-science-foundations/Systems/Mobus/Tuple.lean`; the BERT element table is `bert/docs/mobus-reference.md`.

## The primitives ARE Mobus's atomic work processes

The palette is Mobus 2022 **Figs 3.18–3.19** verbatim:

- **Fig 3.18** (four simple): **Combining** (two substances → a product + waste), **Splitting** (one flow → two products + loss), **Impeding** (slow a flow *"with a consequent back-pressure"*), **Buffering** (*"smooth out the flow volumes over time"*).
- **Fig 3.19** (additional): **Copying** (template + blank → original + copy + waste), **Propelling** (*"push a substance against a gradient, like a pump"*), **Sensing** (responds to a modulated force, emits a low-power modulated signal), **Amplifying** (adds power to a weak modulated input), **Modulating** (applies a signal to a substance flow).

Source: *"All system components recapitulate the fundamental work processes depicted in Figs 3.18 and 3.19"* (Ch. 3 conclusion).

## The conservation ledger IS Mobus's waste heat (Fig 3.17)

Our `dissipated` is not an artifact — it's the Second Law. Fig 3.17: *"Work processes require high-potential energy… some of the energy… is transformed to waste heat. Material transformations involve some material waste; energy transformations produce a greater proportion of waste heat."*

That is exactly why the domain examples differ on the **conservation chart**: a Material/value system (budget, tokens) loses little; an Energy system (neuron, meadow) loses most. The energy-vs-value distinction is Mobus, made runnable — see the substance trichotomy (Energy/Material conserve and degrade; Message copies, never ledgered), also Mobus (Ch. 3, "Three Substance Types").

## The 8-tuple and the parameter license

`S = ⟨C, N, E, G, B, T, H, Δt⟩` (`Tuple.lean`, "Book-revisions Eq. 1"):
C components · N internal network · E = ⟨O,M⟩ environment · G external (boundary-crossing) flows · B = ⟨P,I⟩ boundary · T transforms · H history/knowledge · Δt time scale.

The Lean DESIGN note is load-bearing for *this tool's design*: **T, H, Δt are parametric / domain-specific by intent** — the structural constraints live only in C/N/E/G/B. So **there is no Mobus-canonical per-primitive parameter set**; bert-compose choosing transfer-function parameters (a Source's rate, a Sensing gain, an efficiency) fills the slot Mobus deliberately leaves open. It's licensed by the ontology, not a deviation.

## What's faithful · what's ours · what's a gap

- **Faithful:** the primitives (Figs 3.18–3.19), conservation/waste heat (Fig 3.17), the substance trichotomy, the error-sensing homeostat (Fig 4.12: sensing → inverter → modulator → comparator, computing reference − measured), composition as ontogenic linkage of work processes (Ch. 4 Fig 4.12).
- **Ours (extensions):** the lenses (presentation functors over the same model), the named-substance dictionary, the conservation chart, the gradient flow-mode (grounded separately in Mobus Ch. 4 "fields are generalized flows").
- **Known gaps (roadmap, [bert#87](https://github.com/halcyonic-systems/bert/issues/87)):** the push model has **no back-pressure**, yet Mobus's *Impeding is defined with* back-pressure — so a throttled valve sheds blocked flow instead of backing it up (this is why a budget can appear to "lose" money). Plus stock **capacity**, explicit **setpoint/reference**, **maintenance energy** (Fig 3.17's upkeep cost), and Buffering **time-constant** smoothing. **Threshold** is a candidate too but is neuroscience, *not* Mobus — don't attribute it to him.

## Don't

- Don't "correct" the 8-tuple to a 7-tuple from the 2022 book chapters — the 8-tuple is the authoritative revision (`Tuple.lean`).
- Don't attribute the **threshold** parameter to Mobus.
- Don't treat ledger `dissipated` as a leak to eliminate — for Energy it's the physics (Fig 3.17).
