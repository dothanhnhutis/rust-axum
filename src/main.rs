use axum::{Router, middleware};
use dotenvy::dotenv;
use server::{
    AppState, handlers::handler_404, init_db_pool, middleware::auth_middleware,
    router::create_router,
};

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

    let share_state = AppState {
        db: pool,
        jwt_secret: "secret".to_string(),
    };

    // 3. Build route
    let app = Router::new()
        .nest("/api", create_router("secret".to_string()))
        .fallback(handler_404)
        .with_state(share_state);

    // 4. Chạy server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::info!("Server đang chạy tại: http://localhost:8080");
    axum::serve(listener, app).await.unwrap();
}

// use serde::Deserialize;
// use validator::Validate;

// #[derive(Debug, Deserialize, Validate)]
// pub struct LoginRequest {
//     #[validate(email)]
//     pub email: String,
//     #[validate(length(min = 8, message = "Password phải ít nhất 6 ký tự"))]
//     pub password: String,
// }

// use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
// use serde_json::json;
// use validator::ValidationErrors;

// #[derive(Debug)]
// pub enum AppError {
//     Validation(ValidationErrors),
//     BadRequest(String),
//     NotFound(String),
//     Unauthorized(String),
//     Internal(String),
// }

// impl IntoResponse for AppError {
//     fn into_response(self) -> axum::response::Response {
//         match self {
//             AppError::Validation(e) => (
//                 StatusCode::BAD_REQUEST,
//                 Json(json!({ "error": e.to_string() })),
//             )
//                 .into_response(),

//             AppError::BadRequest(msg) => {
//                 (StatusCode::BAD_REQUEST, Json(json!({ "error": msg }))).into_response()
//             }

//             AppError::NotFound(msg) => {
//                 (StatusCode::NOT_FOUND, Json(json!({ "error": msg }))).into_response()
//             }

//             AppError::Unauthorized(msg) => {
//                 (StatusCode::UNAUTHORIZED, Json(json!({ "error": msg }))).into_response()
//             }

//             AppError::Internal(msg) => (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 Json(json!({ "error": msg })),
//             )
//                 .into_response(),
//         }
//     }
// }

// impl From<ValidationErrors> for AppError {
//     fn from(err: ValidationErrors) -> Self {
//         AppError::Validation(err)
//     }
// }

// impl From<sqlx::Error> for AppError {
//     fn from(_: sqlx::Error) -> Self {
//         AppError::Internal("Database error".into())
//     }
// }
// use axum::extract::rejection::JsonRejection;

// impl From<JsonRejection> for AppError {
//     fn from(err: JsonRejection) -> Self {
//         AppError::BadRequest(err.to_string())
//     }
// }

// async fn login(
//     State(pool): State<PgPool>,
//     payload: Result<Json<LoginRequest>, JsonRejection>,
// ) -> Result<impl IntoResponse, AppError> {
//     let payload = match payload {
//         Ok(Json(data)) => data,
//         Err(err) => {
//             return Err(AppError::BadRequest(err.to_string()));
//         }
//     };

//     // validate
//     payload.validate()?;

//     let user = find_by_email(&pool, &payload.email).await?;

//     let user = user.ok_or(AppError::NotFound("User không tồn tại".into()))?;

//     verify_password(&payload.password, &user.password_hash)
//         .map_err(|_| AppError::Unauthorized("Sai mật khẩu".into()))?;

//     Ok((
//         StatusCode::OK,
//         Json(json!({
//             "message": "Đăng nhập thành công",
//             "user_id": user.id,
//         })),
//     ))
// }

// async fn login(
//     State(pool): State<PgPool>,
//     payload: Result<Json<LoginRequest>, JsonRejection>,
// ) -> impl IntoResponse {
//     // 1. Handle JSON lỗi (missing field, sai format...)
//     let Json(payload) = match payload {
//         Ok(data) => data,
//         Err(err) => {
//             return (
//                 StatusCode::BAD_REQUEST,
//                 Json(json!({
//                     "error": "Invalid request",
//                     "detail": err.to_string()
//                 })),
//             );
//         }
//     };

//     // 1. Validate dữ liệu đầu vào
//     if let Err(e) = payload.validate() {
//         println!("{e:#?}");
//         return (
//             StatusCode::BAD_REQUEST,
//             Json(json!({ "error": e.to_string() })),
//         );
//     }

//     // 2. Tìm user trong DB
//     let user_result = find_by_email(&pool, &payload.email).await;
//     match user_result {
//         Ok(Some(user)) => {
//             if verify_password(&payload.password, &user.password_hash).is_ok() {
//                 (
//                     StatusCode::OK,
//                     Json(json!({
//                         "message": "Đăng nhập thành công",
//                         "user_id": user.id,
//                     })),
//                 )
//             } else {
//                 (
//                     StatusCode::UNAUTHORIZED,
//                     Json(json!({ "error": "Sai mật khẩu" })),
//                 )
//             }
//         }
//         Ok(None) => (
//             StatusCode::NOT_FOUND,
//             Json(json!({ "error": "User không tồn tại" })),
//         ),
//         Err(_) => (
//             StatusCode::INTERNAL_SERVER_ERROR,
//             Json(json!({ "error": "Lỗi hệ thống" })),
//         ),
//     }
// }
