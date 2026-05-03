use axum::{Json, extract::State, response::IntoResponse};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use validator::Validate;

use crate::{
    db::repositories::{
        auth_repo::{create_token, get_token, revoked_token},
        user_repo::find_user_by_email,
    },
    error_handler::AppError,
    utils::{blocking, hash_token, jwt::create_access_token, ok, password::verify_password},
    validators::ValidatedJson,
};

#[derive(Deserialize, Debug, Validate)]
#[serde(deny_unknown_fields)]
pub struct LoginRequest {
    #[validate(email(message = "Email và mật khẩu không hợp lệ."))]
    email: String,
    #[validate(length(min = 8, message = "Email và mật khẩu không hợp lệ."))]
    password: String,
}

pub async fn login_handler(
    State(db): State<PgPool>,
    State(jwt_secret): State<String>,
    ValidatedJson(payload): ValidatedJson<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = find_user_by_email(&db, &payload.email).await?;

    let is_valid =
        blocking(move || verify_password(&payload.password, &user.password_hash)).await?;

    if !is_valid {
        return Err(AppError::InvalidCredentials);
    }

    let access = create_access_token(&user.id.to_string(), &jwt_secret)?;

    let refresh = create_token(&db, &user.id).await?;

    Ok(ok(json!({
        "message": "Đăng nhập thành công",
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

pub async fn refresh(
    State(db): State<PgPool>,
    Json(payload): Json<RefreshRequest>,
) -> Result<impl IntoResponse, AppError> {
    let hash = hash_token(&payload.refresh_token);

    let record = get_token(&db, &hash).await?;

    if record.revoked || record.expires_at < chrono::Utc::now() {
        return Err(AppError::Unauthorized);
    }

    revoked_token(&db, &record.id).await?;

    let new_refresh = create_token(&db, &record.user_id).await?;
    let access = create_access_token(&record.user_id.to_string(), "secret")?;

    Ok(ok(json!({
        "message": "Refresh token thành công",
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

    Ok(ok(json!({
        "message": "Đăng xuất thành công",
    })))
}
