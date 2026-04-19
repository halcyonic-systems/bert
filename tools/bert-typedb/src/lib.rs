//! # BERT TypeDB Transpiler
//!
//! Projects BERT JSON models into TypeDB 3.x typed graphs. Foundation for
//! downstream integrations (Mesa bridge, cross-model queries, Facets).
//!
//! See `docs/bert-typedb-schema.md` in the BERT repo for the schema design
//! this transpiler targets.

pub mod driver;
pub mod error;
pub mod escape;
pub mod insert;
pub mod schema;
pub mod validate;

pub use driver::{SyncSummary, Transpiler};
pub use error::TranspilerError;
pub use escape::escape_typeql_string;
pub use insert::model_to_typeql;
pub use schema::SCHEMA_TQL;
pub use validate::{validate, Severity, ValidationIssue};

use bert::bevy_app::data_model::WorldModel;

/// End-to-end transpile: validate the model, emit TypeQL, and push to TypeDB.
/// Assumes the target database already exists and the schema is loaded.
/// For a first-time run, the caller should `ensure_database()` and
/// `load_schema()` first (or use a convenience wrapper that chains them).
pub async fn transpile_and_push(
    model: &WorldModel,
    model_name: &str,
    transpiler: &Transpiler,
) -> Result<SyncSummary, TranspilerError> {
    // 1. Validate — fail fast on any errors before touching TypeDB.
    let issues = validate(model);
    let errors: Vec<&ValidationIssue> = issues
        .iter()
        .filter(|i| i.severity == Severity::Error)
        .collect();
    if !errors.is_empty() {
        return Err(TranspilerError::InvalidModel(format!(
            "{} validation error(s): {}",
            errors.len(),
            errors
                .iter()
                .map(|e| format!("{}: {}", e.location, e.message))
                .collect::<Vec<_>>()
                .join("; ")
        )));
    }

    // 2. Emit TypeQL.
    let statements = model_to_typeql(model, model_name)?;

    // 3. Push in one Write transaction.
    transpiler.push_statements(&statements).await
}
