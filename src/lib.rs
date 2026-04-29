use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

pub mod handlers;
pub mod router;
pub mod utils;

pub mod error_handler;
pub mod validators;

pub mod middleware;
pub mod state;

pub mod db;

pub async fn init_db_pool(database_url: &str) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(20) // Phù hợp VPS 4GB
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await?;
    Ok(pool)
}
