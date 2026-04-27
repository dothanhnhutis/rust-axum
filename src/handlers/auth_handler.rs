use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use serde_json::json;
use sqlx::{PgPool, prelude::FromRow};
use validator::Validate;

use crate::{
    error_handler::AppError,
    utils::{err, ok},
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
    println!("{payload:#?}");

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
    .await?;

    println!("{user:#?}");

    Ok(ok(json!({
        "user_id": "123"
    })))
}
