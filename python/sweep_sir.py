#!/usr/bin/env python3
"""Parameter sweep over the SIR primitive composition — surfaces the epidemic threshold.

The SIR model (S/I/R Buffering stocks + an I-Sensor whose non-draining observation read
gates the infection transfer) has no stochasticity: each run is deterministic. Variety in
the *dynamics* comes from the parameters, not from a seed. Sweeping the infection gain k
(the I-Sensor's agency_capacity, a beta proxy) while holding recovery fixed reveals a sharp
epidemic threshold — an R0=1 bifurcation that emerges from the wiring, not from code:

  - below a critical k: the seed infecteds recover before spreading; the epidemic fizzles
    (attack rate ~ a few %).
  - above it: the epidemic takes off and burns through the whole susceptible pool
    (attack rate -> 100%).

Dependency-free core (sweep + CSV + table). Plots only if matplotlib is importable.

Usage:
  ./venv/bin/python sweep_sir.py                         # default k sweep, writes CSV (+PNG if mpl)
  ./venv/bin/python sweep_sir.py --param recovery        # sweep the recovery rate instead
  ./venv/bin/python sweep_sir.py --out-csv /tmp/s.csv --out-png /tmp/s.png
"""
import argparse
import csv

import pandas as pd

from model import BertModel


def _system(name, primitive, agency_capacity=0.5):
    return {
        "bert_id": f"sweep:{name}", "display_name": name, "archetype": "Agent",
        "time_constant": "Second", "system_level": 1, "complexity_kind": "Atomic",
        "agent_kind": "Reactive", "agency_capacity": agency_capacity, "primitives": [primitive],
    }


def _flow(fid, src, snk, substance="Energy", amount=0.0, usability="Resource", observation=False):
    return {
        "bert_id": f"sweep:{fid}", "display_name": fid, "substance_type": substance,
        "usability": usability, "interaction_type": "Flow", "amount": amount,
        "source_id": f"sweep:{src}", "sink_id": f"sweep:{snk}", "observation": observation,
    }


def run_once(k, infection, recovery, s0, i0, steps):
    """Run one closed SIR realization; return (peak_I, S_final, R_final, attack_rate)."""
    systems = [
        _system("S", "Buffering"), _system("I", "Buffering"),
        _system("R", "Buffering"), _system("ISensor", "Sensing", agency_capacity=k),
    ]
    interactions = [
        _flow("infection", "S", "I", substance="Energy", amount=infection),
        _flow("observe_I", "I", "ISensor", substance="Energy", amount=0.0, observation=True),
        _flow("control", "ISensor", "S", substance="Message", amount=0.0),
        _flow("recovery", "I", "R", substance="Energy", amount=recovery),
    ]
    model = BertModel(pd.DataFrame(systems), pd.DataFrame(interactions),
                      seed=42, update_mode="synchronous", perturbations={})
    agents = {a.bert_id: a for a in model.agents}
    agents["sweep:S"].state["storage"] = s0
    agents["sweep:I"].state["storage"] = i0

    peak_i = i0
    for _ in range(steps):
        model.step()
        peak_i = max(peak_i, agents["sweep:I"].state["storage"])
    s_final = agents["sweep:S"].state["storage"]
    r_final = agents["sweep:R"].state["storage"]
    return peak_i, s_final, r_final, (s0 - s_final) / s0


def sweep(values, param, base_k, infection, recovery, s0, i0, steps):
    rows = []
    for v in values:
        kw = {"k": base_k, "infection": infection, "recovery": recovery}
        kw[param] = v
        peak_i, s_final, r_final, attack = run_once(kw["k"], kw["infection"], kw["recovery"],
                                                    s0, i0, steps)
        rows.append({param: v, "peak_I": peak_i, "S_final": s_final,
                     "R_final": r_final, "attack_rate": attack})
    return rows


