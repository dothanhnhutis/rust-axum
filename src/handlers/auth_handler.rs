use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::json;
use sqlx::{PgPool, prelude::FromRow};
use validator::Validate;

use crate::{
    error_handler::AppError,
    utils::{
        blocking, err, hash_token,
        jwt::create_access_token,
        ok,
        password::{hash_password, verify_password},
    },
    validators::ValidatedJson,
};

#[derive(Deserialize, Debug, Validate)]
#[serde(deny_unknown_fields)]
pub struct LoginRequest {
    #[validate(email(message = "Email không hợp lệ."))]
    email: String,
    #[validate(length(min = 8, message = "Email và mật khẩu không hợp lệ."))]
    password: String,
}

#[derive(FromRow, Debug, Clone)]
struct UserRow {
    id: String,
    email: String,
    username: String,
    password_hash: String,
}

pub async fn login_handler(
    State(db): State<PgPool>,
    ValidatedJson(payload): ValidatedJson<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = sqlx::query_as!(
        UserRow,
        r#"
        SELECT id, email, username, password_hash
        FROM users
        WHERE email = $1
        "#,
        payload.email
    )
    .fetch_optional(&db)
    .await?
    .ok_or(AppError::InvalidCredentials)?;

    let is_valid =
        blocking(move || verify_password(&payload.password, &user.password_hash)).await?;

    if !is_valid {
        return Err(AppError::InvalidCredentials);
    }

    let access = create_access_token(&user.id.to_string(), "secret")?;
    let refresh = uuid::Uuid::new_v4().to_string();

    let refresh_hash = hash_token(&refresh);

    sqlx::query!(
        "INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
         VALUES ($1, $2, now() + interval '7 days')",
        user.id,
        refresh_hash
    )
    .execute(&db)
    .await?;

    Ok(ok(json!({
        "message": "Đăng nhập thành công",
        "user_id": user.id,
        "access_token": access,
        "refresh_token": refresh
    })))
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct RefreshRequest {
    #[validate(length(min = 10, message = "Refresh token không hợp lệ"))]
    pub refresh_token: String,
}

#[derive(Debug, FromRow)]
pub struct RefreshTokenRow {
    pub id: String,
    pub user_id: String,
    pub revoked: bool,
    pub expires_at: DateTime<Utc>,
}

pub async fn refresh(
    State(db): State<PgPool>,
    Json(payload): Json<RefreshRequest>,
) -> Result<impl IntoResponse, AppError> {
    let hash = hash_token(&payload.refresh_token);

    let record = sqlx::query_as!(
        RefreshTokenRow,
        "SELECT id, user_id, revoked, expires_at
         FROM refresh_tokens
         WHERE token_hash = $1",
        hash
    )
    .fetch_optional(&db)
    .await?
    .ok_or(AppError::Unauthorized)?;

    if record.revoked || record.expires_at < chrono::Utc::now() {
        return Err(AppError::Unauthorized);
    }

    // revoke old token (rotation)
    sqlx::query!(
        "UPDATE refresh_tokens SET revoked = true WHERE id = $1",
        record.id
    )
    .execute(&db)
    .await?;

    let new_refresh = uuid::Uuid::new_v4().to_string();
    let new_hash = hash_token(&new_refresh);

    sqlx::query!(
        "INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
         VALUES ($1, $2, now() + interval '7 days')",
        record.user_id,
        new_hash
    )
    .execute(&db)
    .await?;

    let access = create_access_token(&record.user_id.to_string(), "secret")?;

    Ok(Json(json!({
        "access_token": access,
        "refresh_token": new_refresh
    })))
}

pub async fn logout(
    State(db): State<PgPool>,
    Json(payload): Json<RefreshRequest>,
) -> Result<impl IntoResponse, AppError> {
    let hash = hash_token(&payload.refresh_token);

    sqlx::query!(
        "UPDATE refresh_tokens SET revoked = true WHERE token_hash = $1",
        hash
    )
    .execute(&db)
    .await?;

    Ok(StatusCode::NO_CONTENT)
}
