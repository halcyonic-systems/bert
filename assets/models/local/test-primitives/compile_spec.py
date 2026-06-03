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

    # Wire export boundary flows through their interface processor (has_processor:true).
    # The generator produces the valid BERT structure — root boundary interface,
    # an interface-processor subsystem claiming it (parent_interface), a routing flow
    # target_subsystem -> processor, and the external flow root -> sink referencing the
    # interface. Two post-process touches make it *simulate* the same boundary outflow:
    #   1. the external flow originates from the processor (processor -> sink), so the
    #      processor relays the drained mass out (instead of root, which is no agent);
    #   2. the routing flow (target_subsystem -> processor) carries the export rate, so
    #      the source compartment actually drains at that rate.
    # This keeps the interface referenced + processor-claimed (no orphan-interface or
    # missing-processor warnings) while the sim drains the compartment across the boundary.
    interfaces = next((s for s in model["systems"] if s["info"]["level"] == 0),
                      {"boundary": {"interfaces": []}})["boundary"]["interfaces"]
    iface_id_by_name = {itf["info"]["name"]: itf["info"]["id"] for itf in interfaces}
    proc_id_by_iface = {  # interface id -> processor subsystem id that claims it
        s["boundary"]["parent_interface"]: s["info"]["id"]
        for s in model["systems"] if s.get("boundary", {}).get("parent_interface")
    }
    for ef in spec.get("external_flows", []):
        iface_id = iface_id_by_name.get(ef.get("interface"))
        proc_id = proc_id_by_iface.get(iface_id)
        if not proc_id:
            continue
        rate = ef.get("amount")
        for ix in model["interactions"]:
            # routing flow into the processor: carry the export rate so the source drains
            if ix["sink"] == proc_id and rate is not None:
                ix["amount"] = str(rate)
            # the external flow: emit from the processor out to the sink
            if ix["info"]["name"] == ef["name"]:
                ix["source"] = proc_id
                if rate is not None:
                    ix["amount"] = str(rate)

    out_path = spec_path.replace("-spec.json", ".json")
    json.dump(model, open(out_path, "w"), indent=2)
    print(f"wrote {out_path} ({len(model.get('systems', []))} systems, "
          f"{len(model.get('interactions', []))} interactions)")

if __name__ == "__main__":
    main(sys.argv[1])
