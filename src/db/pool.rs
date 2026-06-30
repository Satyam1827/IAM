use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn create_pool(
    database_url: &str,
) -> Result<PgPool> {
    Ok(
        PgPoolOptions::new()
            .max_connections(20)
            .connect(database_url)
            .await?,
    )
}