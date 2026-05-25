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


def test_chain_propagation():
    """Port -> Propelling -> Port: verify activity propagates end-to-end."""
    systems = [
        {
            "bert_id": "test:PortIn", "display_name": "PortIn",
            "archetype": "Unspecified", "time_constant": "Second",
            "system_level": 1, "complexity_kind": "Atomic",
            "agent_kind": None, "agency_capacity": None, "primitives": [],
        },
        make_system("Pump", "Propelling", agency_capacity=0.8),
        {
            "bert_id": "test:PortOut", "display_name": "PortOut",
            "archetype": "Unspecified", "time_constant": "Second",
            "system_level": 1, "complexity_kind": "Atomic",
            "agent_kind": None, "agency_capacity": None, "primitives": [],
        },
    ]
    interactions = [
        make_flow("env_in", "Src", "PortIn", amount=10.0),
        make_flow("to_pump", "PortIn", "Pump", amount=0.0),
        make_flow("from_pump", "Pump", "PortOut", amount=0.0),
    ]

    def check(agent, model):
        agents = {a.bert_id: a for a in model.agents}
        port_in = agents["test:PortIn"]
        pump = agents["test:Pump"]
        port_out = agents["test:PortOut"]
        assert abs(port_in.state["activity"] - 10.0) < 0.01, \
            f"port_in activity should be 10.0, got {port_in.state['activity']}"
        assert abs(pump.state["activity"] - 8.0) < 0.01, \
            f"pump activity should be 8.0, got {pump.state['activity']}"
        assert abs(port_out.state["activity"] - 8.0) < 0.01, \
            f"port_out activity should be 8.0, got {port_out.state['activity']}"

    return run_test("Chain Propagation", systems, interactions, check)


def test_nan_safety():
    """Agent with None agency_capacity must not produce NaN."""
    import math as _math
    systems = [{
        "bert_id": "test:Relay", "display_name": "Relay",
        "archetype": "Unspecified", "time_constant": "Second",
        "system_level": 1, "complexity_kind": "Atomic",
        "agent_kind": None, "agency_capacity": None, "primitives": [],
    }]
    interactions = [
        make_flow("in", "Src", "Relay", amount=5.0),
        make_flow("out", "Relay", "Snk", amount=0.0, usability="Product"),
    ]

    def check(agent, model):
        assert not _math.isnan(agent.state["activity"]), "activity must not be NaN"
        assert abs(agent.state["activity"] - 5.0) < 0.01, \
            f"relay should pass through 5.0, got {agent.state['activity']}"

    return run_test("NaN Safety", systems, interactions, check)


def test_buffering_accumulation():
    """Inflow > demand: storage must grow over time."""
    def check(agent, model):
        assert agent.state["storage"] >= 200.0, \
            f"storage should accumulate (>=200), got {agent.state['storage']}"
        snapshots = [h.get("storage", 0) for h in agent.history]
        for i in range(1, len(snapshots)):
            assert snapshots[i] >= snapshots[i - 1], \
                f"storage should be monotonically non-decreasing at step {i}"

    return run_test(
        "Buffering Accumulation",
        [make_system("Buffer", "Buffering")],
        [make_flow("in", "Src", "Buffer", amount=10.0),
         make_flow("out", "Buffer", "Snk", amount=3.0, usability="Product")],
        check,
    )


def test_json_models():
    """Load each test-primitives JSON model, run 50 steps, verify no NaN."""
    import os
    import math as _math
    from json_bridge import read_model

    test_dir = os.path.join(os.path.dirname(__file__),
                            "..", "assets", "models", "local", "test-primitives")
    if not os.path.isdir(test_dir):
        return True, "SKIP (test-primitives dir not found)"

    failures = []
    count = 0
    for fname in sorted(os.listdir(test_dir)):
        if not fname.endswith(".json"):
            continue
        count += 1
        fpath = os.path.join(test_dir, fname)
        systems_df, interactions_df = read_model(fpath)
        m = BertModel(systems_df, interactions_df, seed=42)
        for _ in range(50):
            m.step()
        flow_obs, sys_obs = m.collect_all_observations()
        for obs in flow_obs:
            if _math.isnan(obs["amount"]):
                failures.append(f"{fname}: NaN flow amount")
        for obs in sys_obs:
            if _math.isnan(obs["value"]):
                failures.append(f"{fname}: NaN sys value ({obs['key']})")

    if failures:
        return False, "; ".join(failures[:5])
    return True, f"OK ({count} models)"


