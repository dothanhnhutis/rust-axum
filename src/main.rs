use axum::{
    Router,
    extract::rejection::JsonRejection,
    routing::{get, post},
};
use dotenvy::dotenv;
use server::{find_by_email, init_db_pool, utils::password::verify_password};
use sqlx::PgPool;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // 1. Khởi tạo Log
    tracing_subscriber::fmt::init();

    // 2. Kết nối Database từ Lib Infrastructure
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = init_db_pool(&db_url)
        .await
        .expect("Failed to connect to DB");

    // 3. Build route
    let app = Router::new()
        .route(
            "/",
            get(|| async { "Hệ thống quản lý kho hóa chất sẵn sàng!" }),
        )
        .route("/login", post(login))
        .with_state(pool); // Chia sẻ pool cho các handler

    // 4. Chạy server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::info!("Server đang chạy tại: http://localhost:8080");
    axum::serve(listener, app).await.unwrap();
}

use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, message = "Password phải ít nhất 6 ký tự"))]
    pub password: String,
}

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;

async fn login(
    State(pool): State<PgPool>,
    payload: Result<Json<LoginRequest>, JsonRejection>,
) -> impl IntoResponse {
    // 1. Handle JSON lỗi (missing field, sai format...)
    let Json(payload) = match payload {
        Ok(data) => data,
        Err(err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "Invalid request",
                    "detail": err.to_string()
                })),
            );
        }
    };

    // 1. Validate dữ liệu đầu vào
    if let Err(e) = payload.validate() {
        println!("{e:#?}");
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e.to_string() })),
        );
    }

    // 2. Tìm user trong DB
    let user_result = find_by_email(&pool, &payload.email).await;
    match user_result {
        Ok(Some(user)) => {
            if verify_password(&payload.password, &user.password_hash).is_ok() {
                (
                    StatusCode::OK,
                    Json(json!({
                        "message": "Đăng nhập thành công",
                        "user_id": user.id,
                    })),
                )
            } else {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({ "error": "Sai mật khẩu" })),
                )
            }
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "User không tồn tại" })),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Lỗi hệ thống" })),
        ),
    }
}
