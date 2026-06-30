mod config;
mod db;
mod state;
mod auth;
mod errors;
mod services;
mod dto;
mod handlers;
mod routes;
mod app;
mod middleware;

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

    let state = Arc::new(AppState {
        db: pool,
        config: config.clone(),
    });

    let app =
        app::create_router(state);

    let listener =
        tokio::net::TcpListener::bind(
            format!(
                "{}:{}",
                config.host,
                config.port
            )
        )
        .await?;

    println!(
        "Listening on {}:{}",
        config.host,
        config.port
    );

    axum::serve(listener, app)
        .await?;

    Ok(())
}