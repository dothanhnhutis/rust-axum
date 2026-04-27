use axum::Router;

use crate::AppState;
mod auth_routes;

// pub fn create_router<S>() -> Router<S>
// where
//     S: Clone + Send + Sync + 'static,
// {
//     Router::new().nest("/auth", auth_routes::create_router())
// }

pub fn create_router() -> Router<AppState> {
    Router::new().nest("/auth", auth_routes::create_routes())
}
// khi nào dùng Arc::new(AppState { /* ... */ });
