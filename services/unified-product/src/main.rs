mod config;
mod server;
mod api;
mod domain;
mod middleware;

use std::sync::Arc;

use actix_web::{web, App, HttpServer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Config;
use crate::server::Server;
use shared::{DatabaseConnection, RedisConnection};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file if it exists
    dotenv::dotenv().ok();

    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");

    // Initialize database connections
    let db_conn = DatabaseConnection::new(&config.database_url)
        .expect("Failed to connect to database");

    let redis_conn = RedisConnection::new(&config.redis_url)
        .expect("Failed to connect to Redis");

    // Create shared application state
    let app_state = Arc::new(server::AppState {
        config: config.clone(),
        db_conn,
        redis_conn,
    });

    // Start server
    let server = Server::new(app_state);
    server.run().await
}