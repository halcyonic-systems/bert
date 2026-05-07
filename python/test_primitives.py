"""
Standalone test for BERT process primitive T functions.
No TypeDB required — constructs DataFrames directly.

Usage: python test_primitives.py
"""

import sys
import pandas as pd
from model import BertModel

STEPS = 50


def make_system(name, primitive, agency_capacity=0.5):
    return {
        "bert_id": f"test:{name}",
        "display_name": name,
        "archetype": "Agent",
        "time_constant": "Second",
        "system_level": 1,
        "complexity_kind": "Atomic",
        "agent_kind": "Reactive",
        "agency_capacity": agency_capacity,
        "primitives": [primitive],
    }


def make_flow(fid, src, snk, substance="Energy", amount=10.0, usability="Resource"):
    return {
        "bert_id": f"test:{fid}",
        "display_name": fid,
        "substance_type": substance,
        "usability": usability,
        "interaction_type": "Flow",
        "amount": amount,
        "source_id": f"test:{src}",
        "sink_id": f"test:{snk}",
    }


def run_test(name, systems, interactions, check_fn):
    """Run a single primitive test. Returns (pass, message)."""
    sys_df = pd.DataFrame(systems)
    int_df = pd.DataFrame(interactions)
    model = BertModel(sys_df, int_df, seed=42)

    for _ in range(STEPS):
        model.step()

    agent = list(model.agents)[0]
    try:
        check_fn(agent, model)
        return True, "OK"
    except AssertionError as e:
        return False, str(e)


def test_buffering():
    def check(agent, model):
        assert agent.state["storage"] > 0, f"storage should accumulate, got {agent.state['storage']}"
        assert len(agent.history) == STEPS, f"history should have {STEPS} entries"
    return run_test(
        "Buffering",
        [make_system("Buffer", "Buffering")],
        [make_flow("in", "Src", "Buffer", amount=10.0),
         make_flow("out", "Buffer", "Snk", amount=5.0, usability="Product")],
        check,
    )


def test_combining():
    def check(agent, model):
        assert agent.state["activity"] > 0, f"activity should be sum of inflows"
    return run_test(
        "Combining",
        [make_system("Combiner", "Combining")],
        [make_flow("a", "SrcA", "Combiner", amount=7.0),
         make_flow("b", "SrcB", "Combiner", amount=3.0),
         make_flow("out", "Combiner", "Snk", usability="Product")],
        check,
    )


def test_splitting():
    def check(agent, model):
        assert agent.state["activity"] > 0, "should have processed input"
        outs = [f["amount"] for f in agent.outgoing_flows]
        if len(outs) == 2:
            assert abs(outs[0] - outs[1]) < 0.001, f"outputs should be equal, got {outs}"
    return run_test(
        "Splitting",
        [make_system("Splitter", "Splitting")],
        [make_flow("in", "Src", "Splitter", amount=10.0),
         make_flow("outA", "Splitter", "SnkA", usability="Product"),
         make_flow("outB", "Splitter", "SnkB", usability="Product")],
        check,
    )


def test_propelling():
    def check(agent, model):
        expected = 10.0 * 0.7
        assert abs(agent.state["activity"] - expected) < 0.01, \
            f"activity should be {expected}, got {agent.state['activity']}"
    return run_test(
        "Propelling",
        [make_system("Pump", "Propelling", agency_capacity=0.7)],
        [make_flow("in", "Src", "Pump", amount=10.0),
         make_flow("out", "Pump", "Snk", usability="Product")],
        check,
    )


def test_impeding():
    def check(agent, model):
        assert agent.state["back_pressure"] > 0, "should create back pressure"
        assert agent.state["activity"] < 10.0, "output should be less than input"
    return run_test(
        "Impeding",
        [make_system("Resistor", "Impeding", agency_capacity=0.3)],
        [make_flow("in", "Src", "Resistor", amount=10.0),
         make_flow("out", "Resistor", "Snk", usability="Product")],
        check,
    )


