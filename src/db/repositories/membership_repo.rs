use anyhow::Result;
use sqlx::{PgPool, Transaction, Postgres};
use uuid::Uuid;

use crate::{db::models::membership::Membership, errors::AppError};

pub async fn create(
    pool: &PgPool,
    user_id: Uuid,
    organization_id: Uuid,
) -> Result<Membership> {
    let membership =
        sqlx::query_as::<_, Membership>(
            r#"
            INSERT INTO memberships (
                user_id,
                organization_id
            )
            VALUES ($1,$2)
            RETURNING *
            "#
        )
        .bind(user_id)
        .bind(organization_id)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            println!("{e:?}");
            AppError::Internal
        })?;

    Ok(membership)
}

pub async fn create_tx(
    tx: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    organization_id: Uuid,
) -> Result<Membership> {
    let membership =
        sqlx::query_as::<_, Membership>(
            r#"
            INSERT INTO memberships (
                user_id,
                organization_id
            )
            VALUES ($1,$2)
            RETURNING *
            "#
        )
        .bind(user_id)
        .bind(organization_id)
        .fetch_one(tx.as_mut())
        .await?;

    Ok(membership)
}

pub async fn assign_role_tx(
    tx: &mut Transaction<'_, Postgres>,
    membership_id: Uuid,
    role_id: Uuid,
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO member_roles (
            membership_id,
            role_id
        )
        VALUES ($1,$2)
        "#
    )
    .bind(membership_id)
    .bind(role_id)
    .execute(tx.as_mut())
    .await?;

    Ok(())
}

pub async fn find(
    pool: &PgPool,
    user_id: Uuid,
    organization_id: Uuid,
) -> Result<Option<Membership>> {
    let membership =
        sqlx::query_as::<_, Membership>(
            r#"
            SELECT *
            FROM memberships
            WHERE
                user_id = $1
                AND
                organization_id = $2
            "#
        )
        .bind(user_id)
        .bind(organization_id)
        .fetch_optional(pool)
        .await?;

    Ok(membership)
}

pub async fn list_by_organization(
    pool: &PgPool,
    organization_id: Uuid,
) -> Result<Vec<Membership>> {
    let memberships =
        sqlx::query_as::<_, Membership>(
            r#"
            SELECT *
            FROM memberships
            WHERE organization_id = $1
            ORDER BY created_at
            "#
        )
        .bind(organization_id)
        .fetch_all(pool)
        .await?;

    Ok(memberships)
}