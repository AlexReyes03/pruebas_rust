mod config;
mod error;
mod routes;
mod state;
mod modules;

use anyhow::{Context, Result};
use sqlx::sqlite::SqlitePoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Config;
use crate::state::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing/logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "wallet_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Wallet Backend Server...");

    // Load configuration
    let config = Config::load().context("Failed to load configuration")?;
    config.validate().context("Configuration validation failed")?;
    
    tracing::info!("Configuration loaded successfully");
    tracing::debug!("Server config: {}:{}", config.server.host, config.server.port);
    tracing::debug!("Stellar network: {}", config.stellar.network);

    // Initialize database
    let db_pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&config.database.url)
        .await
        .context("Failed to connect to database")?;

    tracing::info!("Database connection established");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .context("Failed to run database migrations")?;

    tracing::info!("Database migrations completed");

    // Initialize application state
    let state = AppState::new(config.clone(), db_pool).await;
    
    tracing::info!("Application state initialized");

    // Create router
    let app = routes::create_router(state);

    // Start server
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .context("Failed to bind to address")?;

    tracing::info!("Server listening on {}", addr);
    tracing::info!("API available at http://{}/api", addr);
    tracing::info!("Health check at http://{}/api/health", addr);

    axum::serve(listener, app)
        .await
        .context("Server error")?;

    Ok(())
}