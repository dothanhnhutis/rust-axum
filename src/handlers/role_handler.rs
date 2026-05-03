use axum::{extract::State, response::IntoResponse};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use validator::Validate;

use crate::{
    db::repositories::role_repo::find_roles, error_handler::AppError, utils::ok,
    validators::ValidatedJson,
};

pub async fn get_roles_handler(State(pool): State<PgPool>) -> Result<impl IntoResponse, AppError> {
    let roles = find_roles(&pool).await?;

    Ok(ok(roles))
}

#[derive(Deserialize, Debug, Validate)]
#[serde(deny_unknown_fields)]
pub struct CreateRoleRequest {
    #[validate(length(min = 1, max = 255, message = "Tên vai trò không được bỏ trống."))]
    pub name: String,

    #[validate(length(max = 500))]
    pub description: Option<String>,

    pub permission_ids: Vec<String>,
}

pub async fn create_role_handler(
    State(pool): State<PgPool>,
    ValidatedJson(payload): ValidatedJson<CreateRoleRequest>,
) -> Result<impl IntoResponse, AppError> {
    println!("{:#?}", payload);

    Ok(ok(json!({
       "message": "oker"
    })))
}

pub async fn update_role_handler(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, AppError> {
    Ok(ok(json!({
       "message": "oker"
    })))
}

pub async fn delete_role_handler(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, AppError> {
    Ok(ok(json!({
       "message": "oker"
    })))
}
