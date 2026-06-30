use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::models::session::Session;

pub async fn create(
    pool: &PgPool,
    user_id: Uuid,
    refresh_token_hash: &str,
    ip_address: Option<&str>,
    user_agent: Option<&str>,
    expires_at: DateTime<Utc>,
) -> Result<Session> {
    let session =
        sqlx::query_as::<_, Session>(
            r#"
            INSERT INTO sessions (
                user_id,
                refresh_token_hash,
                ip_address,
                user_agent,
                expires_at
            )
            VALUES ($1,$2,$3,$4,$5)
            RETURNING *
            "#
        )
        .bind(user_id)
        .bind(refresh_token_hash)
        .bind(ip_address)
        .bind(user_agent)
        .bind(expires_at)
        .fetch_one(pool)
        .await?;

    Ok(session)
}

pub async fn find_by_id(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<Session>> {
    let session =
        sqlx::query_as::<_, Session>(
            r#"
            SELECT *
            FROM sessions
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

    Ok(session)
}

pub async fn delete(
    pool: &PgPool,
    id: Uuid,
) -> Result<()> {
    sqlx::query(
        r#"
        DELETE
        FROM sessions
        WHERE id = $1
        "#
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}