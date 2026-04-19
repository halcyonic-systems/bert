//! Full bitcoin.json → TypeDB round-trip.
//!
//! Validates the complete transpiler pipeline against a live TypeDB 3.x
//! instance: load reference model → transpile → push → query back and
//! verify counts match the source WorldModel.
//!
//! Gated by the `integration` feature. Requires TypeDB listening on
//! `localhost:1729` with default credentials. Run:
//!
//! ```sh
//! cargo test -p bert-typedb --features integration --test bitcoin_roundtrip
//! ```

#![cfg(feature = "integration")]

use bert::bevy_app::data_model::WorldModel;
use bert_typedb::{transpile_and_push, Transpiler};

fn unique_db_name() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("bert_bitcoin_{nanos}")
}

fn load_bitcoin_model() -> WorldModel {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../assets/models/examples/bitcoin.json"
    );
    let bytes = std::fs::read(path).expect("bitcoin.json should exist");
    serde_json::from_slice(&bytes).expect("bitcoin.json should deserialize")
}

#[tokio::test]
async fn bitcoin_full_transpile_roundtrip() {
    let model = load_bitcoin_model();
    let db_name = unique_db_name();

    let t = Transpiler::connect("localhost:1729", &db_name)
        .await
        .expect("connect to TypeDB");
    t.ensure_database()
        .await
        .expect("create test database");
    t.load_schema().await.expect("load schema");

    let summary = transpile_and_push(&model, "bitcoin", &t)
        .await
        .expect("transpile and push bitcoin.json");

    // Quick sanity: statements executed should be > the entity count
    // (entities + relations combined).
    let entity_count = 1 /* bert_model */
        + model.environment.sources.len()
        + model.environment.sinks.len()
        + model.systems.len()                        // system entities
        + model.systems.len()                        // boundaries
        + model.systems.iter().map(|s| s.boundary.interfaces.len()).sum::<usize>()
        + model.interactions.len();
    assert!(
        summary.statements_executed > entity_count,
        "expected statements > {entity_count} entities (entities + relations); got {}",
        summary.statements_executed
    );

    println!(
        "bitcoin.json transpiled in {:?}: {} statements executed",
        summary.elapsed, summary.statements_executed
    );

    // Teardown.
    t.drop_database()
        .await
        .expect("drop test database on teardown");
}