def test_sensing():
    def check(agent, model):
        assert agent.state["signal"] > 0, "should produce signal from physical input"
        assert agent.state["activity"] == agent.state["signal"], "activity should equal signal"
    return run_test(
        "Sensing",
        [make_system("Sensor", "Sensing")],
        [make_flow("in", "Src", "Sensor", substance="Energy", amount=8.0),
         make_flow("out", "Sensor", "Snk", substance="Message", usability="Product")],
        check,
    )


def test_modulating():
    def check(agent, model):
        assert agent.state["control_signal"] > 0, "should read control signal"
        assert agent.state["activity"] > 0, "should produce modulated output"
    return run_test(
        "Modulating",
        [make_system("Modulator", "Modulating")],
        [make_flow("primary", "SrcP", "Modulator", substance="Energy", amount=10.0),
         make_flow("control", "SrcC", "Modulator", substance="Message", amount=0.8),
         make_flow("out", "Modulator", "Snk", usability="Product")],
        check,
    )


def test_inverting():
    def check(agent, model):
        assert agent.state["activity"] >= 0, "inverted output should be non-negative"
        expected = max(0.0, 1.0 - 5.0)
        assert agent.state["activity"] == expected, \
            f"activity should be max(0, 1.0 - 5.0) = {expected}, got {agent.state['activity']}"
    return run_test(
        "Inverting",
        [make_system("Inverter", "Inverting")],
        [make_flow("in", "Src", "Inverter", substance="Message", amount=5.0),
         make_flow("out", "Inverter", "Snk", substance="Message", usability="Product")],
        check,
    )


def test_copying():
    def check(agent, model):
        assert agent.state["activity"] > 0, "should have processed input"
        outs = [f["amount"] for f in agent.outgoing_flows]
        if len(outs) == 2:
            assert all(abs(o - 6.0) < 0.01 for o in outs), \
                f"both outputs should equal input (6.0), got {outs}"
    return run_test(
        "Copying",
        [make_system("Copier", "Copying")],
        [make_flow("in", "Src", "Copier", substance="Message", amount=6.0),
         make_flow("outA", "Copier", "SnkA", substance="Message", usability="Product"),
         make_flow("outB", "Copier", "SnkB", substance="Message", usability="Product")],
        check,
    )


def test_archetype_fallback():
    """Verify that agents WITHOUT primitives still use archetype-based _act()."""
    def check(agent, model):
        assert agent.state["resource_level"] > 0, "Economy archetype should accumulate resources"
    return run_test(
        "Archetype Fallback",
        [{
            "bert_id": "test:Economy",
            "display_name": "Economy",
            "archetype": "Economy",
            "time_constant": "Second",
            "system_level": 1,
            "complexity_kind": "Atomic",
            "agent_kind": None,
            "agency_capacity": 0.5,
            "primitives": [],
        }],
        [make_flow("in", "Src", "Economy", amount=10.0),
         make_flow("out", "Economy", "Snk", usability="Product")],
        check,
    )


ALL_TESTS = [
    ("Buffering",  test_buffering),
    ("Combining",  test_combining),
    ("Splitting",  test_splitting),
    ("Propelling", test_propelling),
    ("Impeding",   test_impeding),
    ("Sensing",    test_sensing),
    ("Modulating", test_modulating),
    ("Inverting",  test_inverting),
    ("Copying",    test_copying),
    ("Archetype Fallback", test_archetype_fallback),
]


def main():
    passed = 0
    failed = 0
    for name, test_fn in ALL_TESTS:
        ok, msg = test_fn()
        status = "PASS" if ok else "FAIL"
        print(f"  {status}  {name}: {msg}")
        if ok:
            passed += 1
        else:
            failed += 1

    print(f"\n{passed}/{passed + failed} tests passed")
    sys.exit(0 if failed == 0 else 1)


if __name__ == "__main__":
    main()
