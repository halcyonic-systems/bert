//! Async TypeDB driver wrapper.
//!
//! Thin layer over `typedb-driver 3.x` that exposes a BERT-shaped API:
//! connect, ensure database, load schema, push a batch of statements.
//! All connection/transaction management lives here so the CLI and
//! library callers never touch the driver crate directly.
//!
//! # Transaction strategy
//!
//! Per the Lane 1 research (`operations/sessions/2026-04-19/typedb-transpiler-research-reference.md`):
//!
//! - **Schema** transaction for `SCHEMA_TQL` loading. Exclusive; blocks
//!   concurrent writes. Done once at setup and on schema updates.
//! - **Write** transaction for all data inserts. Many `.query()` calls,
//!   one commit at the end — batches are cheap server-side buffering.
//! - **Read** transaction would be used for query-back verification
//!   (e.g. in integration tests); not needed in the transpiler's
//!   happy path.

use crate::error::TranspilerError;
use crate::schema::SCHEMA_TQL;
use std::time::{Duration, Instant};
use typedb_driver::{Credentials, DriverOptions, TransactionType, TypeDBDriver};

/// Default local TypeDB server address.
pub const DEFAULT_ADDRESS: &str = "localhost:1729";

/// Default credentials — matches TypeDB Community Edition out-of-box defaults.
/// Production deployments should pass explicit credentials via `connect_with`.
pub const DEFAULT_USERNAME: &str = "admin";
pub const DEFAULT_PASSWORD: &str = "password";

/// Summary of a successful push operation. Useful for CLI output and tests.
#[derive(Debug, Clone)]
pub struct SyncSummary {
    pub statements_executed: usize,
    pub elapsed: Duration,
}

/// Connection-holder for a specific database. Clone-able per the underlying
/// driver's semantics (the driver internally wraps an `Arc`).
pub struct Transpiler {
    driver: TypeDBDriver,
    db_name: String,
}

impl Transpiler {
    /// Connect to a TypeDB server using default credentials. For custom
    /// credentials use [`Self::connect_with`].
    pub async fn connect(host: &str, db_name: &str) -> Result<Self, TranspilerError> {
        Self::connect_with(host, db_name, DEFAULT_USERNAME, DEFAULT_PASSWORD).await
    }

    pub async fn connect_with(
        host: &str,
        db_name: &str,
        username: &str,
        password: &str,
    ) -> Result<Self, TranspilerError> {
        let driver_opts = DriverOptions::new(false, None).map_err(|e| {
            TranspilerError::Internal(format!("DriverOptions construction failed: {e}"))
        })?;
        let driver = TypeDBDriver::new(host, Credentials::new(username, password), driver_opts)
            .await
            .map_err(|e| TranspilerError::Connection {
                host: host.to_string(),
                source: Box::new(e),
            })?;
        Ok(Self {
            driver,
            db_name: db_name.to_string(),
        })
    }

    /// Create the target database if it does not exist. Idempotent.
    pub async fn ensure_database(&self) -> Result<(), TranspilerError> {
        let databases = self.driver.databases();
        let exists = databases
            .contains(&self.db_name)
            .await
            .map_err(map_driver_error)?;
        if !exists {
            databases
                .create(&self.db_name)
                .await
                .map_err(map_driver_error)?;
        }
        Ok(())
    }

    /// Load the BERT schema (from `SCHEMA_TQL`) into the database.
    /// Idempotent — TypeDB's `define` queries are safe to re-run.
    pub async fn load_schema(&self) -> Result<(), TranspilerError> {
        let tx = self
            .driver
            .transaction(&self.db_name, TransactionType::Schema)
            .await
            .map_err(map_driver_error)?;
        tx.query(SCHEMA_TQL).await.map_err(map_driver_error)?;
        tx.commit().await.map_err(map_driver_error)?;
        Ok(())
    }

    /// Push a batch of TypeQL statements in a single Write transaction.
    /// Fails fast if any statement is rejected; nothing is committed on
    /// error (TypeDB rolls back the transaction on close without commit).
    pub async fn push_statements(
        &self,
        statements: &[String],
    ) -> Result<SyncSummary, TranspilerError> {
        let start = Instant::now();
        let tx = self
            .driver
            .transaction(&self.db_name, TransactionType::Write)
            .await
            .map_err(map_driver_error)?;
        for stmt in statements {
            tx.query(stmt).await.map_err(map_driver_error)?;
        }
        tx.commit().await.map_err(map_driver_error)?;
        Ok(SyncSummary {
            statements_executed: statements.len(),
            elapsed: start.elapsed(),
        })
    }

    /// Drop the database. Intended for test teardown — calling this in
    /// production flow would be surprising.
    pub async fn drop_database(&self) -> Result<(), TranspilerError> {
        let databases = self.driver.databases();
        if let Ok(db) = databases.get(&self.db_name).await {
            db.delete().await.map_err(map_driver_error)?;
        }
        Ok(())
    }
}

/// Map a `typedb_driver::Error` to our `TranspilerError` with minimal loss.
/// For phase 1 this is deliberately coarse — we distinguish connection
/// failures from server-side rejections, and otherwise wrap the message.
/// Finer-grained classification (@values violations, duplicate @keys) can
/// come later by parsing `ServerError` messages when the transpiler sees
/// specific patterns in practice.
fn map_driver_error(e: typedb_driver::Error) -> TranspilerError {
    use typedb_driver::Error;
    match e {
        Error::Connection(conn) => TranspilerError::Connection {
            host: "unknown".to_string(),
            source: Box::new(conn),
        },
        Error::Server(srv) => {
            // ServerError::message is pub(crate) in typedb-driver 3.8.
            // Display impl returns the same string — use that instead.
            let msg = srv.to_string();
            if msg.contains("@key") || msg.to_lowercase().contains("unique") {
                TranspilerError::DuplicateId(msg)
            } else if msg.contains("@values") || msg.to_lowercase().contains("allowed values") {
                TranspilerError::SchemaViolation {
                    attribute: String::new(),
                    message: msg,
                }
            } else {
                TranspilerError::QueryParse(msg)
            }
        }
        other => TranspilerError::Internal(other.to_string()),
    }
}
