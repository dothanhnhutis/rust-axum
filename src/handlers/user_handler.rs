use axum::{extract::State, response::IntoResponse};
use serde_json::json;
use sqlx::PgPool;

use crate::{
    db::{
        models::user_model::CurrentUser,
        repositories::user_repo::{
            find_user_permission_codes_by_id, find_user_permissions_by_id, find_user_roles_by_id,
        },
    },
    error_handler::AppError,
    utils::ok,
};

pub async fn current_user(
    State(pool): State<PgPool>,
    user: CurrentUser,
) -> Result<impl IntoResponse, AppError> {
    let a = find_user_roles_by_id(&pool, &user.id).await?;
    let b = find_user_permissions_by_id(&pool, &user.id).await?;
    let c = find_user_permission_codes_by_id(&pool, &user.id).await?;

    println!("{c:#?}");
    Ok(ok(json!(user)))
}