def test_force_parameter_injection():
    """Force from A to B should modulate B's agency_capacity."""
    systems = [
        make_system("Controller", "Propelling", agency_capacity=0.8),
        make_system("Worker", "Propelling", agency_capacity=0.5),
    ]
    interactions = [
        make_flow("env_in_ctrl", "Src", "Controller", amount=1.0),
        make_flow("env_in_work", "Src", "Worker", amount=10.0),
        make_flow("out", "Worker", "Snk", usability="Product"),
        {
            "bert_id": "test:force",
            "display_name": "Control Signal",
            "substance_type": "Message",
            "usability": "Resource",
            "interaction_type": "Force",
            "amount": 0.0,
            "source_id": "test:Controller",
            "sink_id": "test:Worker",
        },
    ]

    def check(agent, model):
        agents = {a.bert_id: a for a in model.agents}
        worker = agents["test:Worker"]
        assert worker.agency_capacity != worker._base_agency_capacity, \
            "force should have modulated agency_capacity"
        assert len(worker.force_inputs) == 1, "worker should have 1 force input"

    return run_test("Force Parameter Injection", systems, interactions, check)


def test_edge_capacity():
    """Flow with capacity=5 should cap output even when activity > 5."""
    systems = [make_system("Pump", "Propelling", agency_capacity=1.0)]
    interactions = [
        make_flow("in", "Src", "Pump", amount=10.0),
        {
            "bert_id": "test:capped_out",
            "display_name": "capped_out",
            "substance_type": "Energy",
            "usability": "Product",
            "interaction_type": "Flow",
            "amount": 0.0,
            "capacity": 5.0,
            "source_id": "test:Pump",
            "sink_id": "test:Snk",
        },
    ]

    def check(agent, model):
        assert agent.state["activity"] == 10.0, \
            f"activity should be 10.0 (uncapped internally), got {agent.state['activity']}"
        out = agent.outgoing_flows[0]["amount"]
        assert abs(out - 5.0) < 0.01, \
            f"output should be capped at 5.0, got {out}"

    return run_test("Edge Capacity", systems, interactions, check)


def test_negative_force():
    """Negative-polarity force should reduce agency_capacity."""
    systems = [
        make_system("Gov", "Propelling", agency_capacity=0.8),
        make_system("Worker", "Propelling", agency_capacity=0.5),
    ]
    interactions = [
        make_flow("env_in_gov", "Src", "Gov", amount=1.0),
        make_flow("env_in_work", "Src", "Worker", amount=10.0),
        make_flow("out", "Worker", "Snk", usability="Product"),
        {
            "bert_id": "test:neg_force",
            "display_name": "Constraint",
            "substance_type": "Message",
            "usability": "Resource",
            "interaction_type": "Force",
            "force_polarity": "negative",
            "amount": 0.0,
            "source_id": "test:Gov",
            "sink_id": "test:Worker",
        },
    ]

    def check(agent, model):
        agents = {a.bert_id: a for a in model.agents}
        worker = agents["test:Worker"]
        assert worker.agency_capacity < worker._base_agency_capacity, \
            f"negative force should reduce capacity: {worker.agency_capacity} should be < {worker._base_agency_capacity}"

    return run_test("Negative Force", systems, interactions, check)


