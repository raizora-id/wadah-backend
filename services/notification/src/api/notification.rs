use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::notification::NotificationService;
use crate::server::AppState;
use shared::utils::error::AppError;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/notifications")
            .route("", web::get().to(list_notifications))
            .route("", web::post().to(send_notification))
            .route("/{id}", web::get().to(get_notification))
            .route("/{id}/read", web::put().to(mark_as_read))
            .route("/batch", web::post().to(send_batch_notifications))
    );
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationDto {
    pub id: Option<Uuid>,
    pub recipient_id: Uuid,
    pub template_id: Option<Uuid>,
    pub channel: String,
    pub title: String,
    pub body: String,
    pub data: Option<serde_json::Value>,
    pub scheduled_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchNotificationDto {
    pub recipient_ids: Vec<Uuid>,
    pub template_id: Option<Uuid>,
    pub channel: String,
    pub title: String,
    pub body: String,
    pub data: Option<serde_json::Value>,
    pub scheduled_at: Option<chrono::DateTime<chrono::Utc>>,
}

async fn list_notifications(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    recipient_id: web::Query<RecipientQuery>,
) -> impl Responder {
    let notification_service = NotificationService::new(state.db_conn.clone(), state.redis_conn.clone(), state.config.clone());
    
    match notification_service.list_notifications(*tenant_id, recipient_id.recipient_id).await {
        Ok(notifications) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": notifications
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

#[derive(Debug, Deserialize)]
struct RecipientQuery {
    recipient_id: Uuid,
}

async fn send_notification(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    req: web::Json<NotificationDto>,
) -> impl Responder {
    let notification_service = NotificationService::new(state.db_conn.clone(), state.redis_conn.clone(), state.config.clone());
    
    match notification_service.send_notification(*tenant_id, &req).await {
        Ok(notification) => HttpResponse::Created().json({
            serde_json::json!({
                "success": true,
                "data": notification
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

async fn get_notification(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let notification_id = path.into_inner();
    let notification_service = NotificationService::new(state.db_conn.clone(), state.redis_conn.clone(), state.config.clone());
    
    match notification_service.get_notification(*tenant_id, notification_id).await {
        Ok(Some(notification)) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": notification
            })
        }),
        Ok(None) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "Notification not found"
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

async fn mark_as_read(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let notification_id = path.into_inner();
    let notification_service = NotificationService::new(state.db_conn.clone(), state.redis_conn.clone(), state.config.clone());
    
    match notification_service.mark_as_read(*tenant_id, notification_id).await {
        Ok(true) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true
            })
        }),
        Ok(false) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "Notification not found"
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

async fn send_batch_notifications(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    req: web::Json<BatchNotificationDto>,
) -> impl Responder {
    let notification_service = NotificationService::new(state.db_conn.clone(), state.redis_conn.clone(), state.config.clone());
    
    match notification_service.send_batch_notifications(*tenant_id, &req).await {
        Ok(batch_id) => HttpResponse::Accepted().json({
            serde_json::json!({
                "success": true,
                "data": {
                    "batch_id": batch_id,
                    "count": req.recipient_ids.len()
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