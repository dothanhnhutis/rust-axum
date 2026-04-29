use sqlx::prelude::FromRow;

#[derive(FromRow, Debug, Clone)]
pub struct UserRow {
    pub id: String,
    pub email: String,
    pub username: String,
    pub password_hash: String,
}
