use axum::Router;
use dotenvy::dotenv;
use server::{handlers::handler_404, init_db_pool, router::create_router, state::AppState};
use std::env;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // 1. Khởi tạo Log
    tracing_subscriber::fmt::init();

    // 2. Kết nối Database từ Lib Infrastructure
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let pool = init_db_pool(&db_url)
        .await
        .expect("Failed to connect to DB");

    let share_state = AppState {
        db: pool,
        jwt_secret,
    };

    // 3. Build route
    let app = Router::new()
        .nest("/api", create_router())
        .fallback(handler_404)
        .with_state(share_state)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    // 4. Chạy server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::info!("Server đang chạy tại: http://localhost:8080");
    axum::serve(listener, app).await.unwrap();
}
