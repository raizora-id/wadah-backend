use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub status: ProductStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ProductStatus {
    Active,
    Inactive,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Plan {
    pub id: Uuid,
    pub product_id: String,
    pub name: String,
    pub description: Option<String>,
    pub duration_type: String,
    pub duration_value: i32,
    pub price: f64,
    pub currency: String,
    pub is_active: bool,
    pub features: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Subscription {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub plan_id: Uuid,
    pub status: SubscriptionStatus,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub trial_end_date: Option<DateTime<Utc>>,
    pub auto_renew: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub canceled_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SubscriptionStatus {
    Active,
    Inactive,
    Canceled,
    Expired,
    PendingPayment,
    Trial,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSubscriptionRequest {
    pub tenant_id: Uuid,
    pub plan_id: Uuid,
    pub start_date: DateTime<Utc>,
    pub trial_period_days: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSubscriptionRequest {
    pub status: Option<SubscriptionStatus>,
    pub auto_renew: Option<bool>,
}
