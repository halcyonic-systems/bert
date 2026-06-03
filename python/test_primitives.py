"""
Executable specification for BERT's simulation layer — the Mobus process primitives
(`agents.py` T-functions) and their compositions, run on the `BertModel` engine (`model.py`).
If a primitive's test passes, its T-function is correct in Mobus's sense; see the
"Verification Contract" in `docs/process-primitives.md`, which this file backs.

Two test modes:
  1. In-code diagnostics — build the simulation directly with `make_system`/`make_flow`
     (pandas DataFrames -> BertModel), no TypeDB and no JSON. Isolates the *dynamics*:
     a failure means the physics is wrong, not the loader. Covers each primitive's
     characteristic transfer function and the in-code composition circuits
     (negative-feedback, fanout, buffer-smoothing, oscillator).
  2. Model-loading tests — load real BERT WorldModel JSONs from
     `assets/models/local/test-primitives/*.json` via `json_bridge.read_model` (the same
     JSON -> DataFrame path the BERT app uses) and assert end-to-end dynamics. These verify
     the generated, GUI-loadable circuits (error-sensing, regulated-buffer, energy-chain,
     info-broadcast, oscillator). Their `*-spec.json` siblings are the generator inputs.

Runs both as pytest (`pytest test_primitives.py`) and as a self-reporting script
(`python test_primitives.py`, via the registration list near the bottom + the __main__ block).
No TypeDB required in either mode.
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


def make_flow(fid, src, snk, substance="Energy", amount=10.0, usability="Resource",
              observation=False):
    return {
        "bert_id": f"test:{fid}",
        "display_name": fid,
        "substance_type": substance,
        "usability": usability,
        "interaction_type": "Flow",
        "amount": amount,
        "source_id": f"test:{src}",
        "sink_id": f"test:{snk}",
        "observation": observation,
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


def test_combining_asymmetric_perturbation():
    """Doubling ONE of two combining inputs should give 1.5x, not 2x."""
    systems = [make_system("Combiner", "Combining")]
    interactions = [
        make_flow("A", "SrcA", "Combiner", "Energy", 4.0),
        make_flow("B", "SrcB", "Combiner", "Energy", 4.0),
    ]
    sys_df = pd.DataFrame(systems)
    int_df = pd.DataFrame(interactions)
    m = BertModel(sys_df, int_df, seed=42, perturbations={})
    for _ in range(10):
        m.step()
    agent = list(m.agents)[0]
    pre_activity = agent.state.get("activity", 0)
    assert abs(pre_activity - 8.0) < 0.1, f"Expected ~8.0, got {pre_activity}"
    for f in agent.incoming_flows:
        if "SrcA" in f.get("_source_id", ""):
            f["amount"] = 8.0
    m.step()
    post_activity = agent.state.get("activity", 0)
    ratio = post_activity / pre_activity if pre_activity else 0
    assert 1.4 < ratio < 1.6, f"Expected ~1.5x ratio, got {ratio:.2f}x (pre={pre_activity}, post={post_activity})"
    return True, f"OK (ratio={ratio:.2f}x, pre={pre_activity:.1f}, post={post_activity:.1f})"


def test_modulating_bilinearity():
    """Doubling BOTH inputs of Modulating should give 4x (bilinear), not 2x.
    Baseline control 0.4 so the doubled control (0.8) stays inside the conservative
    [0,1] gate — bilinearity holds while mass stays bounded by the primary."""
    systems = [make_system("Modulator", "Modulating")]
    interactions = [
        make_flow("Primary", "SrcP", "Modulator", "Energy", 5.0),
        make_flow("Control", "SrcC", "Modulator", "Message", 0.4),
    ]
    sys_df = pd.DataFrame(systems)
    int_df = pd.DataFrame(interactions)
    m = BertModel(sys_df, int_df, seed=42)
    for _ in range(10):
        m.step()
    agent = list(m.agents)[0]
    baseline = agent.state.get("activity", 0)
    assert baseline > 0, f"Baseline activity should be >0, got {baseline}"
    for f in agent.incoming_flows:
        f["amount"] *= 2.0
    m.step()
    doubled = agent.state.get("activity", 0)
    ratio = doubled / baseline if baseline else 0
    assert 3.5 < ratio < 4.5, f"Expected ~4x (bilinear), got {ratio:.2f}x"
    return True, f"OK (ratio={ratio:.2f}x, baseline={baseline:.1f}, doubled={doubled:.1f})"


def test_amplifying():
    """Amplifying adds metered power to a weak signal: out = gain*signal when power
    is ample, but capped by available power (conservation — cannot amplify beyond the
    power source). Mobus Fig. 3.19, distinct from Modulating (gates a flow)."""
    # Ample power: gain (agency 0.5 -> 5.5) * signal 2.0 = 11.0, power 100 not binding
    sys_a = [make_system("Amplifier", "Amplifying", agency_capacity=0.5)]
    int_a = [
        make_flow("signal", "SrcSig", "Amplifier", "Message", 2.0),
        make_flow("power", "SrcPwr", "Amplifier", "Energy", 100.0),
        make_flow("out", "Amplifier", "Snk", "Energy", usability="Product"),
    ]
    m = BertModel(pd.DataFrame(sys_a), pd.DataFrame(int_a), seed=42)
    for _ in range(10):
        m.step()
    out_ample = list(m.agents)[0].state.get("activity", 0.0)
    assert abs(out_ample - 11.0) < 0.5, f"Ample power: expect ~11 (5.5x2), got {out_ample:.2f}"

    # Power-limited: same gain/signal but only 5.0 power -> output capped at 5.0
    sys_b = [make_system("Amplifier", "Amplifying", agency_capacity=0.5)]
    int_b = [
        make_flow("signal", "SrcSig", "Amplifier", "Message", 2.0),
        make_flow("power", "SrcPwr", "Amplifier", "Energy", 5.0),
        make_flow("out", "Amplifier", "Snk", "Energy", usability="Product"),
    ]
    m2 = BertModel(pd.DataFrame(sys_b), pd.DataFrame(int_b), seed=42)
    for _ in range(10):
        m2.step()
    out_limited = list(m2.agents)[0].state.get("activity", 0.0)
    assert abs(out_limited - 5.0) < 0.5, f"Power-limited: output capped at power 5.0, got {out_limited:.2f}"
    assert out_limited < out_ample, "Limited power must reduce output below the ample case"
    return True, f"OK (ample={out_ample:.1f}=5.5x2, power-limited={out_limited:.1f} capped at source)"


def test_inverting_direction():
    """Increasing Message input to Inverter should DECREASE output."""
    systems = [make_system("Inverter", "Inverting")]
    interactions = [
        make_flow("In", "SrcMsg", "Inverter", "Message", 0.3),
    ]
    sys_df = pd.DataFrame(systems)
    int_df = pd.DataFrame(interactions)
    m = BertModel(sys_df, int_df, seed=42)
    for _ in range(10):
        m.step()
    agent = list(m.agents)[0]
    out_low = agent.state.get("activity", 0)
    assert abs(out_low - 0.7) < 0.1, f"Expected ~0.7 (1.0-0.3), got {out_low}"
    for f in agent.incoming_flows:
        f["amount"] = 0.6
    m.step()
    out_high = agent.state.get("activity", 0)
    assert out_high < out_low, f"Output should decrease: was {out_low}, now {out_high}"
    return True, f"OK (input 0.3→output {out_low:.2f}, input 0.6→output {out_high:.2f})"


def test_buffering_temporal_lag():
    """Buffer should show lag: output rises gradually, not instantly with input."""
    systems = [make_system("Buffer", "Buffering")]
    interactions = [
        make_flow("In", "Src", "Buffer", "Material", 10.0),
        make_flow("Out", "Buffer", "Snk", "Material", 3.0, "Product"),
    ]
    sys_df = pd.DataFrame(systems)
    int_df = pd.DataFrame(interactions)
    m = BertModel(sys_df, int_df, seed=42)
    outputs = []
    storages = []
    for _ in range(20):
        m.step()
        agent = list(m.agents)[0]
        outputs.append(agent.state.get("activity", 0))
        storages.append(agent.state.get("storage", 0))
    assert storages[-1] > storages[0], f"Storage should accumulate: start={storages[0]:.1f} end={storages[-1]:.1f}"
    assert outputs[-1] <= 5.0, f"Output should be near demand (3), not near input (10): got {outputs[-1]:.1f}"
    return True, f"OK (storage grew {storages[0]:.1f}→{storages[-1]:.1f}, output stable at {outputs[-1]:.1f})"


def test_copying_non_conservation():
    """Copying should violate conservation: sum of outputs > input."""
    systems = [make_system("Copier", "Copying")]
    interactions = [
        make_flow("In", "Src", "Copier", "Message", 6.0),
        make_flow("OutA", "Copier", "SnkA", "Message", 0.0, "Product"),
        make_flow("OutB", "Copier", "SnkB", "Message", 0.0, "Product"),
    ]
    sys_df = pd.DataFrame(systems)
    int_df = pd.DataFrame(interactions)
    m = BertModel(sys_df, int_df, seed=42)
    for _ in range(10):
        m.step()
    agent = list(m.agents)[0]
    total_out = sum(f.get("amount", 0) for f in agent.outgoing_flows)
    total_in = sum(f.get("amount", 0) for f in agent.incoming_flows)
    assert total_out > total_in, f"Copying should violate conservation: out={total_out} should > in={total_in}"
    return True, f"OK (in={total_in:.1f}, out={total_out:.1f}, ratio={total_out/total_in:.1f}x — non-conservative)"


def test_impeding_backpressure():
    """Impeding should create measurable back_pressure state."""
    systems = [make_system("Resistor", "Impeding", agency_capacity=0.3)]
    interactions = [
        make_flow("In", "Src", "Resistor", "Material", 10.0),
    ]
    sys_df = pd.DataFrame(systems)
    int_df = pd.DataFrame(interactions)
    m = BertModel(sys_df, int_df, seed=42)
    for _ in range(10):
        m.step()
    agent = list(m.agents)[0]
    bp = agent.state.get("back_pressure", 0)
    activity = agent.state.get("activity", 0)
    assert bp > 0, f"back_pressure should be >0, got {bp}"
    assert abs(activity + bp - 10.0) < 0.1, f"activity + back_pressure should = input: {activity} + {bp} != 10.0"
    return True, f"OK (activity={activity:.1f}, back_pressure={bp:.1f}, sum={activity+bp:.1f})"


def test_composition_negative_feedback():
    """Sensing + Inverting + Modulating in a loop should produce negative feedback:
    system converges to a fixed point where output = input / (1 + input*k)."""
    sensor = make_system("Sensor", "Sensing", agency_capacity=0.05)
    inverter = make_system("Inverter", "Inverting", agency_capacity=0.5)
    modulator = make_system("Modulator", "Modulating", agency_capacity=0.5)
    systems = [sensor, inverter, modulator]
    interactions = [
        make_flow("PhysIn", "Source", "Modulator", "Energy", 10.0),
        make_flow("ModToSensor", "Modulator", "Sensor", "Energy", 0.0),
        make_flow("SenseSignal", "Sensor", "Inverter", "Message", 0.0),
        make_flow("ErrorToMod", "Inverter", "Modulator", "Message", 0.0),
        make_flow("Output", "Modulator", "Sink", "Energy", 0.0, "Product"),
    ]
    sys_df = pd.DataFrame(systems)
    int_df = pd.DataFrame(interactions)
    m = BertModel(sys_df, int_df, seed=42)
    outputs = []
    for step in range(40):
        m.step()
        mod = [a for a in m.agents if a.display_name == "Modulator"][0]
        outputs.append(mod.state.get("activity", 0))
    late = outputs[30:40]
    late_mean = sum(late) / len(late)
    late_std = (sum((x - late_mean)**2 for x in late) / len(late)) ** 0.5
    assert late_std < 1.0, f"Should stabilize: std={late_std:.2f}, values={[f'{x:.1f}' for x in late]}"
    assert late_mean > 0, f"Should have positive output, got {late_mean:.2f}"
    return True, f"OK (converged to ~{late_mean:.2f}, std={late_std:.3f})"


def test_composition_sensing_copying_fanout():
    """Sensing → Copying → two Modulators: one stimulus, two parallel control effects."""
    sensor = make_system("Sensor", "Sensing", agency_capacity=0.8)
    copier = make_system("Copier", "Copying", agency_capacity=0.5)
    mod_a = make_system("ModA", "Modulating", agency_capacity=0.5)
    mod_b = make_system("ModB", "Modulating", agency_capacity=0.5)
    systems = [sensor, copier, mod_a, mod_b]
    interactions = [
        make_flow("PhysStimulus", "Source", "Sensor", "Energy", 5.0),
        make_flow("Signal", "Sensor", "Copier", "Message", 0.0),
        make_flow("CopyA", "Copier", "ModA", "Message", 0.0),
        make_flow("CopyB", "Copier", "ModB", "Message", 0.0),
        make_flow("PrimaryA", "SourceA", "ModA", "Energy", 8.0),
        make_flow("PrimaryB", "SourceB", "ModB", "Energy", 8.0),
    ]
    sys_df = pd.DataFrame(systems)
    int_df = pd.DataFrame(interactions)
    m = BertModel(sys_df, int_df, seed=42)
    for _ in range(20):
        m.step()
    mod_a_agent = [a for a in m.agents if a.display_name == "ModA"][0]
    mod_b_agent = [a for a in m.agents if a.display_name == "ModB"][0]
    act_a = mod_a_agent.state.get("activity", 0)
    act_b = mod_b_agent.state.get("activity", 0)
    assert act_a > 0, f"ModA should have activity, got {act_a}"
    assert act_b > 0, f"ModB should have activity, got {act_b}"
    assert abs(act_a - act_b) < 0.1, f"Both modulators should track same signal: A={act_a:.2f} B={act_b:.2f}"
    return True, f"OK (ModA={act_a:.2f}, ModB={act_b:.2f} — parallel control from one stimulus)"


def test_composition_buffering_smooths_perturbation():
    """Buffer downstream of a direct input shock should smooth the transient."""
    buffer = make_system("Tank", "Buffering", agency_capacity=0.5)
    systems = [buffer]
    interactions = [
        make_flow("In", "Source", "Tank", "Material", 5.0),
        make_flow("Out", "Tank", "Sink", "Material", 2.0, "Product"),
    ]
    sys_df = pd.DataFrame(systems)
    int_df = pd.DataFrame(interactions)
    m = BertModel(sys_df, int_df, seed=42)
    storages = []
    outputs = []
    for step in range(60):
        m.step()
        tank = [a for a in m.agents if a.display_name == "Tank"][0]
        storages.append(tank.state.get("storage", 0))
        outputs.append(tank.state.get("activity", 0))
        if step == 29:
            for f in tank.incoming_flows:
                f["amount"] = 15.0
    storage_pre = storages[29]
    storage_post = storages[40]
    output_std = (sum((x - sum(outputs[30:40])/10)**2 for x in outputs[30:40]) / 10) ** 0.5
    assert storage_post > storage_pre, \
        f"Storage should absorb shock: pre={storage_pre:.1f} post={storage_post:.1f}"
    assert output_std < 2.0, f"Output should stay smooth despite input shock: std={output_std:.2f}"
    return True, f"OK (storage absorbed shock: {storage_pre:.0f}→{storage_post:.0f}, output std={output_std:.2f})"


def test_composition_oscillator():
    """Buffering integrator inside a negative-feedback loop produces sustained oscillation.
    Source -> Modulator -> Tank(Buffer) -> Sensor -> Inverter -> (control) -> Modulator.
    The Buffer's integration adds the phase lag that turns the *converging* negative-feedback
    fixed point (cf. test_composition_negative_feedback) into a *bounded limit cycle*. The
    Modulating [0,2] clamp and Inverting max(0, .) clamp bound it. Oscillation emerges purely
    from primitive composition — no hand-coded oscillator."""
    tank = make_system("Tank", "Buffering", agency_capacity=0.5)
    sensor = make_system("Sensor", "Sensing", agency_capacity=0.2)
    inverter = make_system("Inverter", "Inverting", agency_capacity=0.5)
    modulator = make_system("Modulator", "Modulating", agency_capacity=1.0)
    systems = [tank, sensor, inverter, modulator]
    interactions = [
        make_flow("In", "Source", "Modulator", "Energy", 10.0),
        make_flow("ModToTank", "Modulator", "Tank", "Energy", 0.0),
        make_flow("TankToSensor", "Tank", "Sensor", "Energy", 0.0),
        make_flow("SenseSignal", "Sensor", "Inverter", "Message", 0.0),
        make_flow("ErrorToMod", "Inverter", "Modulator", "Message", 0.0),
        make_flow("Out", "Tank", "Sink", "Energy", 5.0, "Product"),
    ]
    sys_df = pd.DataFrame(systems)
    int_df = pd.DataFrame(interactions)
    m = BertModel(sys_df, int_df, seed=42)
    series = []
    for _ in range(80):
        m.step()
        tank_agent = [a for a in m.agents if a.display_name == "Tank"][0]
        series.append(tank_agent.state.get("activity", 0.0))
    tail = series[30:]
    turning_points = sum(
        1 for i in range(1, len(tail) - 1)
        if (tail[i] - tail[i - 1]) * (tail[i + 1] - tail[i]) < 0
    )
    amplitude = max(tail) - min(tail)
    assert turning_points >= 6, \
        f"Should sustain oscillation, got {turning_points} turning points: {[f'{x:.1f}' for x in tail[-12:]]}"
    assert amplitude > 2.0, f"Oscillation should have meaningful amplitude, got {amplitude:.2f}"
    assert max(series) < 50.0, f"Oscillation must stay bounded (limit cycle), peaked at {max(series):.1f}"
    return True, f"OK (sustained limit cycle: {turning_points} turning points, amplitude {amplitude:.1f})"


def test_anticipatory_conditioning():
    """Anticipatory agent should adjust agency_capacity based on activity trend prediction."""
    sys_data = {
        "bert_id": "test:Anticipator", "display_name": "Anticipator",
        "archetype": "Agent", "time_constant": "Second", "system_level": 1,
        "complexity_kind": "Atomic", "agent_kind": "Anticipatory",
        "agency_capacity": 0.5, "primitives": ["Propelling"],
    }
    interactions = [make_flow("In", "Src", "Anticipator", "Energy", 5.0)]
    sys_df = pd.DataFrame([sys_data])
    int_df = pd.DataFrame(interactions)
    m = BertModel(sys_df, int_df, seed=42)
    agent = list(m.agents)[0]
    for step in range(15):
        m.step()
        if step == 7:
            for f in agent.incoming_flows:
                f["amount"] = 10.0
    base_cap = agent._base_agency_capacity
    pf = agent.state.get("_prediction_factor", 1.0)
    assert pf != 1.0, f"Anticipatory agent should predict rising trend after input change, got pf={pf}"
    return True, f"OK (prediction_factor={pf:.3f}, agency_cap={agent.agency_capacity:.3f})"


def test_intentional_conditioning():
    """Intentional agent should adjust effort based on goal-relative performance."""
    sys_data = {
        "bert_id": "test:Intentional", "display_name": "Intentional",
        "archetype": "Agent", "time_constant": "Second", "system_level": 1,
        "complexity_kind": "Atomic", "agent_kind": "Intentional",
        "agency_capacity": 0.8, "primitives": ["Propelling"],
    }
    interactions = [make_flow("In", "Src", "Intentional", "Energy", 5.0)]
    sys_df = pd.DataFrame([sys_data])
    int_df = pd.DataFrame(interactions)
    m = BertModel(sys_df, int_df, seed=42)
    agent = list(m.agents)[0]
    for step in range(15):
        m.step()
        if step == 7:
            for f in agent.incoming_flows:
                f["amount"] = 2.0
    ef = agent.state.get("_effort_factor", 1.0)
    assert ef != 1.0, f"Intentional agent should adjust effort after input drop, got ef={ef}"
    return True, f"OK (effort_factor={ef:.3f}, agency_cap={agent.agency_capacity:.3f})"


def test_regulated_buffer():
    """Regulated buffer circuit: Buffering + Sensing + Inverting + Modulating compose into level control."""
    import os
    from json_bridge import read_model

    fpath = os.path.join(os.path.dirname(__file__),
                         "..", "assets", "models", "local", "test-primitives",
                         "regulated-buffer.json")
    if not os.path.exists(fpath):
        return True, "SKIP (file not found)"

    systems_df, interactions_df = read_model(fpath)
    m = BertModel(systems_df, interactions_df, seed=42, perturbations={100: 2.0})

    storages = []
    for _ in range(200):
        m.step()
        buf = [a for a in m.agents if a.display_name == "Buffer"][0]
        storages.append(buf.state.get("storage", 0.0))

    pre_std = (sum((x - sum(storages[50:90])/40)**2 for x in storages[50:90]) / 40) ** 0.5
    assert storages[-1] > 0, f"Buffer should accumulate storage, got {storages[-1]:.2f}"
    assert pre_std < storages[-1] * 0.5, \
        f"Storage should be more stable than wild, std={pre_std:.2f} vs storage={storages[-1]:.2f}"
    return True, f"OK (final storage={storages[-1]:.0f}, pre-std={pre_std:.2f})"


def test_oscillator():
    """Oscillator circuit (loadable model): same topology as the regulated buffer, but tuned
    (higher sensor gain, larger demand) so the Buffer's integration lag turns the converging
    fixed point into a sustained bounded limit cycle. Sister to test_composition_oscillator —
    that one builds the loop in-code; this one proves the generated, GUI-loadable
    oscillator.json reproduces the same periodic dynamics through the json_bridge path."""
    import os
    from json_bridge import read_model

    fpath = os.path.join(os.path.dirname(__file__),
                         "..", "assets", "models", "local", "test-primitives",
                         "oscillator.json")
    if not os.path.exists(fpath):
        return True, "SKIP (file not found)"

    systems_df, interactions_df = read_model(fpath)
    m = BertModel(systems_df, interactions_df, seed=42)

    series = []
    for _ in range(80):
        m.step()
        buf = [a for a in m.agents if a.display_name == "Buffer"][0]
        series.append(buf.state.get("activity", 0.0))

    tail = series[30:]
    turning_points = sum(
        1 for i in range(1, len(tail) - 1)
        if (tail[i] - tail[i - 1]) * (tail[i + 1] - tail[i]) < 0
    )
    amplitude = max(tail) - min(tail)
    assert turning_points >= 6, \
        f"Loaded oscillator should sustain oscillation, got {turning_points}: {[f'{x:.1f}' for x in tail[-12:]]}"
    assert amplitude > 2.0, f"Oscillation should have meaningful amplitude, got {amplitude:.2f}"
    assert max(series) < 50.0, f"Oscillation must stay bounded, peaked at {max(series):.1f}"
    return True, f"OK (loaded limit cycle: {turning_points} turning points, amplitude {amplitude:.1f})"


def test_energy_chain():
    """Energy processing chain: Combining + Propelling + Splitting. Conservation with efficiency loss."""
    import os
    from json_bridge import read_model

    fpath = os.path.join(os.path.dirname(__file__),
                         "..", "assets", "models", "local", "test-primitives",
                         "energy-chain.json")
    if not os.path.exists(fpath):
        return True, "SKIP (file not found)"

    systems_df, interactions_df = read_model(fpath)
    m = BertModel(systems_df, interactions_df, seed=42, perturbations={100: 2.0})

    outputs = []
    for _ in range(200):
        m.step()
        splitter = [a for a in m.agents if a.display_name == "Splitter"][0]
        outputs.append(splitter.state.get("activity", 0.0))

    pre = sum(outputs[50:90]) / 40
    post = sum(outputs[150:200]) / 50
    assert post > pre, f"Perturbation should increase output: pre={pre:.2f} post={post:.2f}"
    propeller = [a for a in m.agents if a.display_name == "Propeller"][0]
    assert propeller.agency_capacity == 0.7, "Propeller efficiency should be 0.7"
    return True, f"OK (pre={pre:.2f}, post={post:.2f}, η=0.7)"


def test_info_broadcast():
    """Information broadcast: Sensing + Copying + 2x Modulating. One signal controls two processes."""
    import os
    from json_bridge import read_model

    fpath = os.path.join(os.path.dirname(__file__),
                         "..", "assets", "models", "local", "test-primitives",
                         "info-broadcast.json")
    if not os.path.exists(fpath):
        return True, "SKIP (file not found)"

    systems_df, interactions_df = read_model(fpath)
    m = BertModel(systems_df, interactions_df, seed=42)

    for _ in range(50):
        m.step()
    modA = [a for a in m.agents if a.display_name == "Modulator A"][0]
    modB = [a for a in m.agents if a.display_name == "Modulator B"][0]

    diff = abs(modA.state.get("activity", 0) - modB.state.get("activity", 0))
    assert diff < 0.01, f"Both modulators should track same signal, diff={diff:.4f}"
    assert modA.state.get("activity", 0) > 0, "Modulators should have non-zero activity"
    return True, f"OK (ModA={modA.state['activity']:.2f}, ModB={modB.state['activity']:.2f}, synced)"


def test_error_sensing_circuit():
    """Mobus Ch. 4 canonical circuit: 4 Markovian primitives compose into a thermostat."""
    import os
    from json_bridge import read_model

    fpath = os.path.join(os.path.dirname(__file__),
                         "..", "assets", "models", "local", "test-primitives",
                         "error-sensing-circuit.json")
    if not os.path.exists(fpath):
        return True, "SKIP (file not found)"

    systems_df, interactions_df = read_model(fpath)
    m = BertModel(systems_df, interactions_df, seed=42, perturbations={100: 2.0})

    outputs = []
    for _ in range(200):
        m.step()
        combiner = [a for a in m.agents if a.display_name == "Combiner"][0]
        outputs.append(combiner.state.get("activity", 0.0))

    pre = outputs[50:90]
    pre_mean = sum(pre) / len(pre)
    pre_std = (sum((x - pre_mean) ** 2 for x in pre) / len(pre)) ** 0.5

    post = outputs[150:200]
    post_mean = sum(post) / len(post)
    post_std = (sum((x - post_mean) ** 2 for x in post) / len(post)) ** 0.5

    assert pre_std < 1.0, f"Should converge pre-perturbation, std={pre_std:.4f}"
    assert post_std < 1.0, f"Should re-converge post-perturbation, std={post_std:.4f}"
    assert post_mean > pre_mean, \
        f"Post-perturbation setpoint should be higher (2x input): pre={pre_mean:.2f} post={post_mean:.2f}"
    return True, f"OK (pre={pre_mean:.2f}±{pre_std:.2f}, post={post_mean:.2f}±{post_std:.2f}, regulation holds)"


def test_system_level_history():
    """BertModel.system_history should be populated and accessible from agents."""
    systems = [make_system("Tank", "Buffering")]
    interactions = [
        make_flow("in", "Src", "Tank", substance="Energy", amount=10.0),
        make_flow("out", "Tank", "Snk", substance="Energy", amount=3.0, usability="Product"),
    ]
    sys_df = pd.DataFrame(systems)
    int_df = pd.DataFrame(interactions)
    model = BertModel(sys_df, int_df, seed=42)

    for _ in range(20):
        model.step()

    assert len(model.system_history) == 20, \
        f"system_history should have 20 entries, got {len(model.system_history)}"
    last = model.system_history[-1]
    assert "mean_activity" in last, "system_history entry missing mean_activity"
    assert "total_throughput" in last, "system_history entry missing total_throughput"
    assert "tick" in last, "system_history entry missing tick"
    assert last["tick"] == 20, f"last tick should be 20, got {last['tick']}"
    agent = list(model.agents)[0]
    assert hasattr(agent.model, "system_history"), \
        "agent should be able to access model.system_history"
    return True, f"OK ({len(model.system_history)} entries, last tick={last['tick']})"


def test_markovian_primitives():
    """Non-Buffering primitives must produce identical output regardless of history length."""
    markovian = ["Sensing", "Impeding", "Modulating", "Propelling", "Amplifying"]
    for prim_name in markovian:
        systems = [make_system(prim_name, prim_name)]
        if prim_name == "Modulating":
            interactions = [
                make_flow("primary", "Src", prim_name, substance="Energy", amount=10.0),
                make_flow("control", "Ctrl", prim_name, substance="Message", amount=0.5),
                make_flow("out", prim_name, "Snk", substance="Energy", usability="Product"),
            ]
        elif prim_name == "Amplifying":
            interactions = [
                make_flow("signal", "Src", prim_name, substance="Message", amount=2.0),
                make_flow("power", "Pwr", prim_name, substance="Energy", amount=100.0),
                make_flow("out", prim_name, "Snk", substance="Energy", usability="Product"),
            ]
        else:
            interactions = [
                make_flow("in", "Src", prim_name, substance="Energy", amount=10.0),
                make_flow("out", prim_name, "Snk", substance="Energy", usability="Product"),
            ]
        sys_df = pd.DataFrame(systems)
        int_df = pd.DataFrame(interactions)
        m = BertModel(sys_df, int_df, seed=42)

        for _ in range(5):
            m.step()
        activity_short = list(m.agents)[0].state.get("activity", 0.0)

        m2 = BertModel(sys_df, int_df, seed=42)
        for _ in range(50):
            m2.step()
        activity_long = list(m2.agents)[0].state.get("activity", 0.0)

        assert abs(activity_short - activity_long) < 0.001, \
            f"{prim_name} output differs with history length: short={activity_short:.4f} long={activity_long:.4f}"
    return True, f"OK (all {len(markovian)} Markovian primitives produce identical output regardless of history)"


# ---------------------------------------------------------------------------
# Conservation engine (synchronous update mode + observation flows)
# ---------------------------------------------------------------------------


def test_observation_nondraining():
    """An observation flow reads a stock's level without draining it. The probe
    senses the frozen pre-tick level; the stock's storage is untouched by the read."""
    systems = [
        make_system("Stock", "Buffering"),
        make_system("Probe", "Sensing", agency_capacity=1.0),
    ]
    interactions = [
        make_flow("obs", "Stock", "Probe", substance="Energy", observation=True),
    ]
    sys_df = pd.DataFrame(systems)
    int_df = pd.DataFrame(interactions)
    model = BertModel(sys_df, int_df, seed=42, update_mode="synchronous")
    agents = {a.bert_id: a for a in model.agents}
    stock, probe = agents["test:Stock"], agents["test:Probe"]
    stock.state["storage"] = 100.0

    for _ in range(10):
        model.step()

    try:
        assert abs(stock.state["storage"] - 100.0) < 1e-9, \
            f"observation must not drain the stock, got {stock.state['storage']}"
        assert abs(probe.state["signal"] - 100.0) < 1e-9, \
            f"probe should sense level 100.0, got {probe.state['signal']}"
        return True, f"OK (level held at {stock.state['storage']:.1f}, sensed {probe.state['signal']:.1f})"
    except AssertionError as e:
        return False, str(e)


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
    ("Combining Asymmetric Perturbation", test_combining_asymmetric_perturbation),
    ("Modulating Bilinearity", test_modulating_bilinearity),
    ("Amplifying Metered Power", test_amplifying),
    ("Inverting Direction", test_inverting_direction),
    ("Buffering Temporal Lag", test_buffering_temporal_lag),
    ("Copying Non-Conservation", test_copying_non_conservation),
    ("Impeding Backpressure", test_impeding_backpressure),
    ("Composition: Negative Feedback", test_composition_negative_feedback),
    ("Composition: Sensing→Copying Fanout", test_composition_sensing_copying_fanout),
    ("Composition: Buffer Smooths Perturbation", test_composition_buffering_smooths_perturbation),
    ("Composition: Oscillator (limit cycle)", test_composition_oscillator),
    ("Anticipatory H-Conditioning", test_anticipatory_conditioning),
    ("Intentional H-Conditioning", test_intentional_conditioning),
    ("Regulated Buffer Circuit", test_regulated_buffer),
    ("Oscillator Circuit (loaded)", test_oscillator),
    ("Energy Processing Chain", test_energy_chain),
    ("Information Broadcast", test_info_broadcast),
    ("Error-Sensing Circuit", test_error_sensing_circuit),
    ("System-Level History", test_system_level_history),
    ("Markovian Primitives", test_markovian_primitives),
    ("Observation Non-Draining", test_observation_nondraining),
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
