#!/usr/bin/env python3
"""Compile a BERT primitive-composition spec to a loadable WorldModel JSON.

The generator (general-systems-reasoner bert_generator.generate) emits the model
STRUCTURE — environment, boundaries, interfaces, interactions — but not the per-agent
behavior (archetype/primitives/agency_capacity/initial_state). This post-process
injects an `agent` block into each level-1 subsystem by matching the spec's
`subsystems` entries by name. Mirrors the documented #76 workflow.

Usage: compile_spec.py <name>-spec.json  ->  writes <name>.json beside it.
"""
import json, sys, os

def main(spec_path):
    import bert_generator
    spec = json.load(open(spec_path))
    model = json.loads(bert_generator.generate(json.dumps(spec)))

    by_name = {ss["name"]: ss for ss in spec.get("subsystems", [])}
    for s in model.get("systems", []):
        name = s.get("info", {}).get("name")
        ss = by_name.get(name)
        if ss is None:
            continue
        agent = {
            "kind": ss.get("agent_kind", "Reactive"),
            "agency_capacity": float(ss.get("agency_capacity", 0.5)),
            "primitives": ss.get("primitives", []),
        }
        if ss.get("initial_state"):
            agent["initial_state"] = ss["initial_state"]
        s["agent"] = agent

    out_path = spec_path.replace("-spec.json", ".json")
    json.dump(model, open(out_path, "w"), indent=2)
    print(f"wrote {out_path} ({len(model.get('systems', []))} systems, "
          f"{len(model.get('interactions', []))} interactions)")

if __name__ == "__main__":
    main(sys.argv[1])
