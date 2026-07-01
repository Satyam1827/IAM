use anyhow::Result;
use sqlx::PgPool;
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