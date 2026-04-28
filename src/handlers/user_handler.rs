use axum::{extract::Extension, response::IntoResponse};
use serde_json::json;

use crate::{error_handler::AppError, utils::ok};

pub async fn protected(
    Extension(user_id): Extension<String>,
) -> Result<impl IntoResponse, AppError> {
    Ok(ok(json!({
        "user_id": user.id,

    })))
}
