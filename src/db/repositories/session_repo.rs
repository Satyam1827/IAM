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

pub async fn find_by_user(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<Session>> {
    let sessions =
        sqlx::query_as::<_, Session>(
            r#"
            SELECT *
            FROM sessions
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

    Ok(sessions)
}

pub async fn update_refresh_token(
    pool: &PgPool,
    session_id: Uuid,
    refresh_hash: &str,
    expires_at: DateTime<Utc>,
) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE sessions
        SET
            refresh_token_hash = $1,
            expires_at = $2
        WHERE id = $3
        "#
    )
    .bind(refresh_hash)
    .bind(expires_at)
    .bind(session_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn list_by_user(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<Session>> {
    let sessions =
        sqlx::query_as::<_, Session>(
            r#"
            SELECT *
            FROM sessions
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

    Ok(sessions)
}