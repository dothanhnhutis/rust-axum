use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};

use crate::{error_handler::AppError, utils::jwt::verify_token};

pub async fn auth_middleware(
    State(secret): State<String>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    println!("{req:#?}");
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized)?;

    let claims = verify_token(token, &secret).map_err(|_| AppError::Unauthorized)?;

    // attach user_id vào request extensions
    req.extensions_mut().insert(claims.sub);

    Ok(next.run(req).await)
}

use axum::{extract::FromRequestParts, http::request::Parts};

#[derive(Clone)]
pub struct CurrentUser {
    pub user_id: String,
}

impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let user = parts
            .extensions
            .get::<CurrentUser>()
            .ok_or(AppError::Unauthorized)?;

        Ok(CurrentUser {
            user_id: user.user_id.clone(),
        })
    }
}
