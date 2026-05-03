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

    #[error("Deactivated account")]
    DeactivatedAccount,

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
            AppError::BadRequest(msg) => err(
                StatusCode::BAD_REQUEST,
                ErrorDetail {
                    code: "BAD_REQUEST".to_string(),
                    message: Some(msg),
                    fields: None,
                },
            )
            .into_response(),

            AppError::InvalidCredentials => err(
                StatusCode::UNAUTHORIZED,
                ErrorDetail {
                    code: "INVALID_CREDENTIALS".to_string(),
                    message: Some("Email hoặc mật khẩu không đúng".to_string()),
                    fields: None,
                },
            )
            .into_response(),
            AppError::Unauthorized => err(
                StatusCode::UNAUTHORIZED,
                ErrorDetail {
                    code: "UNAUTHORIZED".to_string(),
                    message: Some("Unauthorized".to_string()),
                    fields: None,
                },
            )
            .into_response(),

            AppError::Internal => err(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail {
                    code: "INTERNAL_ERROR".to_string(),
                    message: Some("Something went wrong".to_string()),
                    fields: None,
                },
            )
            .into_response(),
            AppError::NotFound => err(
                StatusCode::NOT_FOUND,
                ErrorDetail {
                    code: "NOT_FOUND".to_string(),
                    message: Some("Resource not found".to_string()),
                    fields: None,
                },
            )
            .into_response(),
            AppError::DeactivatedAccount => err(
                StatusCode::UNAUTHORIZED,
                ErrorDetail {
                    code: "DEACTIVATED_ACCOUNT".to_string(),
                    message: Some("Deactivated account".to_string()),
                    fields: None,
                },
            )
            .into_response(),

            AppError::ValidationError(err) => {
                println!("{err:#?}");

                let fields = format_validation_errors(err);

                err_with_fields(
                    "VALIDATION_ERROR",
                    "Dữ liệu không hợp lệ",
                    fields,
                    StatusCode::BAD_REQUEST,
                )
                .into_response()
            }
            AppError::AxumJsonRejection(error) => {
                let (code, message) = match error {
                    JsonRejection::JsonSyntaxError(_) => ("BAD_JSON", "JSON không hợp lệ"),
                    JsonRejection::JsonDataError(e) => {
                        if e.to_string().contains("unknown field") {
                            ("INVALID_REQUEST", "Request chứa field không hợp lệ")
                        } else {
                            ("INVALID_REQUEST", "Dữ liệu không hợp lệ")
                        }
                    }
                    JsonRejection::MissingJsonContentType(_) => {
                        ("BAD_REQUEST", "Content-Type phải là application/json")
                    }
                    _ => ("BAD_REQUEST", "Request không hợp lệ"),
                };
                err(
                    StatusCode::BAD_REQUEST,
                    ErrorDetail {
                        code: code.to_string(),
                        message: Some(message.to_string()),
                        fields: None,
                    },
                )
                .into_response()
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
