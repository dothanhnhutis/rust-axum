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

pub mod jwt {
    use chrono::{Duration, Utc};
    use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
    use serde::{Deserialize, Serialize};

    const ACCESS_EXP: i64 = 15; // phút
    const REFRESH_EXP_DAYS: i64 = 7;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Claims {
        pub sub: String,
        pub exp: usize,
    }

    pub fn create_access_token(user_id: &str, secret: &str) -> anyhow::Result<String> {
        let exp = Utc::now() + Duration::minutes(ACCESS_EXP);

        let claims = Claims {
            sub: user_id.to_string(),
            exp: exp.timestamp() as usize,
        };

        Ok(encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )?)
    }

    pub fn verify_token(token: &str, secret: &str) -> anyhow::Result<Claims> {
        let data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )?;
        Ok(data.claims)
    }
}

use sha2::{Digest, Sha256};
pub fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token);
    hex::encode(hasher.finalize())
}
