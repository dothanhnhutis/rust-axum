use axum::{Json, http::StatusCode};

use crate::error_handler::{ApiError, ApiResponse, AppError, ErrorDetail};

pub mod password {
    use anyhow::Result;

    use argon2::{
        Algorithm, Argon2, Params, Version,
        password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core},
    };

    pub fn hash_password(password: &str) -> Result<String> {
        // 1. Định nghĩa cấu hình tối ưu
        let params = Params::new(65536, 3, 4, None).unwrap(); // 64MiB RAM, 3 vòng lặp, 4 luồng
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

        let salt = SaltString::generate(&mut rand_core::OsRng);

        // let argon2 = Argon2::default();

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

pub async fn blocking<F, T>(f: F) -> Result<T, AppError>
where
    F: FnOnce() -> Result<T, anyhow::Error> + Send + 'static,
    T: Send + 'static,
{
    tokio::task::spawn_blocking(f)
        .await
        .map_err(|_| AppError::Internal)?
        .map_err(|_| AppError::Internal)
}
