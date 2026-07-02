use anyhow::Result;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::models::api_key::ApiKey;

pub async fn create(
    pool: &PgPool,
    organization_id: Uuid,
    name: &str,
    key_hash: &str,
    created_by: Uuid,
) -> Result<ApiKey> {
    let api_key =
        sqlx::query_as::<_, ApiKey>(
            r#"
            INSERT INTO api_keys (
                organization_id,
                name,
                key_hash,
                created_by,
                created_at
            )
            VALUES ($1,$2,$3,$4,$5)
            RETURNING *
            "#
        )
        .bind(organization_id)
        .bind(name)
        .bind(key_hash)
        .bind(created_by)
        .bind(Utc::now())
        .fetch_one(pool)
        .await?;

    Ok(api_key)
}

pub async fn find_by_id(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<ApiKey>> {
    let key =
        sqlx::query_as::<_, ApiKey>(
            r#"
            SELECT *
            FROM api_keys
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

    Ok(key)
}
// Find API Key By Hash Verification
pub async fn all(
    pool: &PgPool,
) -> Result<Vec<ApiKey>> {
    let keys =
        sqlx::query_as::<_, ApiKey>(
            r#"
            SELECT *
            FROM api_keys
            "#
        )
        .fetch_all(pool)
        .await?;

    Ok(keys)
}

pub async fn delete(
    pool: &PgPool,
    id: Uuid,
) -> Result<()> {
    sqlx::query(
        r#"
        DELETE FROM api_keys
        WHERE id = $1
        "#
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn list_by_organization(
    pool: &PgPool,
    organization_id: Uuid,
) -> Result<Vec<ApiKey>> {
    let keys =
        sqlx::query_as::<_, ApiKey>(
            r#"
            SELECT *
            FROM api_keys
            WHERE organization_id = $1
            ORDER BY created_at DESC
            "#
        )
        .bind(organization_id)
        .fetch_all(pool)
        .await?;

    Ok(keys)
}