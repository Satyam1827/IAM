use anyhow::Result;
use sqlx::{
    PgPool,
    Postgres,
    Transaction,
};
use uuid::Uuid;

use crate::db::models::permission::Permission;

pub async fn create_tx(
    tx: &mut Transaction<'_, Postgres>,
    name: &str,
    description: Option<&str>,
) -> Result<Permission> {
    let permission =
        sqlx::query_as::<_, Permission>(
            r#"
            INSERT INTO permissions (
                name,
                description
            )
            VALUES ($1,$2)
            RETURNING *
            "#
        )
        .bind(name)
        .bind(description)
        .fetch_one(tx.as_mut())
        .await?;

    Ok(permission)
}

pub async fn find_by_name(
    pool: &PgPool,
    name: &str,
) -> Result<Option<Permission>> {
    let permission =
        sqlx::query_as::<_, Permission>(
            r#"
            SELECT *
            FROM permissions
            WHERE name = $1
            "#
        )
        .bind(name)
        .fetch_optional(pool)
        .await?;

    Ok(permission)
}

pub async fn find_by_id(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<Permission>> {
    let permission =
        sqlx::query_as::<_, Permission>(
            r#"
            SELECT *
            FROM permissions
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

    Ok(permission)
}