// This file contains product-specific models that are shared across services

use serde::{Deserialize, Serialize};

// Klolatoko models (retail management)
#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub sku: String,
    pub barcode: Option<String>,
    pub description: Option<String>,
    pub price: f64,
    pub stock: i32,
    pub category: Option<String>,
    pub image_url: Option<String>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sale {
    pub id: String,
    pub date: String,
    pub customer_name: Option<String>,
    pub items: Vec<SaleItem>,
    pub total: f64,
    pub payment_method: String,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaleItem {
    pub product_id: String,
    pub quantity: i32,
    pub price: f64,
    pub discount: Option<f64>,
    pub subtotal: f64,
}

// Klolakos models (property management)
#[derive(Debug, Serialize, Deserialize)]
pub struct Property {
    pub id: String,
    pub name: String,
    pub address: String,
    pub type_: String,
    pub units: i32,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyUnit {
    pub id: String,
    pub property_id: String,
    pub unit_number: String,
    pub floor: Option<i32>,
    pub size: Option<f64>,
    pub bedrooms: Option<i32>,
    pub bathrooms: Option<i32>,
    pub status: String,
    pub rent_amount: Option<f64>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tenant {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub unit_id: String,
    pub lease_start: Option<String>,
    pub lease_end: Option<String>,
    pub rent_amount: Option<f64>,
    pub deposit_amount: Option<f64>,
    pub status: String,
    pub metadata: serde_json::Value,
}

// Klolarental models (rental management)
#[derive(Debug, Serialize, Deserialize)]
pub struct Vehicle {
    pub id: String,
    pub name: String,
    pub type_: String,
    pub license_plate: String,
    pub make: String,
    pub model: String,
    pub year: i32,
    pub color: Option<String>,
    pub daily_rate: f64,
    pub status: String,
    pub image_url: Option<String>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Booking {
    pub id: String,
    pub vehicle_id: String,
    pub customer_name: String,
    pub customer_email: Option<String>,
    pub customer_phone: String,
    pub start_date: String,
    pub end_date: String,
    pub total_amount: f64,
    pub deposit_amount: Option<f64>,
    pub status: String,
    pub notes: Option<String>,
}

// Klolaform models (form builder)
#[derive(Debug, Serialize, Deserialize)]
pub struct Form {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub fields: Vec<FormField>,
    pub status: String,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormField {
    pub id: String,
    pub label: String,
    pub type_: String,
    pub required: bool,
    pub options: Option<Vec<String>>,
    pub placeholder: Option<String>,
    pub default_value: Option<String>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormResponse {
    pub id: String,
    pub form_id: String,
    pub responses: serde_json::Value,
    pub submitted_at: String,
    pub metadata: serde_json::Value,
}
