# BERT Compose — UI/UX vision

What would make Stella, Vensim, NetLogo, and Mesa jealous? Not more features —
a different *relationship* between the modeler and the model. Those tools all
share one flaw: a gap between describing a system and being in causal contact
with it. You draw/code, then compile/run/debug as a separate act, and errors
live in the gap. Compose has no gap and no error states — both are theory, not
polish. The design job is to protect and deepen that, never to bury it under
chrome.

Principle: **every pixel either shows the live system or gets out of the way.**
Calm warm-paper canvas (the Halcyonic shell), color reserved for meaning
(substance identity, state level, the one amber warning), motion reserved for
truth (flow only moves when flow is real).

## The five things that win

### 1. Flow you can read at a glance — make conservation visible, not just true
Today: a number and a moving dot per wire. Jealousy-grade:
- **Wire thickness = magnitude** (log-scaled), so a glance reads the whole
  circuit's balance — fat in, thin out means something's accumulating.
- **Pulse density = rate**, color = substance. A conserved split *looks*
  conserved: one fat wire becoming two half-width wires.
- **Stock as a filling vessel**, not a bar — buffers visibly hold, and a
  buffer near overflow or empty pulses at its rim. The state of the system is
  legible from across the room. Stella draws stocks; none of them *breathe*.

### 2. The inline sparkline — every node carries its own history
Mesa/NetLogo make you wire up a separate plot window. Compose should paint a
**tiny live sparkline under each node** (the last ~40 ticks of its activity or
storage), so the dynamics are *on the diagram*, not in a side panel. The whole
canvas becomes a dashboard of itself. Click a node → the sparkline expands into
the right panel with axes. This is the single highest-jealousy feature: the
model and its behavior are the same picture.

### 3. Direct-manipulation parameters — drag the system, watch it respond live
Vensim has sliders in a separate "SyntheSim" mode. Compose should let you drag
a parameter *while it runs* and watch the loop respond in real time — pull the
source rate up and see the homeostat fight back, live. Even better: **drag on
the node itself** (vertical drag on a buffer = release rate; on a source =
emission). The parameter space becomes something you *play* like an
instrument, not a form you fill.

### 4. The lens switch — one model, every domain (nobody else has this)
The killer differentiator, straight from K≅2. A **lens selector** aligned to
the halcyonic.systems pillars (Systems / Political Economy / Neuromorphics /
Protocol Science / Ecology) that *renames and re-skins* the same running
circuit without touching its dynamics: Sensing→"monitor"/"oracle"/"receptor",
Buffering→"registry"/"mempool"/"membrane". Switch lenses mid-run and watch the
*same regulation* become a quorum throttle, a difficulty adjustment, a
spiking neuron. No other modeling tool can show you that two domains are the
same system because no other tool has the proof underneath. This is the
"jealous" moment — it's not a better SD tool, it's a category they don't have.

### 5. Provenance on demand — the proof is one hover away
Every primitive, every bond, every "no error" can cite its license. Hover a
node → "Buffering: conservative stock, transfer fn verified
(python/agents.py), Mobus Ch.3." Hover the canvas → "valid by construction:
composition is unconditional, Systems/Mobus/Composition.lean." Nobody trusts a
black-box simulator; Compose can make rigor *ambient* — always available,
never in the way. The honesty is the brand.

## Smaller touches that compound
- **Snap-to-grid + auto-route wires** (orthogonal, not straight-through-nodes).
- **Palette items show their I/O substance** as little colored ports, so you
  pre-see what connects to what.
- **Drag-from-port creates a node** ("drag into empty space → pick primitive"),
  so building is one gesture, not add-then-wire.
- **Time scrubber** on the recorded run — drag back through ticks, the canvas
  reconstructs that moment's state (the CSV already holds it).
- **Equilibrium glow**: when a regulated loop settles, it gets a soft steady
  halo — the system *telling you* it found homeostasis.
- **"Why is this amber?"** — the substance/unit warnings already shipped;
  make them one-click-fixable ("change source to Message" / "insert
  transducer").

## What to deliberately NOT do (the discipline)
- No code panel, no equation editor as the *primary* surface — the diagram IS
  the model. (Equations available on hover for the curious; never required.)
- No modal config dialogs — everything inline, everything live.
- No individuated-agent simulation here — that's Mesa's job; Compose is
  aggregate flow. Resisting this keeps the tool honest and the canvas calm.
- No error states. Ever. Composition is unconditional; the worst that happens
  is an amber "this flow is ignored, here's why."

## The one-sentence pitch
Stella shows you a diagram, Vensim shows you a model, Mesa shows you a program —
**Compose lets you reach in and hold the system while it runs, in any domain,
with the proof one hover away.**
