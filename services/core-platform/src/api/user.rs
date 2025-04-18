use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use crate::domain::user::UserService;
use crate::server::AppState;
use shared::models::user::{RegisterUserRequest, UpdateUserRequest};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(list_users))
            .route("", web::post().to(create_user))
            .route("/{id}", web::get().to(get_user))
            .route("/{id}", web::put().to(update_user))
            .route("/{id}", web::delete().to(delete_user))
    );
}

async fn list_users(state: web::Data<std::sync::Arc<AppState>>, tenant_id: web::ReqData<Uuid>) -> impl Responder {
    let user_service = UserService::new(state.db_conn.clone());
    
    match user_service.list_users(*tenant_id).await {
        Ok(users) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": users
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

async fn create_user(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    req: web::Json<RegisterUserRequest>,
) -> impl Responder {
    let user_service = UserService::new(state.db_conn.clone());
    
    match user_service.create_user(*tenant_id, &req).await {
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

async fn get_user(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let user_id = path.into_inner();
    let user_service = UserService::new(state.db_conn.clone());
    
    match user_service.get_user(*tenant_id, user_id).await {
        Ok(Some(user)) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": user
            })
        }),
        Ok(None) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "User not found"
                }
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

async fn update_user(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
    req: web::Json<UpdateUserRequest>,
) -> impl Responder {
    let user_id = path.into_inner();
    let user_service = UserService::new(state.db_conn.clone());
    
    match user_service.update_user(*tenant_id, user_id, &req).await {
        Ok(Some(user)) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": user
            })
        }),
        Ok(None) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "User not found"
                }
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

async fn delete_user(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let user_id = path.into_inner();
    let user_service = UserService::new(state.db_conn.clone());
    
    match user_service.delete_user(*tenant_id, user_id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "User not found"
                }
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
