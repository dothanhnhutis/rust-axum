use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Json};

#[derive(FromRow, Debug, Clone, Serialize)]
pub struct RoleRow {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: String,
    pub deactivated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub can_delete: bool,
    pub can_update: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(FromRow, Debug, Clone, Serialize)]
pub struct RolePermissionRow {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: String,
    pub deactivated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub can_delete: bool,
    pub can_update: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub join_at: DateTime<Utc>,
    pub permissions: Json<Vec<PermissionRow>>,
}

#[derive(FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct PermissionRow {
    pub id: String,
    pub code: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
}
