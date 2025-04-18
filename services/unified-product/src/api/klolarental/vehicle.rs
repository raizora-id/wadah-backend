use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::klolarental::vehicle::VehicleService;
use crate::server::AppState;
use shared::utils::error::AppError;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/vehicles")
            .route("", web::get().to(list_vehicles))
            .route("", web::post().to(create_vehicle))
            .route("/{id}", web::get().to(get_vehicle))
            .route("/{id}", web::put().to(update_vehicle))
            .route("/{id}", web::delete().to(delete_vehicle))
            .route("/{id}/availability", web::get().to(get_vehicle_availability))
            .route("/{id}/maintenance", web::post().to(add_maintenance_record))
            .route("/{id}/maintenance", web::get().to(list_maintenance_records))
    );
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehicleDto {
    pub id: Option<Uuid>,
    pub name: String,
    pub make: String,
    pub model: String,
    pub year: i32,
    pub registration: String,
    pub category: String,
    pub status: Option<String>,
    pub daily_rate: f64,
    pub hourly_rate: Option<f64>,
    pub capacity: i32,
    pub features: Option<Vec<String>>,
    pub specifications: Option<serde_json::Value>,
    pub location: Option<serde_json::Value>,
    pub images: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaintenanceRecordDto {
    pub title: String,
    pub description: String,
    pub maintenance_type: String,
    pub cost: Option<f64>,
    pub performed_by: String,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateRangeQuery {
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
}

async fn list_vehicles(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    query: web::Query<VehicleQuery>,
) -> impl Responder {
    let vehicle_service = VehicleService::new(state.db_conn.clone());
    
    match vehicle_service.list_vehicles(
        *tenant_id, 
        query.category.as_deref(), 
        query.status.as_deref()
    ).await {
        Ok(vehicles) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": vehicles
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
struct VehicleQuery {
    category: Option<String>,
    status: Option<String>,
}

async fn create_vehicle(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    req: web::Json<VehicleDto>,
) -> impl Responder {
    let vehicle_service = VehicleService::new(state.db_conn.clone());
    
    match vehicle_service.create_vehicle(*tenant_id, &req).await {
        Ok(vehicle) => HttpResponse::Created().json({
            serde_json::json!({
                "success": true,
                "data": vehicle
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

async fn get_vehicle(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let vehicle_id = path.into_inner();
    let vehicle_service = VehicleService::new(state.db_conn.clone());
    
    match vehicle_service.get_vehicle(*tenant_id, vehicle_id).await {
        Ok(Some(vehicle)) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": vehicle
            })
        }),
        Ok(None) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "Vehicle not found"
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

async fn update_vehicle(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
    req: web::Json<VehicleDto>,
) -> impl Responder {
    let vehicle_id = path.into_inner();
    let vehicle_service = VehicleService::new(state.db_conn.clone());
    
    match vehicle_service.update_vehicle(*tenant_id, vehicle_id, &req).await {
        Ok(Some(vehicle)) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": vehicle
            })
        }),
        Ok(None) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "Vehicle not found"
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

async fn delete_vehicle(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let vehicle_id = path.into_inner();
    let vehicle_service = VehicleService::new(state.db_conn.clone());
    
    match vehicle_service.delete_vehicle(*tenant_id, vehicle_id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json({
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "NOT_FOUND_ERROR",
                    "message": "Vehicle not found"
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

async fn get_vehicle_availability(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
    query: web::Query<DateRangeQuery>,
) -> impl Responder {
    let vehicle_id = path.into_inner();
    let vehicle_service = VehicleService::new(state.db_conn.clone());
    
    match vehicle_service.get_vehicle_availability(
        *tenant_id,
        vehicle_id,
        query.start_date,
        query.end_date,
    ).await {
        Ok(availability) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": availability
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

async fn add_maintenance_record(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
    req: web::Json<MaintenanceRecordDto>,
) -> impl Responder {
    let vehicle_id = path.into_inner();
    let vehicle_service = VehicleService::new(state.db_conn.clone());
    
    match vehicle_service.add_maintenance_record(*tenant_id, vehicle_id, &req).await {
        Ok(record) => HttpResponse::Created().json({
            serde_json::json!({
                "success": true,
                "data": record
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

async fn list_maintenance_records(
    state: web::Data<std::sync::Arc<AppState>>,
    tenant_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let vehicle_id = path.into_inner();
    let vehicle_service = VehicleService::new(state.db_conn.clone());
    
    match vehicle_service.list_maintenance_records(*tenant_id, vehicle_id).await {
        Ok(records) => HttpResponse::Ok().json({
            serde_json::json!({
                "success": true,
                "data": records
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