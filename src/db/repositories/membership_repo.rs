use anyhow::Result;
use sqlx::{PgPool, Transaction, Postgres};
use uuid::Uuid;

use crate::db::models::membership::Membership;

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
        .await?;

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