use sqlx::PgPool;

use crate::{
    db::models::{
        role_model::{PermissionRow, RolePermissionRow},
        user_model::UserRow,
    },
    error_handler::AppError,
};

pub async fn find_user_by_email(pool: &PgPool, email: &str) -> Result<UserRow, AppError> {
    let user = sqlx::query_as!(
        UserRow,
        r#"
        SELECT *
        FROM users
        WHERE email = $1
        "#,
        email
    )
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::InvalidCredentials)?;

    Ok(user)
}

pub async fn find_user_by_id(pool: &PgPool, id: &str) -> Result<UserRow, AppError> {
    let user = sqlx::query_as!(
        UserRow,
        r#"
        SELECT *
        FROM users
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::InvalidCredentials)?;

    Ok(user)
}

pub async fn find_user_roles_by_id(
    pool: &PgPool,
    id: &str,
) -> Result<Vec<RolePermissionRow>, AppError> {
    let roles = sqlx::query_as::<_, RolePermissionRow>(
        r#"
        SELECT r.*, ur.created_at as join_at,
               COALESCE(p_agg.permissions, '[]') AS permissions
        FROM roles r
        LEFT JOIN user_roles ur 
            ON r.id = ur.role_id 
           AND r.status <> 'DEACTIVATED' 
           AND r.deleted_at IS NULL
        LEFT JOIN LATERAL (
            SELECT jsonb_agg(p.*) AS permissions
            FROM permissions p
            LEFT JOIN role_permissions rp ON rp.permission_id = p.id
            WHERE rp.role_id = r.id
        ) p_agg ON TRUE
        WHERE ur.user_id = $1::TEXT;
        "#,
    )
    .bind(id)
    .fetch_all(pool)
    .await?;

    Ok(roles)
}

pub async fn find_user_permissions_by_id(
    pool: &PgPool,
    id: &str,
) -> Result<Vec<PermissionRow>, AppError> {
    let permissions = sqlx::query_as::<_, PermissionRow>(
        r#"
        SELECT DISTINCT p.*
        FROM permissions p
                LEFT JOIN role_permissions rp ON p.id = rp.permission_id
                LEFT JOIN user_roles ur ON rp.role_id = ur.role_id
        WHERE ur.user_id = $1::TEXT;
        "#,
    )
    .bind(id)
    .fetch_all(pool)
    .await?;

    Ok(permissions)
}

pub async fn find_user_permission_codes_by_id(
    pool: &PgPool,
    id: &str,
) -> Result<Vec<String>, AppError> {
    let permissions = sqlx::query_scalar::<_, String>(
        r#"
       SELECT DISTINCT p.code
        FROM permissions p
                LEFT JOIN role_permissions rp ON p.id = rp.permission_id
                LEFT JOIN user_roles ur ON rp.role_id = ur.role_id
        WHERE ur.user_id = $1::TEXT;
        "#,
    )
    .bind(id)
    .fetch_all(pool)
    .await?;

    Ok(permissions)
}
