# sysml-interop — K≅2 kernel in SysML, via Gaphor

Renders the **K≅2 kernel** (system = morphism) as a real SysML Block Definition
Diagram, in the notation of Bruce McNaughton's ESE enterprise ontology. The point
is **one edge**: `System` specializes both his `System Class` *and* `Morphism`, so
his GSM (slide 18) and his Category Definitions (slide 24) become one model. See the
vault notes in `operations/sessions/2026-06-17/references/` (`bruce-gsm-k2-mapping.md`,
`k2-kernel-in-bruce-notation.md`).

This is the **export-not-import** direction: make our foundations legible in Bruce's
dialect, rather than ingesting his SysML.

## Files

| file | what |
|---|---|
| `generate.py` | builds the model with Gaphor's API, auto-lays-out (graphviz), saves `.gaphor`, renders SVG, self-tests by reloading |
| `k2-kernel.gaphor` | open in Gaphor 3.x and drag/edit |
| `k2-kernel.svg` / `.png` | previews |

## Run

**A) Zero-install, version-exact** (Gaphor's own bundled interpreter — no setup):

```bash
/Applications/Gaphor.app/Contents/MacOS/gaphor exec generate.py
```

**B) Dev venv** (readable source, REPL, autocomplete):

```bash
brew install gtk4 pygobject3 graphviz       # one-time system deps
python3 -m venv .venv && source .venv/bin/activate
pip install gaphor                           # importable library + .py source
python generate.py
```

Keep (A) as the final version-exact check; use (B) for iterating.

## How to use the API properly (the one rule)

**Build models through Gaphor's documented Python API — never hand-author or
hand-edit the `.gaphor` XML.** The XML is an internal serialization (element IDs,
canvas items, matrices); editing it by hand silently corrupts references. The
supported path is: create elements via `ElementFactory` inside a `Transaction`,
place them with `drop()`, lay out with `AutoLayout`, and persist with
`storage.save` — exactly what `generate.py` does. The version-exact API is
discovered by **introspection** (`gaphor exec` a probe that prints
`dir()`/`inspect.signature`), not assumed from memory, because names drift across
releases. The dev venv (B) is the proper environment for that — readable source +
REPL. Treat `generate.py`'s helpers (`block`, `add_prop`, `generalize`) as the
sanctioned interface; extend those rather than reaching into raw XML.

## Gaphor 3.1.0 API notes (ground-truthed by introspection)

- `Session(services=[event_manager, component_registry, element_factory, element_dispatcher, modeling_language])`; edit inside `with Transaction(event_manager):`.
- Elements: `sysml.Block`, `UML.Property` (`.type`=block ref / `.typeValue`=string / `.aggregation="composite"` for «part»), `UML.Operation`, `UML.Generalization` (`.general`=parent, `.specific`=child), `UML.Package`.
- Diagram must be a `UML:Diagram` subtype to nest under a package: `ef.create(sysml.BlockDefinitionDiagram)`, then `pkg.ownedDiagram = d`, `block.package = pkg`.
- Layout: `from gaphor.plugins.autolayout import AutoLayout; AutoLayout().layout(d)`.
- Save/load: `from gaphor.storage import save, load`. Export: `from gaphor.diagram.export import save_svg, save_png`.
- Gotchas: `gaphor.extensions.ipython` is NOT in the `.app` bundle; multiplicity wants `ValueSpecification` objects, not strings (omitted here, cosmetic).

## TODO / next

- Toggle block compartments on so attributes/parts show on open.
- Second diagram: `System` with just its two specialization edges (the pure "one-edge" shot).
- Reproduce one of Bruce's GSM slides as `.gaphor` so his model is manipulable in-tool.
- North star: a BERT JSON ↔ SysML v2 textual (KerML) translator (the real bridge).
