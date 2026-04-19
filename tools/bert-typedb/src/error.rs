//! Error types for the BERT TypeDB transpiler.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TranspilerError {
    #[error("failed to connect to TypeDB at {host}: {source}")]
    Connection {
        host: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("schema violation on attribute {attribute}: {message}")]
    SchemaViolation { attribute: String, message: String },

    #[error("duplicate bert_id: {0} — every model:local_id pair must be unique")]
    DuplicateId(String),

    #[error("TypeQL query parse or execution error: {0}")]
    QueryParse(String),

    #[error("invalid BERT JSON: {0}")]
    InvalidModel(String),

    #[error("required field missing on {entity}: {field}")]
    MissingRequiredField { entity: String, field: String },

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON deserialization error: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("internal transpiler error: {0}")]
    Internal(String),
}

pub type TranspilerResult<T> = Result<T, TranspilerError>;
