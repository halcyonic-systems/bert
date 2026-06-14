#!/usr/bin/env python3
"""Spike: BERT JSON -> SysML v2 textual notation (structural slice).

One-way, lossy by design. Fields with no SysML v2 home are emitted as
// [BERT-only] comments so the gap is visible rather than silently dropped.
Not production: a real emitter lives in Rust alongside the bert-typedb transpiler.
"""
import json, re, sys

SUBSTANCE_ITEM = {"Energy": "Energy", "Material": "Material", "Message": "Message"}


def ident(name, fallback):
    s = re.sub(r"[^0-9a-zA-Z]+", " ", name or "").strip()
    if not s:
        return fallback
    parts = s.split()
    out = parts[0].lower() + "".join(p.capitalize() for p in parts[1:])
    return out if out[0].isalpha() else fallback


def type_ident(name, fallback):
    s = re.sub(r"[^0-9a-zA-Z]+", " ", name or "").strip()
    if not s:
        return fallback
    return "".join(p.capitalize() for p in s.split())


def main(path):
    m = json.load(open(path))
    env = m["environment"]
    systems = m["systems"]
    inter = m["interactions"]

    by_id = {s["info"]["id"]: s for s in systems}
    roots = [s for s in systems if s["parent"] == "E-1"]
    # interface processors are boundary-routing artifacts -> fold into the port
    procs = {s["info"]["id"] for s in systems if s["boundary"].get("parent_interface")}

    # name lookup for every id (systems, interfaces, sources, sinks)
    name = {}
    for s in systems:
        name[s["info"]["id"]] = s["info"]["name"]
        for itf in s["boundary"]["interfaces"]:
            name[itf["info"]["id"]] = itf["info"]["name"]
    for e in env["sources"] + env["sinks"]:
        name[e["info"]["id"]] = e["info"]["name"]

    pkg = type_ident(roots[0]["info"]["name"], "Model") if roots else "Model"
    L = [f"package {pkg} {{", ""]
    L.append("    // ===== substance categories -> item defs =====")
    subs = sorted({i["substance"]["type"] for i in inter})
    for s in subs:
        L.append(f"    item def {SUBSTANCE_ITEM.get(s, s)};")
    L.append("")

    dropped = []  # [BERT-only] semantics with no SysML home

    def port_dir(itf):
        return "in " if itf["type"] == "Import" else "out"

    # ---- root system part def(s) ----
    for r in roots:
        rid = r["info"]["id"]
        tname = type_ident(r["info"]["name"], "RootSystem")
        L.append(f"    part def {tname} {{")
        if r["info"]["description"]:
            L.append(f'        doc /* {r["info"]["description"][:90]}... */')
        # ports from boundary interfaces
        L.append("        // --- ports (BERT boundary interfaces) ---")
        port_name = {}
        for itf in r["boundary"]["interfaces"]:
            iid = itf["info"]["id"]
            pn = ident(itf["info"]["name"], "port" + iid.replace(".", "_"))
            port_name[iid] = pn
            item = "Message"
            proto = f'  protocol="{itf["protocol"]}"' if itf["protocol"] else ""
            L.append(f"        {port_dir(itf)} port {pn} : {item};   // {iid}{proto}")
        # subsystem usages (real components only; skip interface processors)
        kids = [s for s in systems if s["parent"] == rid and s["info"]["id"] not in procs]
        L.append("        // --- subsystems (BERT internal components) ---")
        for c in kids:
            cid = c["info"]["id"]
            ct = type_ident(c["info"]["name"], "Sub" + cid.replace(".", "_"))
            cu = ident(c["info"]["name"], "sub" + cid.replace(".", "_"))
            note = ""
            if c.get("archetype"):
                note = f"   // [BERT-only] archetype={c['archetype']}"
                dropped.append(f"archetype on {cid} ({c['archetype']})")
            L.append(f"        part {cu} : {ct};{note}")
        if procs:
            folded = ", ".join(sorted(procs))
            L.append(f"        // interface processors fold into ports above: {folded}")
        # internal flows (both endpoints inside this root, real components)
        L.append("        // --- internal flows ---")
        for i in inter:
            src, snk = i["source"], i["sink"]
            if src in by_id and snk in by_id and src not in procs and snk not in procs:
                if by_id[src]["parent"] == rid and by_id[snk]["parent"] == rid:
                    su = ident(name.get(src, src), src)
                    ku = ident(name.get(snk, snk), snk)
                    it = SUBSTANCE_ITEM.get(i["substance"]["type"], "Message")
                    L.append(
                        f"        flow of {it} from {su} to {ku};"
                        f"   // {i['info']['id']} {i['info']['name']} [usability={i['usability']}]"
                    )
                    dropped.append(f"usability on {i['info']['id']} ({i['usability']})")
        L.append("    }")
        L.append("")

    # ---- part defs for subsystems ----
    L.append("    // ===== component definitions =====")
    for c in systems:
        if c["parent"] == "E-1" or c["info"]["id"] in procs:
            continue
        ct = type_ident(c["info"]["name"], "Sub")
        L.append(f"    part def {ct} {{")
        if c["info"]["description"]:
            L.append(f'        doc /* {c["info"]["description"][:80]}... */')
        cx = c["complexity"]
        if isinstance(cx, dict) and "Complex" in cx:
            f = cx["Complex"]
            L.append(f"        // [BERT-only] adaptable={f['adaptable']} evolveable={f['evolveable']}")
        L.append("    }")
    L.append("")

    # ---- environment context: wire sources/sinks to root ports ----
    L.append("    // ===== environment context =====")
    L.append("    part environment {")
    for e in env["sources"] + env["sinks"]:
        eu = ident(e["info"]["name"], e["info"]["id"])
        L.append(f"        part {eu};   // {e['info']['id']} {e['type']}")
    for r in roots:
        ru = ident(r["info"]["name"], "system")
        L.append(f"        part {ru} : {type_ident(r['info']['name'],'RootSystem')};")
    # external flows
    for i in inter:
        src, snk = i["source"], i["sink"]
        ext = (src.startswith("Src") or snk.startswith("Snk"))
        if not ext:
            continue
        su = ident(name.get(src, src), src)
        ku = ident(name.get(snk, snk), snk)
        # qualify with port if it touches the root system
        if snk in by_id and i.get("sink_interface"):
            ku = f"{ident(by_id[snk]['info']['name'],'system')}.{ident(name.get(i['sink_interface'],''),'port')}"
        if src in by_id and i.get("source_interface"):
            su = f"{ident(by_id[src]['info']['name'],'system')}.{ident(name.get(i['source_interface'],''),'port')}"
        it = SUBSTANCE_ITEM.get(i["substance"]["type"], "Message")
        L.append(
            f"        flow of {it} from {su} to {ku};"
            f"   // {i['info']['id']} {i['info']['name']} [usability={i['usability']}]"
        )
    L.append("    }")
    L.append("")

    # ---- the lossy ledger ----
    L.append("    // ===== [BERT-only] semantics with NO SysML v2 home (dropped/annotated) =====")
    L.append("    // structural fields present on every system but unrepresentable:")
    L.append("    //   member_autonomy, time_constant, boundary.porosity, boundary.perceptive_fuzziness")
    forces = [i for i in inter if i.get("type") == "Force"]
    L.append(f"    //   Interaction type=Force (influence-without-transfer): {len(forces)} in this model")
    agents = [s["info"]["id"] for s in systems if s.get("agent")]
    if agents:
        L.append(f"    //   Agent behavioral models (kind/agency_capacity/primitives): {', '.join(agents)}")
    L.append("    //   usability (Resource/Product/Waste/Disruption) on every flow -> see inline comments")
    L.append("}")

    print("\n".join(L))


if __name__ == "__main__":
    main(sys.argv[1])