def main():
    p = argparse.ArgumentParser(description="SIR parameter sweep — epidemic threshold")
    p.add_argument("--param", default="k", choices=["k", "infection", "recovery"],
                   help="parameter to sweep (default: k, the infection gain / beta proxy)")
    p.add_argument("--values", type=float, nargs="+", default=None,
                   help="explicit values to sweep; default is a log-spaced grid for k")
    p.add_argument("--base-s", type=float, default=10.0, help="max infection transfer rate")
    p.add_argument("--recovery", type=float, default=1.0, help="recovery rate (gamma)")
    p.add_argument("--base-k", type=float, default=0.1, help="infection gain when not swept")
    p.add_argument("--s0", type=float, default=100.0)
    p.add_argument("--i0", type=float, default=5.0)
    p.add_argument("--steps", type=int, default=300)
    p.add_argument("--out-csv", default="/tmp/sir-sweep.csv")
    p.add_argument("--out-png", default=None, help="write a plot here (needs matplotlib)")
    args = p.parse_args()

    if args.values is not None:
        values = args.values
    elif args.param == "k":
        values = [0.002, 0.004, 0.005, 0.006, 0.0065, 0.007, 0.0075, 0.008, 0.009,
                  0.01, 0.015, 0.02, 0.03, 0.05, 0.08, 0.15, 0.25]
    else:
        values = [round(0.5 * i, 2) for i in range(1, 21)]

    rows = sweep(values, args.param, args.base_k, args.base_s, args.recovery,
                 args.s0, args.i0, args.steps)

    label = {"k": "infection gain k", "infection": "max infection rate", "recovery": "recovery rate"}[args.param]
    print(f"SIR sweep over {label} (S0={args.s0:.0f}, I0={args.i0:.0f}, "
          f"base_S={args.base_s:.0f}, recovery={args.recovery:.1f})\n")
    print(f"{args.param:>8} {'peak_I':>7} {'S_final':>8} {'R_final':>8} {'attack':>7}")
    for r in rows:
        bar = "#" * int(r["attack_rate"] * 40)
        print(f"{r[args.param]:>8.4f} {r['peak_I']:>7.1f} {r['S_final']:>8.1f} "
              f"{r['R_final']:>8.1f} {r['attack_rate'] * 100:>5.0f}%  {bar}")

    with open(args.out_csv, "w", newline="") as f:
        w = csv.DictWriter(f, fieldnames=[args.param, "peak_I", "S_final", "R_final", "attack_rate"])
        w.writeheader()
        w.writerows(rows)
    print(f"\nwrote {args.out_csv}")

    if args.out_png:
        try:
            plot(rows, args.param, label, args.out_png)
            print(f"wrote {args.out_png}")
        except ImportError:
            print("matplotlib not available — skipped plot (CSV written)")


def plot(rows, param, label, out_png):
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt

    xs = [r[param] for r in rows]
    attack = [r["attack_rate"] * 100 for r in rows]
    peak = [r["peak_I"] for r in rows]

    fig, ax1 = plt.subplots(figsize=(7.5, 4.6))
    ax1.semilogx(xs, attack, "o-", color="#c0392b", lw=2, label="attack rate")
    ax1.set_xlabel(label)
    ax1.set_ylabel("attack rate (% of population infected)", color="#c0392b")
    ax1.tick_params(axis="y", labelcolor="#c0392b")
    ax1.set_ylim(-5, 105)

    ax2 = ax1.twinx()
    ax2.semilogx(xs, peak, "s--", color="#2c3e50", lw=1.3, alpha=0.7, label="peak infected")
    ax2.set_ylabel("peak infected", color="#2c3e50")
    ax2.tick_params(axis="y", labelcolor="#2c3e50")

    # mark the threshold: the value with the steepest jump in attack rate
    jumps = [(attack[i + 1] - attack[i], xs[i], xs[i + 1]) for i in range(len(xs) - 1)]
    _, lo, hi = max(jumps)
    crit = (lo * hi) ** 0.5
    ax1.axvline(crit, color="#7f8c8d", ls=":", lw=1)
    ax1.annotate(f"threshold ≈ {crit:.4f}", xy=(crit, 50),
                 xytext=(crit * 1.4, 55), fontsize=9, color="#7f8c8d")

    plt.title("Emergent epidemic threshold — SIR from Mobus primitive composition")
    fig.tight_layout()
    fig.savefig(out_png, dpi=150)


if __name__ == "__main__":
    main()
