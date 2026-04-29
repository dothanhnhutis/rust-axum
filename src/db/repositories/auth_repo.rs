use sqlx::PgPool;

use crate::{
    db::models::refresh_token_row::RefreshTokenRow, error_handler::AppError, utils::hash_token,
};

pub async fn create_token(pool: &PgPool, user_id: &str) -> Result<String, AppError> {
    let refresh = uuid::Uuid::new_v4().to_string();
    let refresh_hash = hash_token(&refresh);
    sqlx::query!(
        "INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
         VALUES ($1, $2, now() + interval '7 days')",
        user_id,
        refresh_hash
    )
    .execute(pool)
    .await?;

    Ok(refresh)
}

pub async fn get_token(pool: &PgPool, token_hash: &str) -> Result<RefreshTokenRow, AppError> {
    let record = sqlx::query_as!(
        RefreshTokenRow,
        "SELECT id, user_id, revoked, expires_at
         FROM refresh_tokens
         WHERE token_hash = $1",
        token_hash
    )
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::Unauthorized)?;

    Ok(record)
}

pub async fn revoked_token(pool: &PgPool, token_id: &str) -> Result<(), AppError> {
    sqlx::query!(
        "UPDATE refresh_tokens SET revoked = true WHERE id = $1",
        token_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
