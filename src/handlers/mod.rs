use crate::error_handler::AppError;

pub mod auth_handler;
pub mod user_handler;

pub async fn handler_404() -> AppError {
    AppError::NotFound
}
