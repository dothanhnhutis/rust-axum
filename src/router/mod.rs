use axum::{Router, extract::FromRef};
use sqlx::PgPool;

use crate::AppState;
mod auth_routes;

// pub fn create_router() -> Router<AppState> {
//     Router::new().nest("/auth", auth_routes::create_routes())
// }

// case 2 advance
pub fn create_router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    PgPool: FromRef<S>,
{
    Router::new().nest("/auth", auth_routes::create_router())
}

// khi nào dùng Arc::new(AppState { /* ... */ });
