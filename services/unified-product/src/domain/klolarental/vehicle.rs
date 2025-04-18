use chrono::Utc;
use uuid::Uuid;

use crate::api::klolarental::vehicle::{VehicleDto, MaintenanceRecordDto};
use shared::{
    DatabaseConnection,
    utils::error::AppError,
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Vehicle {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub make: String,
    pub model: String,
    pub year: i32,
    pub registration: String,
    pub category: String,
    pub status: String,
    pub daily_rate: f64,
    pub hourly_rate: Option<f64>,
    pub capacity: i32,
    pub features: Option<Vec<String>>,
    pub specifications: Option<serde_json::Value>,
    pub location: Option<serde_json::Value>,
    pub images: Option<Vec<String>>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MaintenanceRecord {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub vehicle_id: Uuid,
    pub title: String,
    pub description: String,
    pub maintenance_type: String,
    pub cost: Option<f64>,
    pub performed_by: String,
    pub start_date: chrono::DateTime<Utc>,
    pub end_date: Option<chrono::DateTime<Utc>>,
    pub notes: Option<String>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct VehicleAvailability {
    pub vehicle_id: Uuid,
    pub date: chrono::NaiveDate,
    pub available: bool,
    pub reason: Option<String>,
    pub booking_id: Option<Uuid>,
}

pub struct VehicleService {
    db_conn: DatabaseConnection,
}

impl VehicleService {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        Self { db_conn }
    }
    
    pub async fn list_vehicles(
        &self, 
        tenant_id: Uuid, 
        category: Option<&str>, 
        status: Option<&str>
    ) -> Result<Vec<Vehicle>, AppError> {
        // In a real implementation, this would query the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Mock vehicles for testing
        let mut vehicles = vec![
            Vehicle {
                id: Uuid::new_v4(),
                tenant_id,
                name: "Toyota Camry 2020".to_string(),
                make: "Toyota".to_string(),
                model: "Camry".to_string(),
                year: 2020,
                registration: "ABC-123".to_string(),
                category: "sedan".to_string(),
                status: "available".to_string(),
                daily_rate: 50.0,
                hourly_rate: Some(15.0),
                capacity: 5,
                features: Some(vec![
                    "Air Conditioning".to_string(),
                    "Bluetooth".to_string(),
                    "Cruise Control".to_string(),
                ]),
                specifications: Some(serde_json::json!({
                    "fuel_type": "gasoline",
                    "transmission": "automatic",
                    "engine": "2.5L"
                })),
                location: Some(serde_json::json!({
                    "address": "123 Main St, Chicago, IL",
                    "coordinates": {
                        "lat": 41.8781,
                        "lng": -87.6298
                    }
                })),
                images: Some(vec![
                    "toyota_camry_front.jpg".to_string(),
                    "toyota_camry_interior.jpg".to_string(),
                ]),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            Vehicle {
                id: Uuid::new_v4(),
                tenant_id,
                name: "Ford F-150 2019".to_string(),
                make: "Ford".to_string(),
                model: "F-150".to_string(),
                year: 2019,
                registration: "XYZ-789".to_string(),
                category: "truck".to_string(),
                status: "maintenance".to_string(),
                daily_rate: 80.0,
                hourly_rate: Some(25.0),
                capacity: 5,
                features: Some(vec![
                    "4WD".to_string(),
                    "Towing Package".to_string(),
                    "Backup Camera".to_string(),
                ]),
                specifications: Some(serde_json::json!({
                    "fuel_type": "gasoline",
                    "transmission": "automatic",
                    "engine": "5.0L V8"
                })),
                location: Some(serde_json::json!({
                    "address": "456 Oak Ave, Chicago, IL",
                    "coordinates": {
                        "lat": 41.8700,
                        "lng": -87.6400
                    }
                })),
                images: Some(vec![
                    "ford_f150_front.jpg".to_string(),
                    "ford_f150_bed.jpg".to_string(),
                ]),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            Vehicle {
                id: Uuid::new_v4(),
                tenant_id,
                name: "Honda Civic 2021".to_string(),
                make: "Honda".to_string(),
                model: "Civic".to_string(),
                year: 2021,
                registration: "DEF-456".to_string(),
                category: "sedan".to_string(),
                status: "available".to_string(),
                daily_rate: 45.0,
                hourly_rate: Some(12.0),
                capacity: 5,
                features: Some(vec![
                    "Air Conditioning".to_string(),
                    "Bluetooth".to_string(),
                    "Lane Assist".to_string(),
                ]),
                specifications: Some(serde_json::json!({
                    "fuel_type": "gasoline",
                    "transmission": "automatic",
                    "engine": "1.5L Turbo"
                })),
                location: Some(serde_json::json!({
                    "address": "789 Pine St, Chicago, IL",
                    "coordinates": {
                        "lat": 41.8750,
                        "lng": -87.6350
                    }
                })),
                images: Some(vec![
                    "honda_civic_front.jpg".to_string(),
                    "honda_civic_interior.jpg".to_string(),
                ]),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
        ];
        
        // Filter by category if provided
        if let Some(cat) = category {
            vehicles.retain(|v| v.category == cat);
        }
        
        // Filter by status if provided
        if let Some(stat) = status {
            vehicles.retain(|v| v.status == stat);
        }
        
        Ok(vehicles)
    }
    
    pub async fn create_vehicle(&self, tenant_id: Uuid, req: &VehicleDto) -> Result<Vehicle, AppError> {
        // In a real implementation, this would insert into the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Validate inputs
        if req.name.is_empty() || req.make.is_empty() || req.model.is_empty() || 
           req.registration.is_empty() || req.category.is_empty() {
            return Err(AppError::ValidationError("Required fields cannot be empty".to_string()));
        }
        
        // Validate vehicle category
        if !["sedan", "suv", "truck", "van", "luxury", "sport", "other"].contains(&req.category.as_str()) {
            return Err(AppError::ValidationError(format!("Invalid vehicle category: {}", req.category)));
        }
        
        // Create vehicle
        let vehicle = Vehicle {
            id: req.id.unwrap_or_else(Uuid::new_v4),
            tenant_id,
            name: req.name.clone(),
            make: req.make.clone(),
            model: req.model.clone(),
            year: req.year,
            registration: req.registration.clone(),
            category: req.category.clone(),
            status: req.status.clone().unwrap_or_else(|| "available".to_string()),
            daily_rate: req.daily_rate,
            hourly_rate: req.hourly_rate,
            capacity: req.capacity,
            features: req.features.clone(),
            specifications: req.specifications.clone(),
            location: req.location.clone(),
            images: req.images.clone(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        Ok(vehicle)
    }
    
    pub async fn get_vehicle(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<Vehicle>, AppError> {
        // In a real implementation, this would query the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Mock vehicle for testing
        let vehicle = Vehicle {
            id,
            tenant_id,
            name: "Toyota Camry 2020".to_string(),
            make: "Toyota".to_string(),
            model: "Camry".to_string(),
            year: 2020,
            registration: "ABC-123".to_string(),
            category: "sedan".to_string(),
            status: "available".to_string(),
            daily_rate: 50.0,
            hourly_rate: Some(15.0),
            capacity: 5,
            features: Some(vec![
                "Air Conditioning".to_string(),
                "Bluetooth".to_string(),
                "Cruise Control".to_string(),
            ]),
            specifications: Some(serde_json::json!({
                "fuel_type": "gasoline",
                "transmission": "automatic",
                "engine": "2.5L"
            })),
            location: Some(serde_json::json!({
                "address": "123 Main St, Chicago, IL",
                "coordinates": {
                    "lat": 41.8781,
                    "lng": -87.6298
                }
            })),
            images: Some(vec![
                "toyota_camry_front.jpg".to_string(),
                "toyota_camry_interior.jpg".to_string(),
            ]),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        Ok(Some(vehicle))
    }
    
    pub async fn update_vehicle(&self, tenant_id: Uuid, id: Uuid, req: &VehicleDto) -> Result<Option<Vehicle>, AppError> {
        // In a real implementation, this would update the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Validate inputs
        if req.name.is_empty() || req.make.is_empty() || req.model.is_empty() || 
           req.registration.is_empty() || req.category.is_empty() {
            return Err(AppError::ValidationError("Required fields cannot be empty".to_string()));
        }
        
        // Validate vehicle category
        if !["sedan", "suv", "truck", "van", "luxury", "sport", "other"].contains(&req.category.as_str()) {
            return Err(AppError::ValidationError(format!("Invalid vehicle category: {}", req.category)));
        }
        
        // Check if vehicle exists
        // In a real implementation, this would query the database
        
        // Update vehicle
        let vehicle = Vehicle {
            id,
            tenant_id,
            name: req.name.clone(),
            make: req.make.clone(),
            model: req.model.clone(),
            year: req.year,
            registration: req.registration.clone(),
            category: req.category.clone(),
            status: req.status.clone().unwrap_or_else(|| "available".to_string()),
            daily_rate: req.daily_rate,
            hourly_rate: req.hourly_rate,
            capacity: req.capacity,
            features: req.features.clone(),
            specifications: req.specifications.clone(),
            location: req.location.clone(),
            images: req.images.clone(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        Ok(Some(vehicle))
    }
    
    pub async fn delete_vehicle(&self, tenant_id: Uuid, id: Uuid) -> Result<bool, AppError> {
        // In a real implementation, this would delete from the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Check if vehicle exists
        // In a real implementation, this would query the database
        
        // Delete vehicle
        
        Ok(true)
    }
    
    pub async fn get_vehicle_availability(
        &self,
        tenant_id: Uuid,
        vehicle_id: Uuid,
        start_date: chrono::DateTime<Utc>,
        end_date: chrono::DateTime<Utc>,
    ) -> Result<Vec<VehicleAvailability>, AppError> {
        // In a real implementation, this would query the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Validate dates
        if start_date >= end_date {
            return Err(AppError::ValidationError("Start date must be before end date".to_string()));
        }
        
        // Check if vehicle exists
        let vehicle = self.get_vehicle(tenant_id, vehicle_id).await?;
        
        if vehicle.is_none() {
            return Err(AppError::ValidationError(format!("Vehicle not found: {}", vehicle_id)));
        }
        
        // In a real implementation, this would check bookings and maintenance records
        // For now, we'll generate sample availability data
        
        let mut availability = Vec::new();
        let mut current_date = start_date.date_naive();
        let end_date_naive = end_date.date_naive();
        
        while current_date <= end_date_naive {
            // Randomly determine availability (for demo purposes)
            let available = current_date.day() % 3 != 0;
            
            availability.push(VehicleAvailability {
                vehicle_id,
                date: current_date,
                available,
                reason: if !available { 
                    Some("Booked".to_string()) 
                } else { 
                    None 
                },
                booking_id: if !available { 
                    Some(Uuid::new_v4()) 
                } else { 
                    None 
                },
            });
            
            current_date = current_date.succ_opt().unwrap_or(current_date);
        }
        
        Ok(availability)
    }
    
    pub async fn add_maintenance_record(
        &self,
        tenant_id: Uuid,
        vehicle_id: Uuid,
        req: &MaintenanceRecordDto,
    ) -> Result<MaintenanceRecord, AppError> {
        // In a real implementation, this would insert into the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Validate inputs
        if req.title.is_empty() || req.description.is_empty() || req.maintenance_type.is_empty() || 
           req.performed_by.is_empty() {
            return Err(AppError::ValidationError("Required fields cannot be empty".to_string()));
        }
        
        // Validate maintenance type
        if !["routine", "repair", "inspection", "other"].contains(&req.maintenance_type.as_str()) {
            return Err(AppError::ValidationError(format!("Invalid maintenance type: {}", req.maintenance_type)));
        }
        
        // Check if vehicle exists
        let vehicle = self.get_vehicle(tenant_id, vehicle_id).await?;
        
        if vehicle.is_none() {
            return Err(AppError::ValidationError(format!("Vehicle not found: {}", vehicle_id)));
        }
        
        // Create maintenance record
        let record = MaintenanceRecord {
            id: Uuid::new_v4(),
            tenant_id,
            vehicle_id,
            title: req.title.clone(),
            description: req.description.clone(),
            maintenance_type: req.maintenance_type.clone(),
            cost: req.cost,
            performed_by: req.performed_by.clone(),
            start_date: req.start_date,
            end_date: req.end_date,
            notes: req.notes.clone(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        // In a real implementation, this would also update the vehicle status
        // if the maintenance is ongoing
        
        Ok(record)
    }
    
    pub async fn list_maintenance_records(
        &self,
        tenant_id: Uuid,
        vehicle_id: Uuid,
    ) -> Result<Vec<MaintenanceRecord>, AppError> {
        // In a real implementation, this would query the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Check if vehicle exists
        let vehicle = self.get_vehicle(tenant_id, vehicle_id).await?;
        
        if vehicle.is_none() {
            return Err(AppError::ValidationError(format!("Vehicle not found: {}", vehicle_id)));
        }
        
        // Mock maintenance records for testing
        let records = vec![
            MaintenanceRecord {
                id: Uuid::new_v4(),
                tenant_id,
                vehicle_id,
                title: "Oil Change".to_string(),
                description: "Regular oil change and filter replacement".to_string(),
                maintenance_type: "routine".to_string(),
                cost: Some(50.0),
                performed_by: "John Mechanic".to_string(),
                start_date: Utc::now() - chrono::Duration::days(7),
                end_date: Some(Utc::now() - chrono::Duration::days(7)),
                notes: Some("Used synthetic oil".to_string()),
                created_at: Utc::now() - chrono::Duration::days(7),
                updated_at: Utc::now() - chrono::Duration::days(7),
            },
            MaintenanceRecord {
                id: Uuid::new_v4(),
                tenant_id,
                vehicle_id,
                title: "Brake Pad Replacement".to_string(),
                description: "Replaced front brake pads".to_string(),
                maintenance_type: "repair".to_string(),
                cost: Some(150.0),
                performed_by: "Auto Shop Inc.".to_string(),
                start_date: Utc::now() - chrono::Duration::days(30),
                end_date: Some(Utc::now() - chrono::Duration::days(30)),
                notes: None,
                created_at: Utc::now() - chrono::Duration::days(30),
                updated_at: Utc::now() - chrono::Duration::days(30),
            },
        ];
        
        Ok(records)
    }
}