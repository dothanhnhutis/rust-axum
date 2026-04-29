use axum::extract::FromRef;
use sqlx::PgPool;

#[derive(Clone, FromRef, Debug)]
pub struct AppState {
    pub db: PgPool,
    pub jwt_secret: String,
}