def test_conservation_clamps_output():
    """Propelling with agency_capacity=2.0 would double input — conservation must clamp."""
    systems = [make_system("Amplifier", "Propelling", agency_capacity=2.0)]
    interactions = [
        make_flow("in", "Src", "Amplifier", substance="Energy", amount=10.0),
        make_flow("out", "Amplifier", "Snk", substance="Energy", usability="Product"),
    ]

    def check(agent, model):
        out = agent.outgoing_flows[0]["amount"]
        assert out <= 10.0, \
            f"conservation should clamp output to ≤ 10.0, got {out}"
        assert agent.state["conservation_deficit"] > 0, \
            f"should record deficit, got {agent.state['conservation_deficit']}"

    return run_test("Conservation Clamps Output", systems, interactions, check)


def test_conservation_skips_message():
    """Message flows replicate freely — conservation must not clamp them."""
    systems = [make_system("Copier", "Copying")]
    interactions = [
        make_flow("in", "Src", "Copier", substance="Message", amount=6.0),
        make_flow("outA", "Copier", "SnkA", substance="Message", usability="Product"),
        make_flow("outB", "Copier", "SnkB", substance="Message", usability="Product"),
    ]

    def check(agent, model):
        outs = [f["amount"] for f in agent.outgoing_flows]
        assert all(abs(o - 6.0) < 0.01 for o in outs), \
            f"Message outputs should NOT be clamped, got {outs}"
        assert agent.state["conservation_deficit"] == 0.0, \
            "no deficit for Message-only flows"

    return run_test("Conservation Skips Message", systems, interactions, check)


def test_conservation_with_buffer():
    """Buffering accumulates storage — conservation budget includes it."""
    systems = [make_system("Tank", "Buffering")]
    interactions = [
        make_flow("in", "Src", "Tank", substance="Energy", amount=10.0),
        make_flow("out", "Tank", "Snk", substance="Energy", amount=3.0, usability="Product"),
    ]

    sys_df = pd.DataFrame(systems)
    int_df = pd.DataFrame(interactions)
    model = BertModel(sys_df, int_df, seed=42)

    for _ in range(10):
        model.step()

    agent = list(model.agents)[0]
    try:
        assert agent.state["storage"] > 0, \
            f"storage should have accumulated, got {agent.state['storage']}"
        assert agent.state["conservation_deficit"] == 0.0, \
            "buffered output within budget should have no deficit"
        return True, "OK"
    except AssertionError as e:
        return False, str(e)


def test_copying_ignores_material():
    """Copying should only process Message flows, ignoring Material."""
    def check(agent, model):
        assert abs(agent.state["activity"] - 4.0) < 0.01, \
            f"activity should be 4.0 (Message only), got {agent.state['activity']}"
        outs = [f["amount"] for f in agent.outgoing_flows]
        assert all(abs(o - 4.0) < 0.01 for o in outs), \
            f"outputs should equal Message input (4.0), got {outs}"
    return run_test(
        "Copying Ignores Material",
        [make_system("Copier", "Copying")],
        [make_flow("mat_in", "Src", "Copier", substance="Material", amount=10.0),
         make_flow("msg_in", "SrcM", "Copier", substance="Message", amount=4.0),
         make_flow("outA", "Copier", "SnkA", substance="Message", usability="Product"),
         make_flow("outB", "Copier", "SnkB", substance="Message", usability="Product")],
        check,
    )


def test_splitting_ignores_message():
    """Splitting should only process Energy/Material flows, ignoring Message."""
    def check(agent, model):
        assert abs(agent.state["activity"] - 8.0) < 0.01, \
            f"activity should be 8.0 (Energy only), got {agent.state['activity']}"
        outs = [f["amount"] for f in agent.outgoing_flows]
        assert all(abs(o - 4.0) < 0.01 for o in outs), \
            f"outputs should be 4.0 each, got {outs}"
    return run_test(
        "Splitting Ignores Message",
        [make_system("Splitter", "Splitting")],
        [make_flow("msg_in", "SrcM", "Splitter", substance="Message", amount=5.0),
         make_flow("nrg_in", "Src", "Splitter", substance="Energy", amount=8.0),
         make_flow("outA", "Splitter", "SnkA", usability="Product"),
         make_flow("outB", "Splitter", "SnkB", usability="Product")],
        check,
    )


