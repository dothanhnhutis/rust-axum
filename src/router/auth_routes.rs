use axum::{Router, routing::post};

use crate::{AppState, handlers::auth_handler::login_handler};

pub fn create_routes() -> Router<AppState> {
    Router::new().route("/login", post(login_handler))
}

// pub fn create_router<S>() -> Router<S>
// where
//     S: Clone + Send + Sync + 'static,
// {
//     Router::new().route("/login", post(login_handler))
// }
