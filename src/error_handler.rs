use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::validators::format_validation_errors;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),
}

// impl IntoResponse for ServerError {
//     fn into_response(self) -> Response {
//         match self {
//             ServerError::ValidationError(err) => {
//                 let body = format_validation_errors(err);
//                 (StatusCode::BAD_REQUEST, axum::Json(body)).into_response()
//             }
//             ServerError::AxumFormRejection(err) => {
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

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(err) => {
                let body = format_validation_errors(err);
                (StatusCode::BAD_REQUEST, axum::Json(body)).into_response()
            }

            ServerError::AxumJsonRejection(err) => {
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
