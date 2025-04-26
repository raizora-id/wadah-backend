use std::net::SocketAddr;
use axum::{
    routing::get,
    Router,
};
use infrastructure::{
    config::Config,
    persistence::postgres::PostgresUserRepository,
};
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    dotenv::dotenv().ok();
    let config = Config::from_env()?;

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("LOG_LEVEL").unwrap_or_else(|_| "debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize database
    let pool = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect(&config.database.url)
        .await?;

    // Run migrations
    sqlx::migrate!("../infrastructure/migrations")
        .run(&pool)
        .await?;

    // Initialize repositories
    let user_repository = PostgresUserRepository::new(pool);

    // Build our application
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(TraceLayer::new_for_http())
        .with_state(user_repository);

    // Run it
    let addr = SocketAddr::from((
        config.server.host.parse()?,
        config.server.port,
    ));
    tracing::info!("listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
