use axum::extract::State;
use serde::Deserialize;
use sqlx::PgPool;
use validator::Validate;

use crate::validators::ValidatedJson;

#[derive(Deserialize, Debug, Validate)]
#[serde(deny_unknown_fields)]
pub struct LoginRequest {
    #[validate(email(message = "Email không hợp lệ."))]
    email: String,
    #[validate(length(min = 8, message = "Email và mật khẩu không hợp lệ."))]
    password: String,
}

pub async fn login_handler(
    State(db): State<PgPool>,
    ValidatedJson(payload): ValidatedJson<LoginRequest>,
) {
    println!("{payload:#?}");
}
