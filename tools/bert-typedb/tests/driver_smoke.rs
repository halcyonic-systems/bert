//! Smoke tests against a live TypeDB server.
//!
//! Gated by the `integration` feature — requires a local TypeDB 3.x
//! instance listening on `localhost:1729` with default admin/password
//! credentials. Run with:
//!
//! ```sh
//! cargo test -p bert-typedb --features integration --test driver_smoke
//! ```
//!
//! Each test uses a unique database name to avoid collision between
//! concurrent runs, and drops the database on teardown.

#![cfg(feature = "integration")]

use bert_typedb::{Transpiler, SCHEMA_TQL};

fn unique_db_name(prefix: &str) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{prefix}_{nanos}")
}

#[tokio::test]
async fn connect_and_roundtrip_schema() {
    let db_name = unique_db_name("bert_typedb_smoke");
    let t = Transpiler::connect("localhost:1729", &db_name)
        .await
        .expect("connect to local TypeDB");

    t.ensure_database().await.expect("create test database");

    t.load_schema().await.expect("load BERT schema");

    // Roundtrip: push a trivial entity, then drop the database.
    let statements = vec![
        r#"insert $m isa bert_model, has model_name "smoke_test", has description "smoke";"#
            .to_string(),
    ];
    let summary = t
        .push_statements(&statements)
        .await
        .expect("push one bert_model insert");
    assert_eq!(summary.statements_executed, 1);

    t.drop_database()
        .await
        .expect("drop test database on teardown");
}

#[test]
fn schema_constant_is_nonempty() {
    // Sanity check that the compile-time schema string made it through.
    // Runs even without the integration feature via `cfg!(feature = ...)`
    // — wait, no, the whole file is gated. This is gated too. But runs
    // without TypeDB.
    assert!(!SCHEMA_TQL.is_empty());
    assert!(SCHEMA_TQL.contains("define"));
}
