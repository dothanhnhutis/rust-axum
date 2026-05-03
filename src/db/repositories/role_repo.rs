use sqlx::{PgPool, Postgres, Transaction};

use crate::{
    db::models::role_model::RoleRow, error_handler::AppError,
    handlers::role_handler::CreateRoleRequest,
};

pub async fn find_roles(pool: &PgPool) -> Result<Vec<RoleRow>, AppError> {
    let roles: Vec<RoleRow> = sqlx::query_as!(
        RoleRow,
        r#"
        SELECT *
        FROM roles
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(roles)
}

pub async fn create_role(pool: &PgPool, input: &CreateRoleRequest) -> Result<RoleRow, AppError> {
    let mut tx: Transaction<'_, Postgres> = pool.begin().await?;

    let new_role = sqlx::query_as!(
        RoleRow,
        r#"

            WITH new_role AS (
                INSERT INTO roles (name, description)
                    VALUES ($1::TEXT, $2::TEXT)
                    RETURNING *),
            insert_permissions AS (
                INSERT
                    INTO role_permissions (role_id, permission_id)
                        SELECT new_role.id, p.permission_id
                        FROM new_role,
                            unnest($3::TEXT[]) AS p(permission_id)
                        ON CONFLICT DO NOTHING)

            SELECT *
            FROM new_role;
       
        "#,
        &input.name,
        input.description.as_deref(),
        &input.permission_ids
    )
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(new_role)
}
