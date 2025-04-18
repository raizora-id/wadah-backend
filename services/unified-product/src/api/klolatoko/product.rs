use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::klolatoko::product::ProductService;
use crate::server::AppState;
use shared::utils::error::AppError;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .route("", web::get().to(list_products))
            .route("", web::post().to(create_product))
            .route("/{id}", web::get().to(get_product))
            .route("/{id}", web::put().to(update_product))
            .route("/{id}", web::delete().to(delete_product))
    );
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductDto {
    pub id: Option<Uuid>,
    pub name: String,
    pub sku: String,
    pub description: Option<String>,
    pub price: f64,
    pub cost: Option<f64>,
    pub quantity: i32,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub attributes: Option<serde_json::Value>,
}

async fn list_products(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
) -> impl Responder {
    let product_service = ProductService::new(state.db_conn.clone());
    
    match product_service.list_products(*tenant_id).await {
        Ok(products) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": products
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

async fn create_product(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    req: web::Json<ProductDto>,
) -> impl Responder {
    let product_service = ProductService::new(state.db_conn.clone());
    
    match product_service.create_product(*tenant_id, &req).await {
        Ok(product) => HttpResponse::Created().json({
            serde_json::json!({
                "success": true,
                "data": product
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

async fn get_product(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let product_id = path.into_inner();
    let product_service = ProductService::new(state.db_conn.clone());
    
    match product_service.get_product(*tenant_id, product_id).await {
        Ok(Some(product)) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": product
            })
        }),
        Ok(None) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "Product not found"
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

async fn update_product(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
    req: web::Json<ProductDto>,
) -> impl Responder {
    let product_id = path.into_inner();
    let product_service = ProductService::new(state.db_conn.clone());
    
    match product_service.update_product(*tenant_id, product_id, &req).await {
        Ok(Some(product)) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": product
            })
        }),
        Ok(None) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "Product not found"
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

async fn delete_product(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let product_id = path.into_inner();
    let product_service = ProductService::new(state.db_conn.clone());
    
    match product_service.delete_product(*tenant_id, product_id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "Product not found"
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