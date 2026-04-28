use axum::{Router, extract::FromRef, routing::post};
use sqlx::PgPool;

use crate::handlers::auth_handler::{login_handler, logout, refresh};

// pub fn create_routes() -> Router<AppState> {
//     Router::new().route("/login", post(login_handler))
// }

// case 2 advance
pub fn create_router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    PgPool: FromRef<S>,
{
    Router::new()
        .route("/login", post(login_handler))
        .route("/refresh", post(refresh))
        .route("/logout", post(logout))
}
