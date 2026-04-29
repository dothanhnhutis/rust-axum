use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct RefreshTokenRow {
    pub id: String,
    pub user_id: String,
    pub revoked: bool,
    pub expires_at: DateTime<Utc>,
}
