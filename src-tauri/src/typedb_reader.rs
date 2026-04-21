use futures::StreamExt;
use serde::{Deserialize, Serialize};
use typedb_driver::{Credentials, DriverOptions, TransactionType, TypeDBDriver};

const DEFAULT_HOST: &str = "localhost:1729";
const DEFAULT_USERNAME: &str = "admin";
const DEFAULT_PASSWORD: &str = "password";

pub async fn connect() -> Result<TypeDBDriver, String> {
    let opts = DriverOptions::new(false, None).map_err(|e| format!("DriverOptions: {e}"))?;
    TypeDBDriver::new(
        DEFAULT_HOST,
        Credentials::new(DEFAULT_USERNAME, DEFAULT_PASSWORD),
        opts,
    )
    .await
    .map_err(|e| format!("TypeDB connection failed: {e}"))
}

fn extract_str(concept: Option<&typedb_driver::concept::Concept>, field: &str) -> Result<String, String> {
    concept
        .ok_or_else(|| format!("missing {field}"))?
        .try_get_string()
        .map(|s| s.to_string())
        .ok_or_else(|| format!("{field} is not a string"))
}

fn extract_i64(concept: Option<&typedb_driver::concept::Concept>, field: &str) -> Result<i64, String> {
    concept
        .ok_or_else(|| format!("missing {field}"))?
        .try_get_integer()
        .ok_or_else(|| format!("{field} is not an integer"))
}

