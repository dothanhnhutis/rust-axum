use axum::{Router, extract::FromRef, routing::get};
use sqlx::PgPool;

use crate::handlers::user_handler::current_user;

pub fn create_router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    PgPool: FromRef<S>,
{
    Router::new().route("/me", get(current_user))
}
