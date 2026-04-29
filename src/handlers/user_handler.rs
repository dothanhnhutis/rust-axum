use axum::response::IntoResponse;
use serde_json::json;

use crate::{error_handler::AppError, middleware::CurrentUser, utils::ok};

pub async fn current_user(user: CurrentUser) -> Result<impl IntoResponse, AppError> {
    Ok(ok(json!(user)))
}
