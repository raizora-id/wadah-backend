use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use crate::domain::nocode::schema::SchemaService;
use crate::server::AppState;
use shared::models::entity::EntityDefinition;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/schema")
            .route("/entities", web::get().to(list_entities))
            .route("/entities", web::post().to(create_entity))
            .route("/entities/{id}", web::get().to(get_entity))
            .route("/entities/{id}", web::put().to(update_entity))
            .route("/entities/{id}", web::delete().to(delete_entity))
    );
}

async fn list_entities(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    query: web::Query<ListEntitiesQuery>,
) -> impl Responder {
    let schema_service = SchemaService::new(state.db_conn.clone());
    
    match schema_service.list_entities(*tenant_id, query.product_id.as_deref()).await {
        Ok(entities) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": entities
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

#[derive(serde::Deserialize)]
struct ListEntitiesQuery {
    product_id: Option<String>,
}

async fn create_entity(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    req: web::Json<CreateEntityRequest>,
) -> impl Responder {
    let schema_service = SchemaService::new(state.db_conn.clone());
    
    match schema_service.create_entity(*tenant_id, &req.into_inner()).await {
        Ok(entity) => HttpResponse::Created().json({
            serde_json::json!({
                "success": true,
                "data": entity
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

#[derive(serde::Deserialize)]
struct CreateEntityRequest {
    pub id: String,
    pub product_id: String,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub schema: serde_json::Value,
    pub ui_schema: serde_json::Value,
}

async fn get_entity(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<String>,
) -> impl Responder {
    let entity_id = path.into_inner();
    let schema_service = SchemaService::new(state.db_conn.clone());
    
    match schema_service.get_entity(*tenant_id, &entity_id).await {
        Ok(Some(entity)) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": entity
            })
        }),
        Ok(None) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "Entity not found"
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

async fn update_entity(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<String>,
    req: web::Json<UpdateEntityRequest>,
) -> impl Responder {
    let entity_id = path.into_inner();
    let schema_service = SchemaService::new(state.db_conn.clone());
    
    match schema_service.update_entity(*tenant_id, &entity_id, &req.into_inner()).await {
        Ok(Some(entity)) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": entity
            })
        }),
        Ok(None) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "Entity not found"
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

#[derive(serde::Deserialize)]
struct UpdateEntityRequest {
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub schema: Option<serde_json::Value>,
    pub ui_schema: Option<serde_json::Value>,
}

async fn delete_entity(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<String>,
) -> impl Responder {
    let entity_id = path.into_inner();
    let schema_service = SchemaService::new(state.db_conn.clone());
    
    match schema_service.delete_entity(*tenant_id, &entity_id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "Entity not found"
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