def test_combining_ignores_message():
    """Combining should only sum Energy/Material flows, ignoring Message."""
    def check(agent, model):
        assert abs(agent.state["activity"] - 7.0) < 0.01, \
            f"activity should be 7.0 (Energy only), got {agent.state['activity']}"
    return run_test(
        "Combining Ignores Message",
        [make_system("Combiner", "Combining")],
        [make_flow("nrg", "SrcA", "Combiner", substance="Energy", amount=7.0),
         make_flow("msg", "SrcB", "Combiner", substance="Message", amount=3.0),
         make_flow("out", "Combiner", "Snk", usability="Product")],
        check,
    )


def test_inverting_ignores_physical():
    """Inverting with only Energy input should output max_val (no Message to subtract)."""
    def check(agent, model):
        assert abs(agent.state["activity"] - 1.0) < 0.01, \
            f"activity should be 1.0 (max_signal, no Message input), got {agent.state['activity']}"
    return run_test(
        "Inverting Ignores Physical",
        [make_system("Inverter", "Inverting")],
        [make_flow("nrg_in", "Src", "Inverter", substance="Energy", amount=5.0),
         make_flow("out", "Inverter", "Snk", substance="Message", usability="Product")],
        check,
    )


def test_h_conditioned_buffering():
    """Two-phase test: filling trend → generous release, draining trend → conservative."""
    systems = [make_system("Tank", "Buffering")]
    interactions = [
        make_flow("in", "Src", "Tank", substance="Energy", amount=10.0),
        make_flow("out", "Tank", "Snk", substance="Energy", amount=3.0, usability="Product"),
    ]
    sys_df = pd.DataFrame(systems)
    int_df = pd.DataFrame(interactions)
    model = BertModel(sys_df, int_df, seed=42)

    agent = list(model.agents)[0]

    # Phase 1: high inflow, storage fills → release_factor should rise above 1.0
    for _ in range(20):
        model.step()
    factor_filling = agent.state.get("_release_factor", 1.0)

    # Phase 2: cut inflow to zero, storage drains → release_factor should drop below 1.0
    for f in agent.incoming_flows:
        f["amount"] = 0.0
    for _ in range(20):
        model.step()
    factor_draining = agent.state.get("_release_factor", 1.0)

    try:
        assert factor_filling > 1.0, \
            f"filling trend should push release_factor > 1.0, got {factor_filling}"
        assert factor_draining < 1.0, \
            f"draining trend should push release_factor < 1.0, got {factor_draining}"
        return True, f"OK (filling={factor_filling:.2f}, draining={factor_draining:.2f})"
    except AssertionError as e:
        return False, str(e)


def test_v2_combining_with_perturbation():
    """Combining-v2 with perturbation at step 50 should show non-trivial dynamics."""
    import os, math as _math
    from json_bridge import read_model

    fpath = os.path.join(os.path.dirname(__file__),
                         "..", "assets", "models", "local", "test-primitives",
                         "test-combining-v2.json")
    if not os.path.exists(fpath):
        return True, "SKIP (file not found)"

    systems_df, interactions_df = read_model(fpath)
    m = BertModel(systems_df, interactions_df, seed=42, perturbations={50: 2.0})
    observations = []
    for step in range(100):
        m.step()
        flow_obs, _ = m.collect_all_observations()
        observations.append(sum(o["amount"] for o in flow_obs))

    for obs in flow_obs:
        assert not _math.isnan(obs["amount"]), f"NaN in flow {obs['interaction_id']}"

    pre = observations[40:49]
    post = observations[55:65]
    pre_mean = sum(pre) / len(pre)
    post_mean = sum(post) / len(post)
    assert abs(post_mean - pre_mean) > 0.01, \
        f"Perturbation had no effect: pre={pre_mean:.3f} post={post_mean:.3f}"
    return True, f"OK (pre={pre_mean:.2f}, post={post_mean:.2f})"


