//! CLI entry point for the BERT→TypeDB transpiler.
//!
//! Usage:
//!
//! ```sh
//! bert-typedb path/to/model.json
//! bert-typedb path/to/model.json --host localhost:1729 --db bert-models
//! bert-typedb path/to/model.json --model-name ethereum  # override file stem
//! ```
//!
//! Connects to the target TypeDB instance, creates the database if
//! missing, loads the BERT schema (idempotent), transpiles the JSON
//! model, and pushes all statements in one Write transaction.

use bert::bevy_app::data_model::WorldModel;
use bert_typedb::{transpile_and_push, Transpiler};
use clap::Parser;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(
    name = "bert-typedb",
    about = "Transpile a BERT JSON model into a TypeDB typed graph",
    version
)]
struct Cli {
    /// Path to a BERT JSON model file (e.g. bitcoin.json).
    path: PathBuf,

    /// TypeDB server address.
    #[arg(long, default_value = "localhost:1729")]
    host: String,

    /// TypeDB database name. Created if missing.
    #[arg(long, default_value = "bert-models")]
    db: String,

    /// Model name used as the `bert_id` namespace prefix
    /// (e.g. `bitcoin:S0`). Defaults to the JSON file stem.
    #[arg(long)]
    model_name: Option<String>,

    /// Skip the schema load step (useful if schema is already current).
    #[arg(long)]
    skip_schema: bool,
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(msg) => {
            eprintln!("bert-typedb: error: {msg}");
            ExitCode::FAILURE
        }
    }
}

async fn run(cli: Cli) -> Result<(), String> {
    // Derive model_name from file stem if not provided.
    let model_name = cli
        .model_name
        .clone()
        .or_else(|| {
            cli.path
                .file_stem()
                .and_then(|s| s.to_str())
                .map(String::from)
        })
        .ok_or("unable to derive model_name from path; pass --model-name")?;

    eprintln!("bert-typedb: loading {}", cli.path.display());
    let bytes = std::fs::read(&cli.path)
        .map_err(|e| format!("failed to read {}: {e}", cli.path.display()))?;
    let model: WorldModel = serde_json::from_slice(&bytes)
        .map_err(|e| format!("failed to parse {}: {e}", cli.path.display()))?;
    eprintln!(
        "  parsed: {} systems, {} interactions, {} external entities",
        model.systems.len(),
        model.interactions.len(),
        model.environment.sources.len() + model.environment.sinks.len()
    );

    eprintln!("bert-typedb: connecting to {}", cli.host);
    let t = Transpiler::connect(&cli.host, &cli.db)
        .await
        .map_err(|e| format!("{e}"))?;

    eprintln!("bert-typedb: ensuring database '{}'", cli.db);
    t.ensure_database().await.map_err(|e| format!("{e}"))?;

    if !cli.skip_schema {
        eprintln!("bert-typedb: loading schema (idempotent)");
        t.load_schema().await.map_err(|e| format!("{e}"))?;
    }

    eprintln!(
        "bert-typedb: transpiling model '{}' into {}",
        model_name, cli.db
    );
    let summary = transpile_and_push(&model, &model_name, &t)
        .await
        .map_err(|e| format!("{e}"))?;

    println!(
        "✓ {} statements executed in {:?}",
        summary.statements_executed, summary.elapsed
    );

    Ok(())
}
