#!/usr/bin/env python3
"""
Generate real .gaphor (Gaphor 3.1.0) SysML Block Definition diagrams for the
K≅2 kernel, in Bruce McNaughton's notation.

Two ways to run (both work):

  # A) zero-install, version-exact (Gaphor's bundled interpreter):
  /Applications/Gaphor.app/Contents/MacOS/gaphor exec .../sysml-interop/generate.py

  # B) dev venv (readable source, REPL):  see README.md
  python generate.py

Output (same folder): k2-kernel.gaphor + previews k2-kernel.svg / .png

To play: open the .gaphor in Gaphor, drag boxes around, edit compartments.
To extend: add blocks/edges in build_kernel() below and re-run.
"""
import os

from gaphor.application import Session
from gaphor.core.modeling import Diagram, ElementFactory
from gaphor import UML
from gaphor.SysML import sysml
from gaphor.diagram.drop import drop
from gaphor.storage import save, load
from gaphor.transaction import Transaction

OUT_DIR = os.path.dirname(os.path.abspath(__file__))

# ---------------------------------------------------------------- helpers ----

def new_session():
    s = Session(services=[
        "event_manager", "component_registry", "element_factory",
        "element_dispatcher", "modeling_language",
    ])
    return s, s.get_service("element_factory"), s.get_service("event_manager"), \
        s.get_service("modeling_language")


def block(ef, name):
    b = ef.create(sysml.Block)
    b.name = name
    return b


def add_prop(ef, owner, name, type_block=None, type_value=None, aggregation=None):
    """Add a compartment property. type_block => 'name: BlockName'; else type_value string.

    (Multiplicity is intentionally omitted: gaphor stores it as ValueSpecification
    objects, not strings. It's cosmetic; add later if wanted. See README.)
    """
    p = ef.create(UML.Property)
    p.name = name
    if type_block is not None:
        try:
            p.type = type_block
        except Exception as e:
            print("  ! type set failed for", name, repr(e))
    elif type_value is not None:
        p.typeValue = type_value
    if aggregation:
        try:
            p.aggregation = aggregation       # 'composite' = «part»
        except Exception as e:
            print("  ! aggregation failed for", name, repr(e))
    owner.ownedAttribute = p
    return p


def add_op(ef, owner, name, type_value=None):
    op = ef.create(UML.Operation)
    op.name = name
    if type_value:
        try:
            op.typeValue = type_value
        except Exception:
            pass
    owner.ownedOperation = op
    return op


def generalize(ef, child, parent):
    """child --|> parent  (specialization, SysML ':>')."""
    g = ef.create(UML.Generalization)
    g.general = parent
    g.specific = child
    return g


# ---------------------------------------------------------------- model ------

def build_kernel(ef):
    """The K≅2 kernel as a SysML bdd. Returns the Diagram."""
    pkg = ef.create(UML.Package)
    pkg.name = "Systems Kernel"

    d = ef.create(sysml.BlockDefinitionDiagram)   # a real SysML bdd, like Bruce's slides
    d.name = "K≅2 Kernel"
    pkg.ownedDiagram = d          # nest the diagram under the package (clean model tree)

    # --- the category layer (Bruce's slide 24, promoted to primary) ---
    Object = block(ef, "Object")

    Morphism = block(ef, "Morphism")
    add_prop(ef, Morphism, "source", type_block=Object)   # = input  side of a System Interaction
    add_prop(ef, Morphism, "target", type_block=Object)   # = output side of a System Interaction

    Functor = block(ef, "Functor")

    # --- Bruce's GSM, with the one new edge ---
    SystemClass = block(ef, "System Class")               # his parent (slide 18)

    System = block(ef, "System")
    add_prop(ef, System, "name", type_value="System Name")
    add_prop(ef, System, "purpose", type_value="System Purpose")
    add_prop(ef, System, "sysType", type_value="System Types")
    add_op(ef, System, "function", type_value="System Function")   # the map itself (slide 18 actions)
    add_prop(ef, System, "interaction", type_block=System, aggregation="composite")  # I/O across boundary
    add_prop(ef, System, "subsystem", type_block=System, aggregation="composite")    # composition of sub-morphisms
    add_prop(ef, System, "relation", type_block=System, aggregation="composite")     # = CatComposite (slide 24)

    Lens = block(ef, "Lens")

    # --- the specialization edges (the actual claim) ---
    g_sys_morph = generalize(ef, System, Morphism)        # System :> Morphism   <-- THE ONE EDGE
    g_sys_class = generalize(ef, System, SystemClass)     # System :> System Class (his original)
    g_lens_func = generalize(ef, Lens, Functor)           # Lens :> Functor

    # nest all blocks under the package so the model tree (and CLI export) is clean
    for b in (Object, Morphism, Functor, SystemClass, System, Lens):
        b.package = pkg

    # --- lay it on the canvas ---
    for el in (Object, Morphism, Functor, SystemClass, System, Lens,
               g_sys_morph, g_sys_class, g_lens_func):
        drop(el, d, 0, 0)

    # auto-layout via Gaphor's bundled graphviz (gaphor.extensions.ipython is NOT
    # shipped in the .app, so use the AutoLayout engine directly)
    try:
        from gaphor.plugins.autolayout import AutoLayout
        AutoLayout().layout(d)
        print("  layout: auto (graphviz)")
    except Exception as e:
        print("  ! auto-layout failed, manual grid:", repr(e))
        grid = [(40, 40), (320, 40), (600, 40), (40, 320), (320, 320), (600, 320)]
        for el, (x, y) in zip((Object, Morphism, Functor, SystemClass, System, Lens), grid):
            for pres in el.presentation:
                pres.matrix.translate(x, y)
    return d


# ---------------------------------------------------------------- run --------

def main():
    session, ef, em, ml = new_session()
    with Transaction(em):
        build_kernel(ef)

    gaphor_path = os.path.join(OUT_DIR, "k2-kernel.gaphor")
    with open(gaphor_path, "w") as f:
        save(f, ef)
    print("saved:", gaphor_path, os.path.getsize(gaphor_path), "bytes")

    # preview SVG (in-process; avoids the CLI's package-owner walk)
    try:
        from gaphor.diagram.export import save_svg
        d = next(iter(ef.select(Diagram)))
        svg_path = os.path.join(OUT_DIR, "k2-kernel.svg")
        save_svg(svg_path, d)
        print("preview:", svg_path, os.path.getsize(svg_path), "bytes")
    except Exception as e:
        print("  ! svg preview skipped:", repr(e))

    # ---- self-test: reload into a fresh factory and assert ----
    ef2 = ElementFactory()
    with open(gaphor_path) as f:
        load(f, ef2, ml)
    blocks = sorted(b.name for b in ef2.select(sysml.Block) if b.name)
    diagrams = [d.name for d in ef2.select(Diagram)]
    gens = list(ef2.select(UML.Generalization))
    print("RELOAD OK -> blocks:", blocks)
    print("RELOAD OK -> diagrams:", diagrams)
    print("RELOAD OK -> generalization edges:", len(gens))
    assert "System" in blocks and "Morphism" in blocks, "missing core blocks"
    assert len(gens) == 3, "expected 3 specialization edges"
    print("SELF-TEST PASSED")


if __name__ == "__main__":
    main()
