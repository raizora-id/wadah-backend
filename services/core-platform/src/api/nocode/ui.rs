use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use crate::domain::nocode::ui::UiService;
use crate::server::AppState;
use shared::models::entity::{UiDefinition, ViewType};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/ui")
            .route("/definitions", web::get().to(list_ui_definitions))
            .route("/definitions", web::post().to(create_ui_definition))
            .route("/definitions/{id}", web::get().to(get_ui_definition))
            .route("/definitions/{id}", web::put().to(update_ui_definition))
            .route("/definitions/{id}", web::delete().to(delete_ui_definition))
            .route("/render/{entity_id}/{view_type}", web::get().to(render_ui))
    );
}

async fn list_ui_definitions(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    query: web::Query<ListUiDefinitionsQuery>,
) -> impl Responder {
    let ui_service = UiService::new(state.db_conn.clone());
    
    match ui_service.list_ui_definitions(*tenant_id, query.entity_id.as_deref()).await {
        Ok(definitions) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": definitions
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
struct ListUiDefinitionsQuery {
    entity_id: Option<String>,
}

async fn create_ui_definition(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    req: web::Json<CreateUiDefinitionRequest>,
) -> impl Responder {
    let ui_service = UiService::new(state.db_conn.clone());
    
    match ui_service.create_ui_definition(*tenant_id, &req.into_inner()).await {
        Ok(definition) => HttpResponse::Created().json({
            serde_json::json!({
                "success": true,
                "data": definition
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
struct CreateUiDefinitionRequest {
    pub entity_id: String,
    pub view_type: ViewType,
    pub definition: serde_json::Value,
}

async fn get_ui_definition(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let definition_id = path.into_inner();
    let ui_service = UiService::new(state.db_conn.clone());
    
    match ui_service.get_ui_definition(*tenant_id, definition_id).await {
        Ok(Some(definition)) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": definition
            })
        }),
        Ok(None) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "UI definition not found"
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

async fn update_ui_definition(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
    req: web::Json<UpdateUiDefinitionRequest>,
) -> impl Responder {
    let definition_id = path.into_inner();
    let ui_service = UiService::new(state.db_conn.clone());
    
    match ui_service.update_ui_definition(*tenant_id, definition_id, &req.into_inner()).await {
        Ok(Some(definition)) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": definition
            })
        }),
        Ok(None) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "UI definition not found"
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
struct UpdateUiDefinitionRequest {
    pub view_type: Option<ViewType>,
    pub definition: Option<serde_json::Value>,
}

async fn delete_ui_definition(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let definition_id = path.into_inner();
    let ui_service = UiService::new(state.db_conn.clone());
    
    match ui_service.delete_ui_definition(*tenant_id, definition_id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "UI definition not found"
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

async fn render_ui(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (entity_id, view_type_str) = path.into_inner();
    let view_type = match view_type_str.as_str() {
        "list" => ViewType::List,
        "detail" => ViewType::Detail,
        "form" => ViewType::Form,
        "dashboard" => ViewType::Dashboard,
        _ => ViewType::Custom,
    };
    
    let ui_service = UiService::new(state.db_conn.clone());
    
    match ui_service.render_ui(*tenant_id, &entity_id, view_type).await {
        Ok(Some(ui)) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": ui
            })
        }),
        Ok(None) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "UI definition not found for this entity and view type"
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
