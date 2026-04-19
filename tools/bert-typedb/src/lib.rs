//! # BERT TypeDB Transpiler
//!
//! Projects BERT JSON models into TypeDB 3.x typed graphs. Foundation for
//! downstream integrations (Mesa bridge, cross-model queries, Facets).
//!
//! See `docs/bert-typedb-schema.md` in the BERT repo for the schema design
//! this transpiler targets.

pub mod error;
pub mod escape;
pub mod schema;
pub mod validate;

pub use error::TranspilerError;
pub use escape::escape_typeql_string;
pub use schema::SCHEMA_TQL;
pub use validate::{validate, Severity, ValidationIssue};
