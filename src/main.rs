mod auth;
mod config;
mod db;
mod error;
mod handlers;
mod models;
mod routes;

use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,home_inventory=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = config::Config::from_env()?;

    tracing::info!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    tracing::info!("Running database migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;

    let addr = config.addr();

    let auth_config = auth::AuthConfig {
        username: config.auth_username,
        password: config.auth_password,
    };
    if auth_config.is_enabled() {
        tracing::info!("Basic auth is enabled");
    } else {
        tracing::warn!("Basic auth is DISABLED — set AUTH_USERNAME and AUTH_PASSWORD to enable");
    }

    let app = routes::create_routes(pool, auth_config);
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
