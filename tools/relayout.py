#!/usr/bin/env python3
"""Re-layout BERT model positions using N-gon algorithm.

Surgically updates level-1 subsystem positions and flow angles while
preserving all structure: names, processors, children, interfaces,
flow connectivity.

Level-2 children are NOT moved — their positions are relative to their
parent in the JSON, so they follow automatically via Bevy's scene graph.

Usage:
    python tools/relayout.py model.json                    # in-place
    python tools/relayout.py model.json -o clean.json      # to new file
    python tools/relayout.py model.json --dry-run           # show changes
    python tools/relayout.py model.json --flows-only        # only recompute flow angles
"""

import argparse
import json
import math
import sys


def compute_ngon_positions(n, radius=190.0):
    if n == 0:
        return []
    if n == 1:
        return [(0.0, 0.0)]
    if n == 2:
        return [(-125.0, 0.0), (125.0, 0.0)]
    if n == 3:
        return [(-125.0, -95.0), (125.0, -95.0), (0.0, 160.0)]
    if n == 4:
        return [(0.0, -160.0), (-160.0, 0.0), (160.0, 0.0), (0.0, 160.0)]
    positions = []
    for i in range(n):
        angle = -math.pi / 2 + i * (2 * math.pi / n)
        x = round(radius * math.cos(angle), 2)
        y = round(radius * math.sin(angle), 2)
        positions.append((x, y))
    return positions


def angle_from_to(src, dst):
    dx = dst[0] - src[0]
    dy = dst[1] - src[1]
    if dx == 0.0 and dy == 0.0:
        return 0.0
    return math.atan2(dy, dx)


def is_processor(system):
    radius = system.get("radius", 999)
    parent_iface = system.get("boundary", {}).get("parent_interface")
    return radius <= 12.0 and parent_iface is not None


def get_pos(system):
    t = system.get("transform", {}).get("translation", [0, 0])
    return (t[0], t[1])


def set_pos(system, x, y):
    if "transform" not in system:
        system["transform"] = {"translation": [0, 0], "rotation": 0.0}
    system["transform"]["translation"] = [round(x, 2), round(y, 2)]


def recompute_flow_angles(model):
    """Recompute endpoint_offset angles for all flows based on current positions."""
    systems = model.get("systems", [])
    interactions = model.get("interactions", [])

    pos_lookup = {}
    for s in systems:
        pos_lookup[s["info"]["id"]] = get_pos(s)
    env = model.get("environment", {})
    for src in env.get("sources", []):
        pos_lookup[src["info"]["id"]] = get_pos(src)
    for snk in env.get("sinks", []):
        pos_lookup[snk["info"]["id"]] = get_pos(snk)

    count = 0
    for flow in interactions:
        src_id = flow.get("source", "")
        snk_id = flow.get("sink", "")
        src_pos = pos_lookup.get(src_id)
        snk_pos = pos_lookup.get(snk_id)

        if src_pos and snk_pos:
            start_angle = angle_from_to(src_pos, snk_pos)
            end_angle = angle_from_to(snk_pos, src_pos)
            flow["endpoint_offset"] = {
                "start_angle": round(start_angle, 6),
                "end_angle": round(end_angle, 6),
            }
            count += 1
    return count


def relayout_positions(model, dry_run=False):
    """Re-layout level-1 regular subsystems using N-gon algorithm."""
    systems = model.get("systems", [])

    regular = []
    for s in systems:
        level = s["info"].get("level", 0)
        parent = s.get("parent", "")
        if level == 1 and parent == "S0" and not is_processor(s):
            regular.append(s)

    if not regular:
        print("  No regular subsystems found to re-layout.")
        return 0

    regular.sort(key=lambda s: math.atan2(get_pos(s)[1], get_pos(s)[0]))

    new_positions = compute_ngon_positions(len(regular))
    changes = 0

    for s, new_pos in zip(regular, new_positions):
        old_pos = get_pos(s)
        dx = new_pos[0] - old_pos[0]
        dy = new_pos[1] - old_pos[1]

        if abs(dx) < 0.5 and abs(dy) < 0.5:
            continue

        changes += 1
        name = s["info"].get("name", s["info"]["id"])
        if dry_run:
            print(f"  {name}: ({old_pos[0]:.1f}, {old_pos[1]:.1f}) → ({new_pos[0]:.1f}, {new_pos[1]:.1f})")
        else:
            set_pos(s, new_pos[0], new_pos[1])

    return changes


def main():
    parser = argparse.ArgumentParser(description="Re-layout BERT model positions")
    parser.add_argument("input", help="Path to BERT model JSON")
    parser.add_argument("-o", "--output", help="Output path (default: overwrite input)")
    parser.add_argument("--dry-run", action="store_true", help="Show changes without writing")
    parser.add_argument("--flows-only", action="store_true",
                        help="Only recompute flow angles, don't move subsystems")
    args = parser.parse_args()

    with open(args.input) as f:
        model = json.load(f)

    sys_count = len(model.get("systems", []))
    int_count = len(model.get("interactions", []))
    name = "unknown"
    for s in model.get("systems", []):
        if s["info"].get("level", -1) == 0:
            name = s["info"].get("name", "unknown")
            break

    print(f"Model: {name} ({sys_count} systems, {int_count} interactions)")

    if args.flows_only:
        if args.dry_run:
            print("  --flows-only with --dry-run: would recompute all flow angles.")
            return
        flow_count = recompute_flow_angles(model)
        print(f"  Recomputed {flow_count} flow endpoint angles.")
    else:
        changes = relayout_positions(model, dry_run=args.dry_run)
        if args.dry_run:
            if changes == 0:
                print("  No position changes needed.")
            else:
                print(f"  {changes} subsystem(s) would be moved.")
            return

        if changes > 0:
            recompute_flow_angles(model)
            print(f"  Moved {changes} subsystem(s) + recomputed flow angles.")
        else:
            print("  No position changes needed.")
            return

    out_path = args.output or args.input
    with open(out_path, "w") as f:
        json.dump(model, f, indent=2)
        f.write("\n")

    verify_sys = len(model.get("systems", []))
    verify_int = len(model.get("interactions", []))
    assert verify_sys == sys_count, f"System count changed: {sys_count} → {verify_sys}"
    assert verify_int == int_count, f"Interaction count changed: {int_count} → {verify_int}"
    print(f"  Written to {out_path}")
    print(f"  Verified: {verify_sys} systems, {verify_int} interactions (unchanged)")


if __name__ == "__main__":
    main()
