use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::auth::AuthService;
use crate::server::AppState;
use shared::models::user::{LoginRequest, RegisterUserRequest};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(login))
            .route("/register", web::post().to(register))
            .route("/refresh-token", web::post().to(refresh_token))
            .route("/me", web::get().to(me))
    );
}

async fn login(state: web::Data<std::sync::Arc<AppState>>, req: web::Json<LoginRequest>) -> impl Responder {
    let auth_service = AuthService::new(state.db_conn.clone(), state.redis_conn.clone(), state.config.jwt_secret.clone(), state.config.jwt_expiry);
    
    match auth_service.login(&req.email, &req.password).await {
        Ok(tokens) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": tokens
            })
        }),
        Err(e) => HttpResponse::Unauthorized().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "AUTHENTICATION_ERROR",
                    "message": e.to_string()
                }
            })
        }),
    }
}

async fn register(state: web::Data<std::sync::Arc<AppState>>, req: web::Json<RegisterUserRequest>) -> impl Responder {
    let auth_service = AuthService::new(state.db_conn.clone(), state.redis_conn.clone(), state.config.jwt_secret.clone(), state.config.jwt_expiry);
    
    match auth_service.register(&req).await {
        Ok(user) => HttpResponse::Created().json({
            serde_json::json!({
                "success": true,
                "data": user
            })
        }),
        Err(e) => HttpResponse::BadRequest().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "VALIDATION_ERROR",
                    "message": e.to_string()
                }
            })
        }),
    }
}

#[derive(Deserialize)]
struct RefreshTokenRequest {
    refresh_token: String,
}

async fn refresh_token(state: web::Data<std::sync::Arc<AppState>>, req: web::Json<RefreshTokenRequest>) -> impl Responder {
    let auth_service = AuthService::new(state.db_conn.clone(), state.redis_conn.clone(), state.config.jwt_secret.clone(), state.config.jwt_expiry);
    
    match auth_service.refresh_token(&req.refresh_token).await {
        Ok(tokens) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": tokens
            })
        }),
        Err(e) => HttpResponse::Unauthorized().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "AUTHENTICATION_ERROR",
                    "message": e.to_string()
                }
            })
        }),
    }
}

async fn me(state: web::Data<std::sync::Arc<AppState>>, user_id: web::ReqData<Uuid>) -> impl Responder {
    let auth_service = AuthService::new(state.db_conn.clone(), state.redis_conn.clone(), state.config.jwt_secret.clone(), state.config.jwt_expiry);
    
    match auth_service.get_current_user(*user_id).await {
        Ok(user) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": user
            })
        }),
        Err(e) => HttpResponse::InternalServerError().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "INTERNAL_SERVER_ERROR",
                    "message": e.to_string()
                }
            })
        }),
    }
}
