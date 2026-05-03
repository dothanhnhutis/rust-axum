use axum::{
    body::Body,
    extract::{FromRef, State},
    http::Request,
    middleware::Next,
    response::Response,
};
use serde::Serialize;
use sqlx::PgPool;

use crate::{
    db::{
        models::user_model::{CurrentUser, UserRow},
        repositories::user_repo::{find_user_by_id, find_user_permission_codes_by_id},
    },
    error_handler::AppError,
    state::AppState,
    utils::jwt::verify_token,
};

pub async fn auth_middleware(
    State(secret): State<String>, // 👈 và đ
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized)?;

    let claims = verify_token(token, &secret).map_err(|_| AppError::Unauthorized)?;

    req.extensions_mut().insert(claims.sub);

    Ok(next.run(req).await)
}

use axum::{extract::FromRequestParts, http::request::Parts};

impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
    PgPool: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let user_id = parts
            .extensions
            .get::<String>()
            .ok_or(AppError::Unauthorized)?;

        let pool = PgPool::from_ref(state);
        let user = find_user_by_id(&pool, user_id).await?;

        let UserRow {
            id,
            username,
            email,
            status,
            deactivated_at,
            created_at,
            updated_at,
            ..
        } = user;

        user.deactivated_at
            .map(|_| Err(AppError::DeactivatedAccount))
            .unwrap_or(Ok(()))?;

        Ok(CurrentUser {
            id,
            username,
            email,
            status,
            deactivated_at,
            created_at,
            updated_at,
        })
    }
}

pub async fn required_permission(
    required_code: String,
    user: CurrentUser,
    req: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    // let codes = find_user_permission_codes_by_id(&pool, &user.id).await?;
    println!("{required_code}");
    // if !codes.contains(&required_code) {
    //     return Err(AppError::Unauthorized);
    // }
    Ok(next.run(req).await)
}
