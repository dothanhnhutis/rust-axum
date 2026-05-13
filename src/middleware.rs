use axum::{
    body::Body,
    extract::{FromRef, State},
    http::Request,
    middleware::Next,
    response::Response,
};
use sqlx::PgPool;

use crate::{
    db::{
        models::user_model::{CurrentUser, UserRow},
        repositories::user_repo::find_user_by_id,
    },
    error_handler::AppError,
    state::Config,
    utils::jwt::verify_token,
};

#[derive(Clone, Debug)]
pub struct AuthContext {
    pub token: String,
}

pub async fn auth_middleware(mut req: Request<Body>, next: Next) -> Result<Response, AppError> {
    // let auth_header = req
    //     .headers()
    //     .get("Authorization")
    //     .and_then(|h| h.to_str().ok())
    //     .ok_or(AppError::Unauthorized)?;

    // let token = auth_header
    //     .strip_prefix("Bearer ")
    //     .ok_or(AppError::Unauthorized)?;

    let token = {
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or(AppError::Unauthorized)?;

        let token_str = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AppError::Unauthorized)?;

        token_str.to_string() // Chuyển thành String để giải phóng tham chiếu đến req
    };

    req.extensions_mut().insert(AuthContext { token });

    Ok(next.run(req).await)
}

use axum::{extract::FromRequestParts, http::request::Parts};

impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
    PgPool: FromRef<S>,
    Config: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let config = Config::from_ref(state);

        let auth_context = parts
            .extensions
            .get::<AuthContext>()
            .ok_or(AppError::Unauthorized)?;

        let claims = verify_token(&auth_context.token, &config.jwt_secret)
            .map_err(|_| AppError::Unauthorized)?;

        let pool = PgPool::from_ref(state);

        let user = find_user_by_id(&pool, &claims.sub).await?;

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
