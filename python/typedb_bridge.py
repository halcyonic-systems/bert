"""
TypeDB bridge for BERT Mesa runner.

Reads static model structure (systems, interactions, agent configs) and
writes simulation observations back. Follows the gov-graphs API pattern.
"""

import pandas as pd
from typedb.driver import TypeDB
from typedb.api.connection.credentials import Credentials
from typedb.api.connection.driver_options import DriverOptions
from typedb.api.connection.transaction import TransactionType


def connect(host="localhost:1729", username="admin", password="password"):
    return TypeDB.driver(host, Credentials(username, password), DriverOptions(is_tls_enabled=False))


def _rows_to_dicts(answer, columns):
    results = []
    for row in answer.as_concept_rows():
        d = {}
        for col in columns:
            try:
                val = row.get(col)
                if hasattr(val, "get_value"):
                    d[col] = val.get_value()
                elif hasattr(val, "get_label"):
                    d[col] = str(val.get_label())
                else:
                    d[col] = str(val)
            except Exception:
                d[col] = None
        results.append(d)
    return results


def read_systems(driver, db: str, model_name: str) -> pd.DataFrame:
    """Read all systems for a model, including agent config if present."""
    query = f"""
        match
            $s isa system, has bert_id $id, has display_name $name,
                has archetype $arch, has time_constant $tc,
                has system_level $lvl, has complexity_kind $ck;
            $id like "^{model_name}:.*";
        fetch {{
            "bert_id": $id,
            "display_name": $name,
            "archetype": $arch,
            "time_constant": $tc,
            "system_level": $lvl,
            "complexity_kind": $ck
        }};
    """
    with driver.transaction(db, TransactionType.READ) as tx:
        rows = _rows_to_dicts(
            tx.query(query).resolve(),
            ["bert_id", "display_name", "archetype", "time_constant", "system_level", "complexity_kind"],
        )

    if not rows:
        return pd.DataFrame()

    df = pd.DataFrame(rows)

    agent_query = f"""
        match
            $s isa system, has bert_id $id;
            (system: $s, config: $a) isa has_agent_config;
            $a isa agent_model, has agent_kind $ak, has agency_capacity $ac;
            $id like "^{model_name}:.*";
        fetch {{
            "bert_id": $id,
            "agent_kind": $ak,
            "agency_capacity": $ac
        }};
    """
    with driver.transaction(db, TransactionType.READ) as tx:
        agent_rows = _rows_to_dicts(
            tx.query(agent_query).resolve(),
            ["bert_id", "agent_kind", "agency_capacity"],
        )

    if agent_rows:
        agent_df = pd.DataFrame(agent_rows)
        df = df.merge(agent_df, on="bert_id", how="left")
    else:
        df["agent_kind"] = None
        df["agency_capacity"] = None

    return df


def read_interactions(driver, db: str, model_name: str) -> pd.DataFrame:
    """Read all interactions (flows/forces) for a model with source/sink."""
    query = f"""
        match
            $ix isa interaction, has bert_id $id, has display_name $name,
                has substance_type $st, has usability $use,
                has interaction_type $itype, has amount $amt;
            (participant: $src, interaction: $ix) isa participates_in, has participation_role "source";
            $src has bert_id $src_id;
            (participant: $snk, interaction: $ix) isa participates_in, has participation_role "sink";
            $snk has bert_id $snk_id;
            $id like "^{model_name}:.*";
        fetch {{
            "bert_id": $id,
            "display_name": $name,
            "substance_type": $st,
            "usability": $use,
            "interaction_type": $itype,
            "amount": $amt,
            "source_id": $src_id,
            "sink_id": $snk_id
        }};
    """
    with driver.transaction(db, TransactionType.READ) as tx:
        rows = _rows_to_dicts(
            tx.query(query).resolve(),
            ["bert_id", "display_name", "substance_type", "usability",
             "interaction_type", "amount", "source_id", "sink_id"],
        )

    return pd.DataFrame(rows) if rows else pd.DataFrame()


# --- Simulation write operations ---


def write_simulation_run(driver, db: str, run_id: str, model_ref: str,
                         seed: int, tick_count: int):
    query = f"""
        insert $r isa simulation_run,
            has run_id "{run_id}",
            has model_ref "{model_ref}",
            has seed {seed},
            has tick_count {tick_count},
            has run_status "Running";
    """
    with driver.transaction(db, TransactionType.WRITE) as tx:
        tx.query(query).resolve()
        tx.commit()


def update_run_status(driver, db: str, run_id: str, status: str, tick_count: int = None):
    if tick_count is not None:
        query = f"""
            match $r isa simulation_run, has run_id "{run_id}";
            delete $r has run_status $old;
            insert $r has run_status "{status}", has tick_count {tick_count};
        """
    else:
        query = f"""
            match $r isa simulation_run, has run_id "{run_id}";
            delete $r has run_status $old;
            insert $r has run_status "{status}";
        """
    with driver.transaction(db, TransactionType.WRITE) as tx:
        tx.query(query).resolve()
        tx.commit()


def write_flow_observations(driver, db: str, run_id: str, tick: int, observations: list[dict]):
    """Write per-tick flow observations. Each dict has interaction_id and amount."""
    if not observations:
        return

    statements = []
    for i, obs in enumerate(observations):
        iid = obs["interaction_id"]
        amt = obs["amount"]
        statements.append(f"""
            match $ix isa interaction, has bert_id "{iid}";
            match $r isa simulation_run, has run_id "{run_id}";
            insert $fo{i} isa flow_observation,
                has run_id "{run_id}", has tick {tick}, has observed_amount {amt};
            (observer: $fo{i}, interaction: $ix) isa observes_interaction;
            (run: $r, observation: $fo{i}) isa run_observation;
        """)

    with driver.transaction(db, TransactionType.WRITE) as tx:
        for stmt in statements:
            tx.query(stmt).resolve()
        tx.commit()


def write_system_observations(driver, db: str, run_id: str, tick: int, observations: list[dict]):
    """Write per-tick system observations. Each dict has system_id, key, value."""
    if not observations:
        return

    statements = []
    for i, obs in enumerate(observations):
        sid = obs["system_id"]
        key = obs["key"]
        val = obs["value"]
        statements.append(f"""
            match $s isa system, has bert_id "{sid}";
            match $r isa simulation_run, has run_id "{run_id}";
            insert $so{i} isa system_observation,
                has run_id "{run_id}", has tick {tick},
                has observation_key "{key}", has observed_value {val};
            (observer: $so{i}, system: $s) isa observes_system;
            (run: $r, observation: $so{i}) isa run_observation;
        """)

    with driver.transaction(db, TransactionType.WRITE) as tx:
        for stmt in statements:
            tx.query(stmt).resolve()
        tx.commit()
