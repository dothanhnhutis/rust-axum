use axum::{
    Router,
    extract::FromRef,
    routing::{get, patch},
};
use sqlx::PgPool;

use crate::handlers::role_handler::{create_role_handler, get_roles_handler, update_role_handler};

pub fn create_router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    PgPool: FromRef<S>,
{
    Router::new()
        .route("/", get(get_roles_handler).post(create_role_handler))
        .route("/{id}", patch(update_role_handler))
}
