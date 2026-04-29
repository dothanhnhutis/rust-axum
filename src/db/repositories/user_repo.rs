use sqlx::PgPool;

use crate::{db::models::user_row::UserRow, error_handler::AppError};

pub async fn find_user_by_email(pool: &PgPool, email: &str) -> Result<UserRow, AppError> {
    let user = sqlx::query_as!(
        UserRow,
        r#"
        SELECT id, email, username, password_hash
        FROM users
        WHERE email = $1
        "#,
        email
    )
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::InvalidCredentials)?;

    Ok(user)
}
