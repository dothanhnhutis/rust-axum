use crate::{
    utils::{err, err_with_fields},
    validators::format_validation_errors,
};
use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

#[derive(Serialize)]
pub struct ApiError {
    pub success: bool,
    pub error: ErrorDetail,
}

#[derive(Serialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: Option<String>,
    pub fields: Option<serde_json::Value>,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Unauthorized")]
    Unauthorized,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("{0}")]
    BadRequest(String),

    #[error("Not found")]
    NotFound,

    #[error("Internal server error")]
    Internal,

    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::BadRequest(msg) => {
                err("BAD_REQUEST", &msg, StatusCode::BAD_REQUEST).into_response()
            }

            AppError::InvalidCredentials => err(
                "INVALID_CREDENTIALS",
                "Email hoặc mật khẩu không đúng",
                StatusCode::UNAUTHORIZED,
            )
            .into_response(),

            AppError::Unauthorized => {
                err("UNAUTHORIZED", "Unauthorized", StatusCode::UNAUTHORIZED).into_response()
            }
            AppError::Internal => err(
                "INTERNAL_ERROR",
                "Something went wrong",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
            .into_response(),
            AppError::NotFound => {
                err("NOT_FOUND", "Resource not found", StatusCode::NOT_FOUND).into_response()
            }

            AppError::ValidationError(err) => {
                let fields = format_validation_errors(err);

                err_with_fields(
                    "VALIDATION_ERROR",
                    "Dữ liệu không hợp lệ",
                    fields,
                    StatusCode::BAD_REQUEST,
                )
                .into_response()
            }
            AppError::AxumJsonRejection(err) => {
                let message = err.to_string();
                // detect unknown field
                if message.contains("unknown field") {
                    let body = serde_json::json!({
                        "success": false,
                        "error": {
                            "code": "INVALID_REQUEST",
                            "message": "Request body không hợp lệ"
                        }
                    });

                    return (StatusCode::BAD_REQUEST, axum::Json(body)).into_response();
                }

                let body = serde_json::json!({
                    "success": false,
                    "error": {
                        "code": "BAD_JSON",
                        "message": message
                    }
                });

                (StatusCode::BAD_REQUEST, axum::Json(body)).into_response()
            }
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::BadRequest("Không tìm thấy dữ liệu".into()),

            sqlx::Error::Database(db_err) => {
                if let Some(code) = db_err.code() {
                    if code == "23505" {
                        return AppError::BadRequest("Dữ liệu đã tồn tại".into());
                    }
                }
                AppError::Internal
            }

            _ => AppError::Internal,
        }
    }
}

impl From<anyhow::Error> for AppError {
    fn from(_: anyhow::Error) -> Self {
        AppError::Internal
    }
}

// impl IntoResponse for AppError {
//     fn into_response(self) -> Response {
//         match self {
//             AppError::ValidationError(err) => {
//                 let body = format_validation_errors(err);
//                 (StatusCode::BAD_REQUEST, axum::Json(body)).into_response()
//             }
//             AppError::AxumFormRejection(err) => {
//                 let body = serde_json::json!({
//                     "success": false,
//                     "error": {
//                         "code": "BAD_REQUEST",
//                         "message": err.to_string()
//                     }
//                 });

//                 (StatusCode::BAD_REQUEST, axum::Json(body)).into_response()
//             }
//         }
//         .into_response()
//     }
// }
