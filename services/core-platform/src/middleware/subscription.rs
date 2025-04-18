use std::future::{ready, Ready};
use std::collections::HashMap;

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use serde_json::Value;
use uuid::Uuid;

use shared::{DatabaseConnection, models::tenant::TenantContext};
use shared::utils::error::AppError;

pub struct SubscriptionMiddleware {
    db_conn: DatabaseConnection,
}

impl SubscriptionMiddleware {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        Self { db_conn }
    }
    
    // Check if tenant has access to requested product
    async fn check_product_access(&self, tenant_id: &Uuid, product_id: &str) -> Result<bool, AppError> {
        // In a real implementation, this would query the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // This would be a proper database query in a real implementation
        // For example, checking if the tenant has an active subscription for the product
        
        // Return true for now (access granted)
        Ok(true)
    }
    
    // Get feature limitations for current subscription
    async fn get_limitations(&self, tenant_id: &Uuid, product_id: &str) -> Result<HashMap<String, Value>, AppError> {
        // In a real implementation, this would query the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // This would be a proper database query in a real implementation
        // It would return limitations like max users, storage limits, etc.
        
        let mut limitations = HashMap::new();
        limitations.insert("max_users".to_string(), Value::Number(serde_json::Number::from(10)));
        limitations.insert("max_storage_gb".to_string(), Value::Number(serde_json::Number::from(5)));
        limitations.insert("max_workflows".to_string(), Value::Number(serde_json::Number::from(20)));
        
        Ok(limitations)
    }
    
    // Extract product ID from path
    fn extract_product_id(&self, path: &str) -> Option<String> {
        // In a real implementation, this would parse the URL path to extract the product ID
        // For example, for a path like /api/v1/klolatoko/products, the product ID would be "klolatoko"
        
        let parts: Vec<&str> = path.split('/').collect();
        
        if parts.len() >= 3 && parts[1] == "api" && parts[2] == "v1" {
            if parts.len() >= 4 {
                match parts[3] {
                    "klolatoko" | "klolakos" | "klolarental" | "klolaform" => return Some(parts[3].to_string()),
                    _ => {}
                }
            }
        }
        
        None
    }
}

impl<S, B> Transform<S, ServiceRequest> for SubscriptionMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = SubscriptionMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SubscriptionMiddlewareService {
            service,
            db_conn: self.db_conn.clone(),
        }))
    }
}

pub struct SubscriptionMiddlewareService<S> {
    service: S,
    db_conn: DatabaseConnection,
}

impl<S, B> Service<ServiceRequest> for SubscriptionMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Skip subscription checks for public routes and core platform routes
        let path = req.path().to_string();
        
        if path.starts_with("/api/v1/auth") || path.starts_with("/api/v1/tenants") || 
           path.starts_with("/api/v1/subscriptions") {
            return Box::pin(self.service.call(req));
        }
        
        let middleware = SubscriptionMiddleware::new(self.db_conn.clone());
        
        // Extract product ID from path
        let product_id = middleware.extract_product_id(&path);
        
        if product_id.is_none() {
            // No product-specific route, continue
            return Box::pin(self.service.call(req));
        }
        
        let product_id = product_id.unwrap();
        
        // Get tenant context from request extensions
        let tenant_context = req.extensions().get::<TenantContext>().cloned();
        
        if tenant_context.is_none() {
            // Tenant context not found, continue (this should not happen if tenant middleware is properly set up)
            return Box::pin(self.service.call(req));
        }
        
        let tenant_context = tenant_context.unwrap();
        
        let fut = self.service.call(req);
        
        Box::pin(async move {
            // Check if tenant has access to the requested product
            let access_result = middleware.check_product_access(&tenant_context.tenant_id, &product_id).await;
            
            if let Err(e) = access_result {
                return Err(actix_web::error::ErrorInternalServerError(e.to_string()));
            }
            
            let has_access = access_result.unwrap();
            
            if !has_access {
                return Err(actix_web::error::ErrorForbidden(
                    format!("Tenant does not have access to product: {}", product_id)
                ));
            }
            
            // Get limitations for the subscription
            let limitations_result = middleware.get_limitations(&tenant_context.tenant_id, &product_id).await;
            
            if let Err(e) = limitations_result {
                return Err(actix_web::error::ErrorInternalServerError(e.to_string()));
            }
            
            let limitations = limitations_result.unwrap();
            
            // Add limitations to request extensions
            req.extensions_mut().insert(limitations);
            
            // Call the next middleware in the chain
            let res = fut.await?;
            
            Ok(res)
        })
    }
}