use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use crate::domain::nocode::workflow::WorkflowService;
use crate::server::AppState;
use shared::models::entity::{WorkflowDefinition, WorkflowStep};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/workflow")
            .route("/definitions", web::get().to(list_workflow_definitions))
            .route("/definitions", web::post().to(create_workflow_definition))
            .route("/definitions/{id}", web::get().to(get_workflow_definition))
            .route("/definitions/{id}", web::put().to(update_workflow_definition))
            .route("/definitions/{id}", web::delete().to(delete_workflow_definition))
            .route("/execute/{id}", web::post().to(execute_workflow))
    );
}

async fn list_workflow_definitions(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    query: web::Query<ListWorkflowsQuery>,
) -> impl Responder {
    let workflow_service = WorkflowService::new(state.db_conn.clone());
    
    match workflow_service.list_workflow_definitions(*tenant_id, query.entity_id.as_deref()).await {
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
struct ListWorkflowsQuery {
    entity_id: Option<String>,
}

async fn create_workflow_definition(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    req: web::Json<CreateWorkflowRequest>,
) -> impl Responder {
    let workflow_service = WorkflowService::new(state.db_conn.clone());
    
    match workflow_service.create_workflow_definition(*tenant_id, &req.into_inner()).await {
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
struct CreateWorkflowRequest {
    pub name: String,
    pub description: Option<String>,
    pub entity_id: Option<String>,
    pub trigger_type: String,
    pub trigger_config: serde_json::Value,
    pub steps: Vec<WorkflowStep>,
}

async fn get_workflow_definition(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let workflow_id = path.into_inner();
    let workflow_service = WorkflowService::new(state.db_conn.clone());
    
    match workflow_service.get_workflow_definition(*tenant_id, workflow_id).await {
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
                    "message": "Workflow definition not found"
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

async fn update_workflow_definition(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
    req: web::Json<UpdateWorkflowRequest>,
) -> impl Responder {
    let workflow_id = path.into_inner();
    let workflow_service = WorkflowService::new(state.db_conn.clone());
    
    match workflow_service.update_workflow_definition(*tenant_id, workflow_id, &req.into_inner()).await {
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
                    "message": "Workflow definition not found"
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
struct UpdateWorkflowRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub entity_id: Option<String>,
    pub trigger_type: Option<String>,
    pub trigger_config: Option<serde_json::Value>,
    pub steps: Option<Vec<WorkflowStep>>,
    pub status: Option<String>,
}

async fn delete_workflow_definition(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let workflow_id = path.into_inner();
    let workflow_service = WorkflowService::new(state.db_conn.clone());
    
    match workflow_service.delete_workflow_definition(*tenant_id, workflow_id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "Workflow definition not found"
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

async fn execute_workflow(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
    req: web::Json<ExecuteWorkflowRequest>,
) -> impl Responder {
    let workflow_id = path.into_inner();
    let workflow_service = WorkflowService::new(state.db_conn.clone());
    
    match workflow_service.execute_workflow(*tenant_id, workflow_id, &req.data).await {
        Ok(result) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": result
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
struct ExecuteWorkflowRequest {
    pub data: serde_json::Value,
}
