mod config;
mod db;
mod state;

use anyhow::Result;
use std::sync::Arc;

use state::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = config::Settings::new()?;

    let pool =
        db::pool::create_pool(
            &config.database_url,
        )
        .await?;

    db::migrate::run(&pool).await?;

    let _state = Arc::new(AppState {
        db: pool,
        config,
    });

    println!("Database ready.");

    Ok(())
}