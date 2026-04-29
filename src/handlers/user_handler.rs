use axum::{extract::Extension, response::IntoResponse};
use serde_json::json;

use crate::{error_handler::AppError, middleware::CurrentUser, utils::ok};

pub async fn current_user(
    // Extension(user_id): Extension<String>,
    user: CurrentUser,
) -> Result<impl IntoResponse, AppError> {
    Ok(ok(json!({
        "user_id": user.user_id,

    })))
}
