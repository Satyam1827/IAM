use anyhow::Result;
use sqlx::{
    PgPool,
    Postgres,
    Transaction,
};
use uuid::Uuid;

use crate::db::models::role::Role;

pub async fn create_tx(
    tx: &mut Transaction<'_, Postgres>,
    organization_id: Uuid,
    name: &str,
    description: Option<&str>,
) -> Result<Role> {
    let role =
        sqlx::query_as::<_, Role>(
            r#"
            INSERT INTO roles (
                organization_id,
                name,
                description
            )
            VALUES ($1,$2,$3)
            RETURNING *
            "#
        )
        .bind(organization_id)
        .bind(name)
        .bind(description)
        .fetch_one(tx.as_mut())
        .await?;

    Ok(role)
}

pub async fn assign_permission_tx(
    tx: &mut Transaction<'_, Postgres>,
    role_id: Uuid,
    permission_id: Uuid,
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO role_permissions (
            role_id,
            permission_id
        )
        VALUES ($1,$2)
        "#
    )
    .bind(role_id)
    .bind(permission_id)
    .execute(tx.as_mut())
    .await?;

    Ok(())
}


// This function answers
// Does this user have this permission
// inside this organization?
pub async fn has_permission(
    pool: &PgPool,
    user_id: Uuid,
    organization_id: Uuid,
    permission: &str,
) -> Result<bool> {
    let exists:
        (bool,) =
        sqlx::query_as(
            r#"
            SELECT EXISTS(
                SELECT 1
                FROM memberships m
                JOIN member_roles mr
                    ON mr.membership_id = m.id
                JOIN role_permissions rp
                    ON rp.role_id = mr.role_id
                JOIN permissions p
                    ON p.id = rp.permission_id
                WHERE
                    m.user_id = $1
                    AND
                    m.organization_id = $2
                    AND
                    p.name = $3
            )
            "#
        )
        .bind(user_id)
        .bind(organization_id)
        .bind(permission)
        .fetch_one(pool)
        .await?;

    Ok(exists.0)
}


// Inserts custom roles into roles table
pub async fn create(
    pool: &PgPool,
    organization_id: Uuid,
    name: &str,
    description: Option<&str>,
) -> Result<Role> {
    let role =
        sqlx::query_as::<_, Role>(
            r#"
            INSERT INTO roles (
                organization_id,
                name,
                description
            )
            VALUES ($1,$2,$3)
            RETURNING *
            "#
        )
        .bind(organization_id)
        .bind(name)
        .bind(description)
        .fetch_one(pool)
        .await?;

    Ok(role)
}

pub async fn find_by_id(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<Role>> {
    let role =
        sqlx::query_as::<_, Role>(
            r#"
            SELECT *
            FROM roles
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

    Ok(role)
}