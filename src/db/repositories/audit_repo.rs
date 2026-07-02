use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::models::audit_log::AuditLog;

pub async fn create(
    pool: &PgPool,
    organization_id: Option<Uuid>,
    actor_id: Option<Uuid>,
    action: &str,
    resource_type: Option<&str>,
    resource_id: Option<Uuid>,
) -> Result<AuditLog> {
    let log =
        sqlx::query_as::<_, AuditLog>(
            r#"
            INSERT INTO audit_logs (
                organization_id,
                actor_id,
                action,
                resource_type,
                resource_id
            )
            VALUES ($1,$2,$3,$4,$5)
            RETURNING *
            "#
        )
        .bind(organization_id)
        .bind(actor_id)
        .bind(action)
        .bind(resource_type)
        .bind(resource_id)
        .fetch_one(pool)
        .await?;

    Ok(log)
}