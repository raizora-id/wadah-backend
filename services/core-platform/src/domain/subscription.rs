use chrono::{Duration, Utc};
use uuid::Uuid;

use shared::{
    DatabaseConnection,
    models::subscription::{
        CreateSubscriptionRequest, Plan, Product, ProductStatus, Subscription, 
        SubscriptionStatus, UpdateSubscriptionRequest
    },
    utils::error::AppError,
};

pub struct SubscriptionService {
    db_conn: DatabaseConnection,
}

impl SubscriptionService {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        Self { db_conn }
    }
    
    pub async fn list_subscriptions(&self, tenant_id: Uuid) -> Result<Vec<Subscription>, AppError> {
        // In a real implementation, this would query the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Mock subscriptions for testing
        let subscriptions = vec![
            Subscription {
                id: Uuid::new_v4(),
                tenant_id,
                plan_id: Uuid::new_v4(),
                status: SubscriptionStatus::Active,
                start_date: Utc::now(),
                end_date: Utc::now() + Duration::days(365),
                trial_end_date: Some(Utc::now() + Duration::days(30)),
                auto_renew: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                canceled_at: None,
            },
            Subscription {
                id: Uuid::new_v4(),
                tenant_id,
                plan_id: Uuid::new_v4(),
                status: SubscriptionStatus::Active,
                start_date: Utc::now(),
                end_date: Utc::now() + Duration::days(365),
                trial_end_date: None,
                auto_renew: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                canceled_at: None,
            },
        ];
        
        Ok(subscriptions)
    }
    
    pub async fn create_subscription(&self, req: &CreateSubscriptionRequest) -> Result<Subscription, AppError> {
        // In a real implementation, this would insert into the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Check if plan exists
        // In a real implementation, this would query the database
        
        // Calculate end date based on plan duration
        // In a real implementation, this would be based on the plan's duration
        let end_date = req.start_date + Duration::days(365);
        
        // Calculate trial end date if applicable
        let trial_end_date = req.trial_period_days.map(|days| {
            req.start_date + Duration::days(days as i64)
        });
        
        // Create subscription
        let subscription = Subscription {
            id: Uuid::new_v4(),
            tenant_id: req.tenant_id,
            plan_id: req.plan_id,
            status: if trial_end_date.is_some() {
                SubscriptionStatus::Trial
            } else {
                SubscriptionStatus::Active
            },
            start_date: req.start_date,
            end_date,
            trial_end_date,
            auto_renew: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            canceled_at: None,
        };
        
        Ok(subscription)
    }
    
    pub async fn get_subscription(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<Subscription>, AppError> {
        // In a real implementation, this would query the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Mock subscription for testing
        let subscription = Subscription {
            id,
            tenant_id,
            plan_id: Uuid::new_v4(),
            status: SubscriptionStatus::Active,
            start_date: Utc::now(),
            end_date: Utc::now() + Duration::days(365),
            trial_end_date: Some(Utc::now() + Duration::days(30)),
            auto_renew: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            canceled_at: None,
        };
        
        Ok(Some(subscription))
    }
    
    pub async fn update_subscription(&self, tenant_id: Uuid, id: Uuid, req: &UpdateSubscriptionRequest) -> Result<Option<Subscription>, AppError> {
        // In a real implementation, this would update the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Check if subscription exists
        // In a real implementation, this would query the database
        
        // Update subscription
        let subscription = Subscription {
            id,
            tenant_id,
            plan_id: Uuid::new_v4(),
            status: req.status.clone().unwrap_or(SubscriptionStatus::Active),
            start_date: Utc::now(),
            end_date: Utc::now() + Duration::days(365),
            trial_end_date: Some(Utc::now() + Duration::days(30)),
            auto_renew: req.auto_renew.unwrap_or(true),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            canceled_at: None,
        };
        
        Ok(Some(subscription))
    }
    
    pub async fn cancel_subscription(&self, tenant_id: Uuid, id: Uuid) -> Result<bool, AppError> {
        // In a real implementation, this would update the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Check if subscription exists
        // In a real implementation, this would query the database
        
        // Cancel subscription (mark as canceled and set canceled_at)
        
        Ok(true)
    }
    
    pub async fn list_products(&self) -> Result<Vec<Product>, AppError> {
        // In a real implementation, this would query the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Mock products for testing
        let products = vec![
            Product {
                id: "klolatoko".to_string(),
                name: "Klolatoko".to_string(),
                description: Some("Retail management system".to_string()),
                status: ProductStatus::Active,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            Product {
                id: "klolakos".to_string(),
                name: "Klolakos".to_string(),
                description: Some("Property management system".to_string()),
                status: ProductStatus::Active,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            Product {
                id: "klolarental".to_string(),
                name: "Klolarental".to_string(),
                description: Some("Rental management system".to_string()),
                status: ProductStatus::Active,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            Product {
                id: "klolaform".to_string(),
                name: "Klolaform".to_string(),
                description: Some("Form builder".to_string()),
                status: ProductStatus::Active,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
        ];
        
        Ok(products)
    }
    
    pub async fn list_product_plans(&self, product_id: &str) -> Result<Vec<Plan>, AppError> {
        // In a real implementation, this would query the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Mock plans for testing
        let plans = vec![
            Plan {
                id: Uuid::new_v4(),
                product_id: product_id.to_string(),
                name: "Basic".to_string(),
                description: Some("Basic plan".to_string()),
                duration_type: "month".to_string(),
                duration_value: 1,
                price: 10.0,
                currency: "IDR".to_string(),
                is_active: true,
                features: serde_json::json!({
                    "max_users": 5,
                    "max_storage_gb": 5,
                }),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            Plan {
                id: Uuid::new_v4(),
                product_id: product_id.to_string(),
                name: "Pro".to_string(),
                description: Some("Professional plan".to_string()),
                duration_type: "month".to_string(),
                duration_value: 1,
                price: 25.0,
                currency: "IDR".to_string(),
                is_active: true,
                features: serde_json::json!({
                    "max_users": 20,
                    "max_storage_gb": 20,
                }),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            Plan {
                id: Uuid::new_v4(),
                product_id: product_id.to_string(),
                name: "Enterprise".to_string(),
                description: Some("Enterprise plan".to_string()),
                duration_type: "month".to_string(),
                duration_value: 1,
                price: 50.0,
                currency: "IDR".to_string(),
                is_active: true,
                features: serde_json::json!({
                    "max_users": 100,
                    "max_storage_gb": 100,
                }),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
        ];
        
        Ok(plans)
    }
}