use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tenant {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub status: TenantStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub config: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TenantStatus {
    Active,
    Suspended,
    Inactive,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenantContext {
    pub tenant_id: Uuid,
    pub tenant_slug: String,
    pub subscription_status: String,
    pub active_products: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTenantRequest {
    pub name: String,
    pub slug: String,
    pub admin_email: String,
    pub admin_password: String,
    pub admin_full_name: String,
    pub product_id: String,
    pub plan_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTenantRequest {
    pub name: Option<String>,
    pub status: Option<TenantStatus>,
    pub config: Option<serde_json::Value>,
}
