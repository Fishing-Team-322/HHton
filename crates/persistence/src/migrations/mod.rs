//! Database schema management utilities.

use sqlx::PgPool;

/// Runs all pending migrations against the configured PostgreSQL database.
pub async fn run(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}
