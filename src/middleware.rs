use axum::{extract::State, http::Request, middleware::Next, response::Response};

use crate::{error_handler::AppError, utils::jwt::verify_token};

pub async fn auth_middleware<B>(
    State(secret): State<String>,
    mut req: Request<B>,
    next: Next<B>,
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

    // attach user_id vào request extensions
    req.extensions_mut().insert(claims.sub);

    Ok(next.run(req).await)
}
