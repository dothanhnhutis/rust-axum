use axum::{Router, extract::FromRef, middleware, routing::get};
use sqlx::PgPool;

use crate::{
    handlers::user_handler::current_user, middleware::required_permission, state::AppState,
};

pub fn create_router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new().route("/me", get(current_user))
}
