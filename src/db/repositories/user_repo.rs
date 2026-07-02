use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::models::user::User;

pub async fn find_by_id(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<User>> {
    let user =
        sqlx::query_as::<_, User>(
            r#"
            SELECT *
            FROM users
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

pub async fn find_by_email(
    pool: &PgPool,
    email: &str,
) -> Result<Option<User>> {
    let user =
        sqlx::query_as::<_, User>(
            r#"
            SELECT *
            FROM users
            WHERE email = $1
            "#
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

pub async fn create(
    pool: &PgPool,
    email: &str,
    password_hash: &str,
    name: &str,
) -> Result<User> {
    let user =
        sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (
                email,
                password_hash,
                name
            )
            VALUES ($1,$2,$3)
            RETURNING *
            "#
        )
        .bind(email)
        .bind(password_hash)
        .bind(name)
        .fetch_one(pool)
        .await?;

    Ok(user)
}

pub async fn update_name(
    pool: &PgPool,
    id: Uuid,
    name: &str,
) -> Result<User> {
    let user =
        sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET name = $2
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(id)
        .bind(name)
        .fetch_one(pool)
        .await?;

    Ok(user)
}