def test_v2_splitting_with_perturbation():
    """Splitting-v2 with perturbation at step 50 should show non-trivial dynamics."""
    import os, math as _math
    from json_bridge import read_model

    fpath = os.path.join(os.path.dirname(__file__),
                         "..", "assets", "models", "local", "test-primitives",
                         "test-splitting-v2.json")
    if not os.path.exists(fpath):
        return True, "SKIP (file not found)"

    systems_df, interactions_df = read_model(fpath)
    m = BertModel(systems_df, interactions_df, seed=42, perturbations={50: 2.0})
    observations = []
    for step in range(100):
        m.step()
        flow_obs, _ = m.collect_all_observations()
        observations.append(sum(o["amount"] for o in flow_obs))

    for obs in flow_obs:
        assert not _math.isnan(obs["amount"]), f"NaN in flow {obs['interaction_id']}"

    pre = observations[40:49]
    post = observations[55:65]
    pre_mean = sum(pre) / len(pre)
    post_mean = sum(post) / len(post)
    assert abs(post_mean - pre_mean) > 0.01, \
        f"Perturbation had no effect: pre={pre_mean:.3f} post={post_mean:.3f}"
    return True, f"OK (pre={pre_mean:.2f}, post={post_mean:.2f})"


def test_duplicate_id_detection():
    """json_bridge should raise ValueError on duplicate IDs."""
    import os, tempfile, json as _json
    from json_bridge import read_model

    model = {
        "systems": [
            {"info": {"id": "S0", "level": 0, "name": "Root", "description": ""},
             "boundary": {"info": {"id": "B0", "level": 0, "name": "", "description": ""},
                          "porosity": 0, "perceptive_fuzziness": 0, "interfaces": []},
             "sources": [], "sinks": [], "complexity": "Complex"},
            {"info": {"id": "S0", "level": 0, "name": "Dupe", "description": ""},
             "boundary": {"info": {"id": "B1", "level": 0, "name": "", "description": ""},
                          "porosity": 0, "perceptive_fuzziness": 0, "interfaces": []},
             "sources": [], "sinks": [], "complexity": "Complex"},
        ],
        "interactions": [],
    }
    with tempfile.NamedTemporaryFile(mode="w", suffix=".json", delete=False) as f:
        _json.dump(model, f)
        tmp = f.name
    try:
        read_model(tmp)
        return False, "Should have raised ValueError for duplicate S0"
    except ValueError as e:
        if "Duplicate" in str(e):
            return True, f"OK ({e})"
        return False, f"Wrong error: {e}"
    finally:
        os.unlink(tmp)


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
    ("Chain Propagation", test_chain_propagation),
    ("NaN Safety", test_nan_safety),
    ("Buffering Accumulation", test_buffering_accumulation),
    ("Force Parameter Injection", test_force_parameter_injection),
    ("Edge Capacity", test_edge_capacity),
    ("Negative Force", test_negative_force),
    ("JSON Models Integration", test_json_models),
    ("Conservation Clamps Output", test_conservation_clamps_output),
    ("Conservation Skips Message", test_conservation_skips_message),
    ("Conservation With Buffer", test_conservation_with_buffer),
    ("Copying Ignores Material", test_copying_ignores_material),
    ("Splitting Ignores Message", test_splitting_ignores_message),
    ("Combining Ignores Message", test_combining_ignores_message),
    ("Inverting Ignores Physical", test_inverting_ignores_physical),
    ("H-Conditioned Buffering", test_h_conditioned_buffering),
    ("V2 Combining With Perturbation", test_v2_combining_with_perturbation),
    ("V2 Splitting With Perturbation", test_v2_splitting_with_perturbation),
    ("Duplicate ID Detection", test_duplicate_id_detection),
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
