use tower_http::cors::{CorsLayer, Origin};
use std::time::Duration;

pub fn create_cors_middleware() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([
            http::Method::GET,
            http::Method::POST,
            http::Method::PUT,
            http::Method::DELETE,
            http::Method::OPTIONS,
            http::Method::PATCH,
        ])
        .allow_headers([
            http::header::AUTHORIZATION,
            http::header::ACCEPT,
            http::header::CONTENT_TYPE,
        ])
        .allow_credentials(true)
        .max_age(Duration::from_secs(3600))
        .allow_origin(Origin::list(vec![
            "http://localhost:3000".parse().unwrap(),
            "http://127.0.0.1:3000".parse().unwrap(),
        ]))
}
