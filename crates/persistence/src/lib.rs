//! Database connectivity and persistence utilities.

use sqlx::postgres::PgPool;
use tracing::info;

/// Thin wrapper around a sqlx connection pool for future customization.
#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Instantiate the database abstraction from an existing connection pool.
    pub fn new(pool: PgPool) -> Self {
        info!(target: "persistence.database", "initialising PostgreSQL connection pool");
        Self { pool }
    }

    /// Expose a reference to the raw connection pool for migrations or diagnostics.
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}
