use std::sync::Arc;

use actix_web::{web, App, HttpServer};

use crate::api;
use crate::config::Config;
use crate::middleware::auth::AuthMiddleware;
use shared::{DatabaseConnection, RedisConnection};

pub struct AppState {
    pub config: Config,
    pub db_conn: DatabaseConnection,
    pub redis_conn: RedisConnection,
}

pub struct Server {
    app_state: Arc<AppState>,
}

impl Server {
    pub fn new(app_state: Arc<AppState>) -> Self {
        Server { app_state }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        let app_state = self.app_state.clone();
        let host = app_state.config.server.host.clone();
        let port = app_state.config.server.port;

        tracing::info!("Starting unified product service at http://{}:{}", host, port);

        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(app_state.clone()))
                .wrap(AuthMiddleware::new(app_state.clone()))
                .configure(api::configure)
        })
        .bind((host, port))?
        .run()
        .await
    }
}