use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::models::organization::Organization;

pub async fn create(
    pool: &PgPool,
    name: &str,
    slug: &str,
    created_by: Uuid,
) -> Result<Organization> {
    let organization =
        sqlx::query_as::<_, Organization>(
            r#"
            INSERT INTO organizations (
                name,
                slug,
                created_by
            )
            VALUES ($1,$2,$3)
            RETURNING *
            "#
        )
        .bind(name)
        .bind(slug)
        .bind(created_by)
        .fetch_one(pool)
        .await?;

    Ok(organization)
}

pub async fn find_by_id(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<Organization>> {
    let organization =
        sqlx::query_as::<_, Organization>(
            r#"
            SELECT *
            FROM organizations
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

    Ok(organization)
}

pub async fn list(
    pool: &PgPool,
) -> Result<Vec<Organization>> {
    let organizations =
        sqlx::query_as::<_, Organization>(
            r#"
            SELECT *
            FROM organizations
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(pool)
        .await?;

    Ok(organizations)
}