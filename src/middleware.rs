use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use serde::{Deserialize, Serialize};

use crate::{error_handler::AppError, state::AppState, utils::jwt::verify_token};

pub async fn auth_middleware(
    State(state): State<AppState>,
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

    let claims = verify_token(token, &state.jwt_secret).map_err(|_| AppError::Unauthorized)?;

    req.extensions_mut().insert(claims.sub);

    Ok(next.run(req).await)
}

use axum::{extract::FromRequestParts, http::request::Parts};

#[derive(Clone, Serialize)]
pub struct CurrentUser {
    pub user_id: String,
}

impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let user_id = parts
            .extensions
            .get::<String>()
            .ok_or(AppError::Unauthorized)?;

        Ok(CurrentUser {
            user_id: user_id.clone(),
        })
    }
}
