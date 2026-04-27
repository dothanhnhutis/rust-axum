use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

pub mod handlers;
pub mod router;
pub mod utils;

pub mod error_handler;
pub mod validators;

mod state;
pub use state::AppState;

pub async fn init_db_pool(database_url: &str) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(20) // Phù hợp VPS 4GB
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await?;
    Ok(pool)
}

// #[derive(Clone, Debug)]
// pub struct User {
//     pub id: String,
//     pub username: String,
//     pub email: String,
//     pub password_hash: String,
// }

// pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
//     sqlx::query_as!(
//         User,
//         r#"
//         SELECT id, email, username, password_hash
//         FROM users
//         WHERE email = $1
//         "#,
//         email
//     )
//     .fetch_optional(pool)
//     .await
// }

// pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>> {
//     let row = sqlx::query_as!(
//         User,
//         r#"
//         SELECT id, email, username, password_hash
//         FROM users
//         WHERE email = $1
//         "#,
//         email
//     )
//     .fetch_optional(pool)
//     .await?;

//     Ok(row)
// }
