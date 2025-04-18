use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::klolaform::form::FormService;
use crate::server::AppState;
use shared::utils::error::AppError;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/forms")
            .route("", web::get().to(list_forms))
            .route("", web::post().to(create_form))
            .route("/{id}", web::get().to(get_form))
            .route("/{id}", web::put().to(update_form))
            .route("/{id}", web::delete().to(delete_form))
            .route("/{id}/publish", web::post().to(publish_form))
            .route("/{id}/unpublish", web::post().to(unpublish_form))
            .route("/{id}/submissions", web::get().to(list_form_submissions))
    );
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormDto {
    pub id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub schema: serde_json::Value,
    pub ui_schema: serde_json::Value,
    pub settings: Option<serde_json::Value>,
    pub status: Option<String>,
}

async fn list_forms(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
) -> impl Responder {
    let form_service = FormService::new(state.db_conn.clone());
    
    match form_service.list_forms(*tenant_id).await {
        Ok(forms) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": forms
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

async fn create_form(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    req: web::Json<FormDto>,
) -> impl Responder {
    let form_service = FormService::new(state.db_conn.clone());
    
    match form_service.create_form(*tenant_id, &req).await {
        Ok(form) => HttpResponse::Created().json({
            serde_json::json!({
                "success": true,
                "data": form
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

async fn get_form(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let form_id = path.into_inner();
    let form_service = FormService::new(state.db_conn.clone());
    
    match form_service.get_form(*tenant_id, form_id).await {
        Ok(Some(form)) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": form
            })
        }),
        Ok(None) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "Form not found"
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

async fn update_form(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
    req: web::Json<FormDto>,
) -> impl Responder {
    let form_id = path.into_inner();
    let form_service = FormService::new(state.db_conn.clone());
    
    match form_service.update_form(*tenant_id, form_id, &req).await {
        Ok(Some(form)) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": form
            })
        }),
        Ok(None) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "Form not found"
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

async fn delete_form(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let form_id = path.into_inner();
    let form_service = FormService::new(state.db_conn.clone());
    
    match form_service.delete_form(*tenant_id, form_id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "Form not found"
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

async fn publish_form(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let form_id = path.into_inner();
    let form_service = FormService::new(state.db_conn.clone());
    
    match form_service.publish_form(*tenant_id, form_id).await {
        Ok(Some(form)) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": form
            })
        }),
        Ok(None) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "Form not found"
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

async fn unpublish_form(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let form_id = path.into_inner();
    let form_service = FormService::new(state.db_conn.clone());
    
    match form_service.unpublish_form(*tenant_id, form_id).await {
        Ok(Some(form)) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": form
            })
        }),
        Ok(None) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "Form not found"
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

async fn list_form_submissions(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let form_id = path.into_inner();
    let form_service = FormService::new(state.db_conn.clone());
    
    match form_service.list_form_submissions(*tenant_id, form_id).await {
        Ok(submissions) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": submissions
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