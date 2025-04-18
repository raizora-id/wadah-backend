use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use crate::domain::tenant::TenantService;
use crate::server::AppState;
use shared::models::tenant::{CreateTenantRequest, UpdateTenantRequest};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tenants")
            .route("", web::get().to(list_tenants))
            .route("", web::post().to(create_tenant))
            .route("/{id}", web::get().to(get_tenant))
            .route("/{id}", web::put().to(update_tenant))
            .route("/{id}", web::delete().to(delete_tenant))
    );
}

async fn list_tenants(state: web::Data<std::sync::Arc<AppState>>) -> impl Responder {
    let tenant_service = TenantService::new(state.db_conn.clone());
    
    match tenant_service.list_tenants().await {
        Ok(tenants) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": tenants
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

async fn create_tenant(state: web::Data<std::sync::Arc<AppState>>, req: web::Json<CreateTenantRequest>) -> impl Responder {
    let tenant_service = TenantService::new(state.db_conn.clone());
    
    match tenant_service.create_tenant(&req).await {
        Ok(tenant) => HttpResponse::Created().json({
            serde_json::json!({
                "success": true,
                "data": tenant
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

async fn get_tenant(state: web::Data<std::sync::Arc<AppState>>, path: web::Path<Uuid>) -> impl Responder {
    let tenant_id = path.into_inner();
    let tenant_service = TenantService::new(state.db_conn.clone());
    
    match tenant_service.get_tenant(tenant_id).await {
        Ok(Some(tenant)) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": tenant
            })
        }),
        Ok(None) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "Tenant not found"
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

async fn update_tenant(
    state: web::Data<std::sync::Arc<AppState>>,
    path: web::Path<Uuid>,
    req: web::Json<UpdateTenantRequest>,
) -> impl Responder {
    let tenant_id = path.into_inner();
    let tenant_service = TenantService::new(state.db_conn.clone());
    
    match tenant_service.update_tenant(tenant_id, &req).await {
        Ok(Some(tenant)) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": tenant
            })
        }),
        Ok(None) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "Tenant not found"
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

async fn delete_tenant(
    state: web::Data<std::sync::Arc<AppState>>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let tenant_id = path.into_inner();
    let tenant_service = TenantService::new(state.db_conn.clone());
    
    match tenant_service.delete_tenant(tenant_id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "Tenant not found"
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
