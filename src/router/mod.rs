use axum::{Router, extract::FromRef, middleware};
use sqlx::PgPool;

use crate::{middleware::auth_middleware, state::AppState};
mod auth_routes;
mod role_routes;
mod user_routes;
// pub fn create_router() -> Router<AppState> {
//     Router::new().nest("/auth", auth_routes::create_routes())
// }

// case 2 advance
pub fn create_router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    PgPool: FromRef<S>,
    String: FromRef<S>,
{
    let protected = Router::new()
        .nest("/users", user_routes::create_router())
        .nest("/roles", role_routes::create_router())
        .layer(middleware::from_fn(auth_middleware));

    Router::new()
        .nest("/auth", auth_routes::create_router())
        .merge(protected)
}
