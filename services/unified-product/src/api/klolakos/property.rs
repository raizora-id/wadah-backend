use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::klolakos::property::PropertyService;
use crate::server::AppState;
use shared::utils::error::AppError;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/properties")
            .route("", web::get().to(list_properties))
            .route("", web::post().to(create_property))
            .route("/{id}", web::get().to(get_property))
            .route("/{id}", web::put().to(update_property))
            .route("/{id}", web::delete().to(delete_property))
    );
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyDto {
    pub id: Option<Uuid>,
    pub name: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    pub country: String,
    pub property_type: String,
    pub units_count: i32,
    pub description: Option<String>,
    pub features: Option<Vec<String>>,
    pub attributes: Option<serde_json::Value>,
}

async fn list_properties(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
) -> impl Responder {
    let property_service = PropertyService::new(state.db_conn.clone());
    
    match property_service.list_properties(*tenant_id).await {
        Ok(properties) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": properties
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

async fn create_property(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    req: web::Json<PropertyDto>,
) -> impl Responder {
    let property_service = PropertyService::new(state.db_conn.clone());
    
    match property_service.create_property(*tenant_id, &req).await {
        Ok(property) => HttpResponse::Created().json({
            serde_json::json!({
                "success": true,
                "data": property
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

async fn get_property(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let property_id = path.into_inner();
    let property_service = PropertyService::new(state.db_conn.clone());
    
    match property_service.get_property(*tenant_id, property_id).await {
        Ok(Some(property)) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": property
            })
        }),
        Ok(None) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "Property not found"
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

async fn update_property(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
    req: web::Json<PropertyDto>,
) -> impl Responder {
    let property_id = path.into_inner();
    let property_service = PropertyService::new(state.db_conn.clone());
    
    match property_service.update_property(*tenant_id, property_id, &req).await {
        Ok(Some(property)) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": property
            })
        }),
        Ok(None) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "Property not found"
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

async fn delete_property(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let property_id = path.into_inner();
    let property_service = PropertyService::new(state.db_conn.clone());
    
    match property_service.delete_property(*tenant_id, property_id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "Property not found"
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