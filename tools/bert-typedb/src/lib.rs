//! # BERT TypeDB Transpiler
//!
//! Projects BERT JSON models into TypeDB 3.x typed graphs. Foundation for
//! downstream integrations (Mesa bridge, cross-model queries, Facets).
//!
//! See `docs/bert-typedb-schema.md` in the BERT repo for the schema design
//! this transpiler targets.

pub mod error;
pub mod schema;

pub use error::TranspilerError;
pub use schema::SCHEMA_TQL;
