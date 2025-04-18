use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use crate::domain::subscription::SubscriptionService;
use crate::server::AppState;
use shared::models::subscription::{CreateSubscriptionRequest, UpdateSubscriptionRequest};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/subscriptions")
            .route("", web::get().to(list_subscriptions))
            .route("", web::post().to(create_subscription))
            .route("/{id}", web::get().to(get_subscription))
            .route("/{id}", web::put().to(update_subscription))
            .route("/{id}", web::delete().to(cancel_subscription))
            .service(
                web::scope("/products")
                    .route("", web::get().to(list_products))
                    .route("/{id}/plans", web::get().to(list_product_plans))
            )
    );
}

async fn list_subscriptions(state: web::Data<std::sync::Arc<AppState>>, tenant_id: web::ReqData<Uuid>) -> impl Responder {
    let subscription_service = SubscriptionService::new(state.db_conn.clone());
    
    match subscription_service.list_subscriptions(*tenant_id).await {
        Ok(subscriptions) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": subscriptions
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

async fn create_subscription(
    state: web::Data<std::sync::Arc<AppState>>,
    req: web::Json<CreateSubscriptionRequest>,
) -> impl Responder {
    let subscription_service = SubscriptionService::new(state.db_conn.clone());
    
    match subscription_service.create_subscription(&req).await {
        Ok(subscription) => HttpResponse::Created().json({
            serde_json::json!({
                "success": true,
                "data": subscription
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

async fn get_subscription(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let subscription_id = path.into_inner();
    let subscription_service = SubscriptionService::new(state.db_conn.clone());
    
    match subscription_service.get_subscription(*tenant_id, subscription_id).await {
        Ok(Some(subscription)) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": subscription
            })
        }),
        Ok(None) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "Subscription not found"
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

async fn update_subscription(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
    req: web::Json<UpdateSubscriptionRequest>,
) -> impl Responder {
    let subscription_id = path.into_inner();
    let subscription_service = SubscriptionService::new(state.db_conn.clone());
    
    match subscription_service.update_subscription(*tenant_id, subscription_id, &req).await {
        Ok(Some(subscription)) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": subscription
            })
        }),
        Ok(None) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "Subscription not found"
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

async fn cancel_subscription(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let subscription_id = path.into_inner();
    let subscription_service = SubscriptionService::new(state.db_conn.clone());
    
    match subscription_service.cancel_subscription(*tenant_id, subscription_id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "Subscription not found"
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

async fn list_products(state: web::Data<std::sync::Arc<AppState>>) -> impl Responder {
    let subscription_service = SubscriptionService::new(state.db_conn.clone());
    
    match subscription_service.list_products().await {
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

async fn list_product_plans(
    state: web::Data<std::sync::Arc<AppState>>,
    path: web::Path<String>,
) -> impl Responder {
    let product_id = path.into_inner();
    let subscription_service = SubscriptionService::new(state.db_conn.clone());
    
    match subscription_service.list_product_plans(&product_id).await {
        Ok(plans) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": plans
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
