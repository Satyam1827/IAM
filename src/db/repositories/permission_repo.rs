use anyhow::Result;
use sqlx::{
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