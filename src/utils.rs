use axum::{Json, http::StatusCode};

use crate::error_handler::{ApiError, ApiResponse, ErrorDetail};

pub mod password {
    use anyhow::Result;

    use argon2::{
        Argon2,
        password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core},
    };

    pub fn hash_password(password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut rand_core::OsRng);

        let argon2 = Argon2::default();

        let hash = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        Ok(hash)
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)?;

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

pub fn ok<T: serde::Serialize>(data: T) -> (StatusCode, Json<ApiResponse<T>>) {
    (
        StatusCode::OK,
        Json(ApiResponse {
            success: true,
            data: Some(data),
            message: None,
        }),
    )
}

pub fn err(code: &str, message: &str, status: StatusCode) -> (StatusCode, Json<ApiError>) {
    (
        status,
        Json(ApiError {
            success: false,
            error: ErrorDetail {
                code: code.to_string(),
                message: Some(message.to_string()),
                fields: None,
            },
        }),
    )
}

pub fn err_with_fields(
    code: &str,
    message: &str,
    fields: serde_json::Value,
    status: StatusCode,
) -> (StatusCode, Json<ApiError>) {
    (
        status,
        Json(ApiError {
            success: false,
            error: ErrorDetail {
                code: code.to_string(),
                message: Some(message.to_string()),
                fields: Some(fields),
            },
        }),
    )
}
