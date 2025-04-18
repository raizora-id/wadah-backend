use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::klolarental::booking::BookingService;
use crate::server::AppState;
use shared::utils::error::AppError;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/bookings")
            .route("", web::get().to(list_bookings))
            .route("", web::post().to(create_booking))
            .route("/{id}", web::get().to(get_booking))
            .route("/{id}", web::put().to(update_booking))
            .route("/{id}", web::delete().to(cancel_booking))
            .route("/{id}/confirm", web::post().to(confirm_booking))
            .route("/{id}/complete", web::post().to(complete_booking))
            .route("/{id}/invoice", web::get().to(get_booking_invoice))
            .route("/check-availability", web::post().to(check_availability))
    );
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookingDto {
    pub id: Option<Uuid>,
    pub vehicle_id: Uuid,
    pub customer_id: Option<Uuid>,
    pub customer_info: Option<CustomerInfoDto>,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
    pub pickup_location: Option<serde_json::Value>,
    pub dropoff_location: Option<serde_json::Value>,
    pub status: Option<String>,
    pub notes: Option<String>,
    pub additional_services: Option<Vec<String>>,
    pub total_price: Option<f64>,
    pub deposit_amount: Option<f64>,
    pub payment_status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerInfoDto {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: Option<String>,
    pub driver_license: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AvailabilityCheckDto {
    pub vehicle_id: Option<Uuid>,
    pub vehicle_category: Option<String>,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
}

async fn list_bookings(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    query: web::Query<BookingQuery>,
) -> impl Responder {
    let booking_service = BookingService::new(state.db_conn.clone());
    
    match booking_service.list_bookings(
        *tenant_id,
        query.status.as_deref(),
        query.start_date,
        query.end_date,
        query.vehicle_id,
    ).await {
        Ok(bookings) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": bookings
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
struct BookingQuery {
    status: Option<String>,
    start_date: Option<chrono::DateTime<chrono::Utc>>,
    end_date: Option<chrono::DateTime<chrono::Utc>>,
    vehicle_id: Option<Uuid>,
}

async fn create_booking(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    req: web::Json<BookingDto>,
) -> impl Responder {
    let booking_service = BookingService::new(state.db_conn.clone());
    
    match booking_service.create_booking(*tenant_id, &req).await {
        Ok(booking) => HttpResponse::Created().json({
            serde_json::json!({
                "success": true,
                "data": booking
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

async fn get_booking(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let booking_id = path.into_inner();
    let booking_service = BookingService::new(state.db_conn.clone());
    
    match booking_service.get_booking(*tenant_id, booking_id).await {
        Ok(Some(booking)) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": booking
            })
        }),
        Ok(None) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "Booking not found"
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

async fn update_booking(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
    req: web::Json<BookingDto>,
) -> impl Responder {
    let booking_id = path.into_inner();
    let booking_service = BookingService::new(state.db_conn.clone());
    
    match booking_service.update_booking(*tenant_id, booking_id, &req).await {
        Ok(Some(booking)) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": booking
            })
        }),
        Ok(None) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "Booking not found"
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

async fn cancel_booking(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let booking_id = path.into_inner();
    let booking_service = BookingService::new(state.db_conn.clone());
    
    match booking_service.cancel_booking(*tenant_id, booking_id).await {
        Ok(Some(booking)) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": booking
            })
        }),
        Ok(None) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "Booking not found"
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

async fn confirm_booking(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let booking_id = path.into_inner();
    let booking_service = BookingService::new(state.db_conn.clone());
    
    match booking_service.update_booking_status(*tenant_id, booking_id, "confirmed").await {
        Ok(Some(booking)) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": booking
            })
        }),
        Ok(None) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "Booking not found"
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

async fn complete_booking(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let booking_id = path.into_inner();
    let booking_service = BookingService::new(state.db_conn.clone());
    
    match booking_service.update_booking_status(*tenant_id, booking_id, "completed").await {
        Ok(Some(booking)) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": booking
            })
        }),
        Ok(None) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "Booking not found"
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

async fn get_booking_invoice(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let booking_id = path.into_inner();
    let booking_service = BookingService::new(state.db_conn.clone());
    
    match booking_service.get_booking_invoice(*tenant_id, booking_id).await {
        Ok(Some(invoice)) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": invoice
            })
        }),
        Ok(None) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "Booking or invoice not found"
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

async fn check_availability(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    req: web::Json<AvailabilityCheckDto>,
) -> impl Responder {
    let booking_service = BookingService::new(state.db_conn.clone());
    
    match booking_service.check_availability(
        *tenant_id,
        req.vehicle_id,
        req.vehicle_category.as_deref(),
        req.start_date,
        req.end_date,
    ).await {
        Ok(available_vehicles) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": available_vehicles
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