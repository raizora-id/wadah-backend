use std::{net::SocketAddr, time::Duration};
use axum::{
    routing::{get, IntoMakeService},
    Router,
};
use infrastructure::{
    config::Config,
    persistence::postgres::PostgresUserRepository,
};
use metrics_exporter_prometheus::PrometheusHandle;
use sqlx::postgres::PgPoolOptions;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    trace::TraceLayer,
    timeout::TimeoutLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod middleware;
use middleware::{cors, metrics, rate_limit, request_id};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    dotenv::dotenv().ok();
    let config = Config::new()?;

    // Initialize metrics
    metrics::setup_metrics_recorder();
    let prometheus_handle = setup_prometheus_metrics();

    // Initialize tracing with JSON format for production
    let is_prod = config.is_production();
    let env_filter = tracing_subscriber::EnvFilter::new(
        std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".into()),
    );

    let subscriber = tracing_subscriber::registry().with(env_filter);

    if is_prod {
        subscriber.with(tracing_subscriber::fmt::layer().json()).init();
    } else {
        subscriber.with(tracing_subscriber::fmt::layer()).init();
    }

    // Initialize database with retry logic
    let pool = retry_connect_database(&config).await?;

    // Run migrations if enabled (typically in development)
    if !is_prod {
        tracing::info!("Running database migrations");
        sqlx::migrate!("../infrastructure/migrations")
            .run(&pool)
            .await?;
    }

    // Initialize repositories
    let user_repository = PostgresUserRepository::new(pool.clone());

    // Create middleware stack
    let middleware_stack = ServiceBuilder::new()
        .layer(request_id::create_request_id_middleware())
        .layer(metrics::track_metrics)
        .layer(TraceLayer::new_for_http())
        .layer(cors::create_cors_middleware())
        .layer(CompressionLayer::new())
        .layer(TimeoutLayer::new(Duration::from_secs(30)));

    if is_prod {
        middleware_stack.layer(rate_limit::create_rate_limit_middleware());
    }

    // Build our application
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/metrics", get(move || metrics_handler(prometheus_handle.clone())))
        .route("/", get(|| async { "Hello, World!" }))
        .with_state(user_repository)
        .layer(middleware_stack.into_inner());

    // Setup graceful shutdown
    let addr = SocketAddr::from((
        config.server.host.parse()?,
        config.server.port,
    ));
    
    tracing::info!("listening on {}", addr);
    
    let server = axum::Server::bind(&addr)
        .serve(app.into_make_service());

    // Graceful shutdown
    let graceful = server.with_graceful_shutdown(shutdown_signal());

    // Start the server
    if let Err(e) = graceful.await {
        tracing::error!("server error: {}", e);
    }

    // Cleanup
    tracing::info!("shutting down");
    pool.close().await;

    Ok(())
}

// Prometheus metrics handler
async fn metrics_handler(prometheus_handle: PrometheusHandle) -> String {
    prometheus_handle.render()
}

// Setup Prometheus metrics
fn setup_prometheus_metrics() -> PrometheusHandle {
    use metrics_exporter_prometheus::PrometheusBuilder;
    PrometheusBuilder::new()
        .install_recorder()
        .expect("failed to install Prometheus recorder")
}

// Health check endpoint
async fn health_check() -> &'static str {
    "OK"
}

// Graceful shutdown signal handler
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("shutdown signal received");
}

// Database connection with retry logic
async fn retry_connect_database(config: &Config) -> Result<sqlx::PgPool, sqlx::Error> {
    let mut retry_count = 0;
    let max_retries = 5;
    let retry_delay = Duration::from_secs(5);

    loop {
        match PgPoolOptions::new()
            .max_connections(config.database.max_connections)
            .acquire_timeout(Duration::from_secs(3))
            .connect(&config.database.url)
            .await
        {
            Ok(pool) => {
                tracing::info!("Database connection established");
                return Ok(pool);
            }
            Err(e) => {
                retry_count += 1;
                if retry_count >= max_retries {
                    tracing::error!("Failed to connect to database after {} retries", max_retries);
                    return Err(e);
                }
                tracing::warn!(
                    "Failed to connect to database, retrying in {} seconds (attempt {}/{})",
                    retry_delay.as_secs(),
                    retry_count,
                    max_retries
                );
                tokio::time::sleep(retry_delay).await;
            }
        }
    }
}
