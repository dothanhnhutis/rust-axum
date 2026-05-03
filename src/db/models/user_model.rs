use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(FromRow, Debug, Clone)]
pub struct UserRow {
    pub id: String,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub status: String,
    pub deactivated_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Serialize)]
pub struct CurrentUser {
    pub id: String,
    pub email: String,
    pub username: String,
    pub status: String,
    pub deactivated_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
