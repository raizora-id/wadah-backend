use std::future::{ready, Ready};
use std::sync::Arc;

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use awc::Client;
use futures_util::future::LocalBoxFuture;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::server::AppState;
use shared::utils::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
struct AuthCheckResponse {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub tenant_slug: String,
    pub products: Vec<String>,
}

pub struct AuthMiddleware {
    app_state: Arc<AppState>,
}

impl AuthMiddleware {
    pub fn new(app_state: Arc<AppState>) -> Self {
        Self { app_state }
    }
    
    async fn verify_auth(&self, token: &str) -> Result<AuthCheckResponse, AppError> {
        // Call core platform auth verification endpoint
        let core_url = format!("{}/api/v1/auth/verify", self.app_state.config.core_platform_url);
        
        let client = Client::default();
        
        let response = client
            .get(core_url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| AppError::AuthenticationError(format!("Failed to verify token: {}", e)))?;
            
        if !response.status().is_success() {
            return Err(AppError::AuthenticationError("Invalid token".to_string()));
        }
        
        let body: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::AuthenticationError(format!("Failed to parse response: {}", e)))?;
            
        if !body.get("success").and_then(|v| v.as_bool()).unwrap_or(false) {
            return Err(AppError::AuthenticationError("Authentication failed".to_string()));
        }
        
        let auth_data: AuthCheckResponse = serde_json::from_value(body.get("data").cloned().unwrap_or_default())
            .map_err(|e| AppError::AuthenticationError(format!("Failed to parse auth data: {}", e)))?;
            
        Ok(auth_data)
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service,
            app_state: self.app_state.clone(),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
    app_state: Arc<AppState>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
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
        // Extract token from request
        let auth_header = req.headers().get("Authorization");
        
        if auth_header.is_none() {
            return Box::pin(async {
                Err(actix_web::error::ErrorUnauthorized("Missing authorization header"))
            });
        }
        
        let auth_header = auth_header.unwrap().to_str();
        
        if auth_header.is_err() {
            return Box::pin(async {
                Err(actix_web::error::ErrorUnauthorized("Invalid authorization header"))
            });
        }
        
        let auth_header = auth_header.unwrap();
        
        if !auth_header.starts_with("Bearer ") {
            return Box::pin(async {
                Err(actix_web::error::ErrorUnauthorized("Invalid authorization header format"))
            });
        }
        
        let token = &auth_header[7..];
        
        let app_state = self.app_state.clone();
        let middleware = AuthMiddleware::new(app_state);
        
        let fut = self.service.call(req);
        
        Box::pin(async move {
            // Verify token with core platform
            let auth_result = middleware.verify_auth(token).await;
            
            if let Err(e) = auth_result {
                return Err(actix_web::error::ErrorUnauthorized(e.to_string()));
            }
            
            let auth_data = auth_result.unwrap();
            
            // Add user_id and tenant_id to request extensions
            req.extensions_mut().insert(auth_data.user_id);
            req.extensions_mut().insert(auth_data.tenant_id);
            req.extensions_mut().insert(auth_data.products);
            
            // Set tenant schema in database connection
            let db_result = middleware.app_state.db_conn.set_tenant_schema(&auth_data.tenant_slug);
            
            if let Err(e) = db_result {
                return Err(actix_web::error::ErrorInternalServerError(e.to_string()));
            }
            
            // Call the next middleware in the chain
            let res = fut.await?;
            
            // Reset schema path after request is processed
            let _ = middleware.app_state.db_conn.reset_schema();
            
            Ok(res)
        })
    }
}