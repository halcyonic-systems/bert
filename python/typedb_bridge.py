"""
TypeDB bridge for BERT Mesa runner.

Reads static model structure (systems, interactions, agent configs) and
writes simulation observations back. Uses plain `match` queries (concept rows),
not `fetch` (concept documents), matching the gov-graphs API pattern.
"""

import pandas as pd
from typedb.driver import TypeDB
from typedb.api.connection.credentials import Credentials
from typedb.api.connection.driver_options import DriverOptions
from typedb.api.connection.transaction import TransactionType


def connect(host="localhost:1729", username="admin", password="password"):
    return TypeDB.driver(host, Credentials(username, password), DriverOptions(is_tls_enabled=False))


def _val(concept):
    if concept is None:
        return None
    if hasattr(concept, "get_value"):
        return concept.get_value()
    if hasattr(concept, "get_label"):
        return str(concept.get_label())
    return str(concept)


def _query_rows(tx, query, columns):
    answer = tx.query(query).resolve()
    results = []
    for row in answer.as_concept_rows():
        d = {}
        for col in columns:
            try:
                d[col] = _val(row.get(col))
            except Exception:
                d[col] = None
        results.append(d)
    return results


def read_systems(driver, db: str, model_name: str) -> pd.DataFrame:
    query = f"""
        match
            $s isa system, has bert_id $id, has display_name $name,
                has archetype $arch, has time_constant $tc,
                has system_level $lvl, has complexity_kind $ck;
            $id like "^{model_name}:.*";
    """
    with driver.transaction(db, TransactionType.READ) as tx:
        rows = _query_rows(tx, query, ["id", "name", "arch", "tc", "lvl", "ck"])

    if not rows:
        return pd.DataFrame()

    df = pd.DataFrame(rows)
    df = df.rename(columns={
        "id": "bert_id", "name": "display_name", "arch": "archetype",
        "tc": "time_constant", "lvl": "system_level", "ck": "complexity_kind",
    })

    agent_query = f"""
        match
            $s isa system, has bert_id $id;
            (system: $s, config: $a) isa has_agent_config;
            $a isa agent_model, has agent_kind $ak, has agency_capacity $ac;
            $id like "^{model_name}:.*";
    """
    with driver.transaction(db, TransactionType.READ) as tx:
        agent_rows = _query_rows(tx, agent_query, ["id", "ak", "ac"])

    if agent_rows:
        agent_df = pd.DataFrame(agent_rows).rename(columns={
            "id": "bert_id", "ak": "agent_kind", "ac": "agency_capacity",
        })
        df = df.merge(agent_df, on="bert_id", how="left")
    else:
        df["agent_kind"] = None
        df["agency_capacity"] = None

    return df


def read_interactions(driver, db: str, model_name: str) -> pd.DataFrame:
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
    """
    with driver.transaction(db, TransactionType.READ) as tx:
        rows = _query_rows(tx, query, ["id", "name", "st", "use", "itype", "amt", "src_id", "snk_id"])

    if not rows:
        return pd.DataFrame()

    df = pd.DataFrame(rows)
    df = df.rename(columns={
        "id": "bert_id", "name": "display_name", "st": "substance_type",
        "use": "usability", "itype": "interaction_type", "amt": "amount",
        "src_id": "source_id", "snk_id": "sink_id",
    })
    return df


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
    with driver.transaction(db, TransactionType.WRITE) as tx:
        # Delete old status
        tx.query(f"""
            match $r isa simulation_run, has run_id "{run_id}", has run_status $old;
            delete $old of $r;
        """).resolve()

        # Insert new status
        insert_q = f"""
            match $r isa simulation_run, has run_id "{run_id}";
            insert $r has run_status "{status}";
        """
        tx.query(insert_q).resolve()

        if tick_count is not None:
            tx.query(f"""
                match $r isa simulation_run, has run_id "{run_id}", has tick_count $old;
                delete $old of $r;
            """).resolve()
            tx.query(f"""
                match $r isa simulation_run, has run_id "{run_id}";
                insert $r has tick_count {tick_count};
            """).resolve()

        tx.commit()


def write_flow_observations(driver, db: str, run_id: str, tick: int, observations: list[dict]):
    if not observations:
        return

    with driver.transaction(db, TransactionType.WRITE) as tx:
        for i, obs in enumerate(observations):
            iid = obs["interaction_id"]
            amt = obs["amount"]
            tx.query(f"""
                match $ix isa interaction, has bert_id "{iid}";
                match $r isa simulation_run, has run_id "{run_id}";
                insert $fo isa flow_observation,
                    has run_id "{run_id}", has tick {tick}, has observed_amount {amt};
                (observer: $fo, interaction: $ix) isa observes_interaction;
                (run: $r, observation: $fo) isa run_observation;
            """).resolve()
        tx.commit()


def write_system_observations(driver, db: str, run_id: str, tick: int, observations: list[dict]):
    if not observations:
        return

    with driver.transaction(db, TransactionType.WRITE) as tx:
        for i, obs in enumerate(observations):
            sid = obs["system_id"]
            key = obs["key"]
            val = obs["value"]
            tx.query(f"""
                match $s isa system, has bert_id "{sid}";
                match $r isa simulation_run, has run_id "{run_id}";
                insert $so isa system_observation,
                    has run_id "{run_id}", has tick {tick},
                    has observation_key "{key}", has observed_value {val};
                (observer: $so, system: $s) isa observes_system;
                (run: $r, observation: $so) isa run_observation;
            """).resolve()
        tx.commit()
