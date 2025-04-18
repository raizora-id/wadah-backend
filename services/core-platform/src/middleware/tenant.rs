use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use uuid::Uuid;

use shared::{DatabaseConnection, models::tenant::TenantContext};
use shared::utils::error::AppError;

pub struct TenantMiddleware {
    db_conn: DatabaseConnection,
}

impl TenantMiddleware {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        Self { db_conn }
    }
    
    // Extract tenant from request (subdomain, header, or JWT)
    async fn extract_tenant(&self, req: &ServiceRequest) -> Result<TenantContext, AppError> {
        // First, try to get tenant_id from request extensions (set by AuthMiddleware)
        if let Some(tenant_id) = req.extensions().get::<Uuid>() {
            return self.get_tenant_context(*tenant_id).await;
        }
        
        // If not found in extensions, try to get from header
        if let Some(tenant_header) = req.headers().get("X-Tenant-ID") {
            if let Ok(tenant_id_str) = tenant_header.to_str() {
                if let Ok(tenant_id) = Uuid::parse_str(tenant_id_str) {
                    return self.get_tenant_context(tenant_id).await;
                }
            }
        }
        
        // If still not found, try to extract from subdomain
        if let Some(host) = req.headers().get("Host") {
            if let Ok(host_str) = host.to_str() {
                let parts: Vec<&str> = host_str.split('.').collect();
                if parts.len() > 2 {
                    let subdomain = parts[0];
                    return self.get_tenant_context_by_slug(subdomain).await;
                }
            }
        }
        
        // If tenant cannot be determined, return error
        Err(AppError::TenantError("Tenant not found".to_string()))
    }
    
    // Get tenant context by ID
    async fn get_tenant_context(&self, tenant_id: Uuid) -> Result<TenantContext, AppError> {
        // In a real implementation, this would query the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // This would be a proper database query in a real implementation
        let tenant_slug = "acme"; // Mock - would come from database
        let subscription_status = "active"; // Mock - would come from database
        let active_products = vec!["klolatoko".to_string(), "klolaform".to_string()]; // Mock - would come from database
        
        Ok(TenantContext {
            tenant_id,
            tenant_slug: tenant_slug.to_string(),
            subscription_status: subscription_status.to_string(),
            active_products,
        })
    }
    
    // Get tenant context by slug
    async fn get_tenant_context_by_slug(&self, slug: &str) -> Result<TenantContext, AppError> {
        // In a real implementation, this would query the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // This would be a proper database query in a real implementation
        let tenant_id = Uuid::new_v4(); // Mock - would come from database
        let subscription_status = "active"; // Mock - would come from database
        let active_products = vec!["klolatoko".to_string(), "klolaform".to_string()]; // Mock - would come from database
        
        Ok(TenantContext {
            tenant_id,
            tenant_slug: slug.to_string(),
            subscription_status: subscription_status.to_string(),
            active_products,
        })
    }
    
    // Set database search path for the request
    async fn set_tenant_schema(&self, tenant_slug: &str) -> Result<(), AppError> {
        self.db_conn.set_tenant_schema(tenant_slug)
    }
}

impl<S, B> Transform<S, ServiceRequest> for TenantMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = TenantMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TenantMiddlewareService {
            service,
            db_conn: self.db_conn.clone(),
        }))
    }
}

pub struct TenantMiddlewareService<S> {
    service: S,
    db_conn: DatabaseConnection,
}

impl<S, B> Service<ServiceRequest> for TenantMiddlewareService<S>
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
        // Skip tenant middleware for public routes
        let path = req.path().to_string();
        
        if path == "/api/v1/auth/login" || path == "/api/v1/auth/register" {
            return Box::pin(self.service.call(req));
        }
        
        let middleware = TenantMiddleware::new(self.db_conn.clone());
        let fut = self.service.call(req);
        
        Box::pin(async move {
            // Extract tenant context from the request
            let tenant_context_result = middleware.extract_tenant(&req).await;
            
            if let Err(e) = tenant_context_result {
                return Err(actix_web::error::ErrorUnauthorized(e.to_string()));
            }
            
            let tenant_context = tenant_context_result.unwrap();
            
            // Set the search path for the tenant
            let schema_result = middleware.set_tenant_schema(&tenant_context.tenant_slug).await;
            
            if let Err(e) = schema_result {
                return Err(actix_web::error::ErrorInternalServerError(e.to_string()));
            }
            
            // Add tenant context to request extensions
            req.extensions_mut().insert(tenant_context);
            
            // Call the next middleware in the chain
            let res = fut.await?;
            
            // Reset schema path after request is processed
            let _ = middleware.db_conn.reset_schema();
            
            Ok(res)
        })
    }
}