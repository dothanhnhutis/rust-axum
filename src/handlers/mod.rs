use axum::{http::StatusCode, response::IntoResponse};

pub mod auth_handler;

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "NOT_FOUND")
}
