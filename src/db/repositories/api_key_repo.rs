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
            "#
        )
        .bind(organization_id)
        .fetch_all(pool)
        .await?;

    Ok(keys)
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