fn extract_f64(concept: Option<&typedb_driver::concept::Concept>, field: &str) -> Result<f64, String> {
    concept
        .ok_or_else(|| format!("missing {field}"))?
        .try_get_double()
        .ok_or_else(|| format!("{field} is not a double"))
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RunStatus {
    pub run_id: String,
    pub status: String,
    pub tick_count: u64,
    pub total_ticks: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RunInfo {
    pub run_id: String,
    pub status: String,
    pub tick_count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FlowTimeseries {
    pub interaction_id: String,
    pub name: String,
    pub ticks: Vec<u64>,
    pub values: Vec<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SystemTimeseries {
    pub system_id: String,
    pub name: String,
    pub key: String,
    pub ticks: Vec<u64>,
    pub values: Vec<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SimulationResults {
    pub run_id: String,
    pub flow_timeseries: Vec<FlowTimeseries>,
    pub system_timeseries: Vec<SystemTimeseries>,
}

pub async fn query_run_status(driver: &TypeDBDriver, db: &str, run_id: &str) -> Result<RunStatus, String> {
    let query = format!(
        r#"match $r isa simulation_run, has run_id "{run_id}", has run_status $st, has tick_count $tc;"#
    );

    let tx = driver
        .transaction(db, TransactionType::Read)
        .await
        .map_err(|e| format!("Transaction: {e}"))?;

    let answer = tx.query(&query).await.map_err(|e| format!("Query: {e}"))?;
    let mut stream = answer.into_rows();

    if let Some(row_result) = stream.next().await {
        let row = row_result.map_err(|e| format!("Row: {e}"))?;
        let status = extract_str(row.get("st").map_err(|e| format!("{e}"))?, "st")?;
        let tick_count = extract_i64(row.get("tc").map_err(|e| format!("{e}"))?, "tc")? as u64;

        Ok(RunStatus {
            run_id: run_id.to_string(),
            status,
            tick_count,
            total_ticks: 0,
        })
    } else {
        Err(format!("No simulation run found with id '{run_id}'"))
    }
}

pub async fn query_runs(driver: &TypeDBDriver, db: &str, model_name: &str) -> Result<Vec<RunInfo>, String> {
    let query = format!(
        r#"match $r isa simulation_run, has run_id $rid, has model_ref "{model_name}", has run_status $st, has tick_count $tc;"#
    );

    let tx = driver
        .transaction(db, TransactionType::Read)
        .await
        .map_err(|e| format!("Transaction: {e}"))?;

    let answer = tx.query(&query).await.map_err(|e| format!("Query: {e}"))?;
    let mut stream = answer.into_rows();
    let mut runs = Vec::new();

    while let Some(row_result) = stream.next().await {
        let row = row_result.map_err(|e| format!("Row: {e}"))?;
        let run_id = extract_str(row.get("rid").map_err(|e| format!("{e}"))?, "rid")?;
        let status = extract_str(row.get("st").map_err(|e| format!("{e}"))?, "st")?;
        let tick_count = extract_i64(row.get("tc").map_err(|e| format!("{e}"))?, "tc")? as u64;

        runs.push(RunInfo { run_id, status, tick_count });
    }

    Ok(runs)
}

pub async fn query_results(driver: &TypeDBDriver, db: &str, run_id: &str) -> Result<SimulationResults, String> {
    let flow_query = format!(
        r#"match
            $fo isa flow_observation, has run_id "{run_id}", has tick $t, has observed_amount $amt;
            (observer: $fo, interaction: $ix) isa observes_interaction;
            $ix has bert_id $iid, has display_name $iname;"#
    );

    let sys_query = format!(
        r#"match
            $so isa system_observation, has run_id "{run_id}", has tick $t, has observation_key $k, has observed_value $v;
            (observer: $so, system: $s) isa observes_system;
            $s has bert_id $sid, has display_name $sname;"#
    );

    let tx = driver
        .transaction(db, TransactionType::Read)
        .await
        .map_err(|e| format!("Transaction: {e}"))?;

    // Flow observations
    let flow_answer = tx.query(&flow_query).await.map_err(|e| format!("Flow query: {e}"))?;
    let mut flow_stream = flow_answer.into_rows();
    let mut flow_map: std::collections::HashMap<String, FlowTimeseries> = std::collections::HashMap::new();

    while let Some(row_result) = flow_stream.next().await {
        let row = row_result.map_err(|e| format!("Flow row: {e}"))?;
        let iid = extract_str(row.get("iid").map_err(|e| format!("{e}"))?, "iid")?;
        let name = extract_str(row.get("iname").map_err(|e| format!("{e}"))?, "iname")?;
        let tick = extract_i64(row.get("t").map_err(|e| format!("{e}"))?, "t")? as u64;
        let amount = extract_f64(row.get("amt").map_err(|e| format!("{e}"))?, "amt")?;

        let entry = flow_map.entry(iid.clone()).or_insert_with(|| FlowTimeseries {
            interaction_id: iid,
            name,
            ticks: Vec::new(),
            values: Vec::new(),
        });
        entry.ticks.push(tick);
        entry.values.push(amount);
    }

    // System observations
    let sys_answer = tx.query(&sys_query).await.map_err(|e| format!("System query: {e}"))?;
    let mut sys_stream = sys_answer.into_rows();
    let mut sys_map: std::collections::HashMap<String, SystemTimeseries> = std::collections::HashMap::new();

    while let Some(row_result) = sys_stream.next().await {
        let row = row_result.map_err(|e| format!("System row: {e}"))?;
        let sid = extract_str(row.get("sid").map_err(|e| format!("{e}"))?, "sid")?;
        let name = extract_str(row.get("sname").map_err(|e| format!("{e}"))?, "sname")?;
        let key = extract_str(row.get("k").map_err(|e| format!("{e}"))?, "k")?;
        let tick = extract_i64(row.get("t").map_err(|e| format!("{e}"))?, "t")? as u64;
        let value = extract_f64(row.get("v").map_err(|e| format!("{e}"))?, "v")?;

        let map_key = format!("{sid}:{key}");
        let entry = sys_map.entry(map_key).or_insert_with(|| SystemTimeseries {
            system_id: sid,
            name,
            key,
            ticks: Vec::new(),
            values: Vec::new(),
        });
        entry.ticks.push(tick);
        entry.values.push(value);
    }

    Ok(SimulationResults {
        run_id: run_id.to_string(),
        flow_timeseries: flow_map.into_values().collect(),
        system_timeseries: sys_map.into_values().collect(),
    })
}
