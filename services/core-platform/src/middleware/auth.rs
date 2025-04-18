use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use uuid::Uuid;

use shared::{RedisConnection, models::user::TokenClaims};
use shared::utils::error::AppError;

pub struct AuthMiddleware {
    jwt_secret: String,
    redis_conn: RedisConnection,
}

impl AuthMiddleware {
    pub fn new(jwt_secret: String, redis_conn: RedisConnection) -> Self {
        Self { jwt_secret, redis_conn }
    }
    
    fn extract_token(&self, req: &ServiceRequest) -> Result<String, AppError> {
        let auth_header = req
            .headers()
            .get("Authorization")
            .ok_or_else(|| AppError::AuthenticationError("Missing authorization header".to_string()))?
            .to_str()
            .map_err(|_| AppError::AuthenticationError("Invalid authorization header".to_string()))?;
            
        if !auth_header.starts_with("Bearer ") {
            return Err(AppError::AuthenticationError("Invalid authorization header format".to_string()));
        }
        
        let token = auth_header[7..].to_string();
        Ok(token)
    }
    
    fn validate_token(&self, token: &str) -> Result<TokenClaims, AppError> {
        let validation = Validation::new(Algorithm::HS256);
        
        let claims = decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &validation,
        )
        .map_err(|e| AppError::AuthenticationError(format!("Invalid token: {}", e)))?
        .claims;
        
        // Check if token is blacklisted
        let blacklist_key = format!("token:blacklist:{}", token);
        let is_blacklisted = self.redis_conn.get_value(&blacklist_key)
            .map_err(|e| AppError::AuthenticationError(format!("Error checking token blacklist: {}", e)))?
            .is_some();
            
        if is_blacklisted {
            return Err(AppError::AuthenticationError("Token is blacklisted".to_string()));
        }
        
        Ok(claims)
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
            jwt_secret: self.jwt_secret.clone(),
            redis_conn: self.redis_conn.clone(),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
    jwt_secret: String,
    redis_conn: RedisConnection,
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
        // Skip auth for login and register
        let path = req.path().to_string();
        
        if path == "/api/v1/auth/login" || path == "/api/v1/auth/register" {
            return Box::pin(self.service.call(req));
        }
        
        let middleware = AuthMiddleware::new(self.jwt_secret.clone(), self.redis_conn.clone());
        
        // Extract token from request
        let token_result = middleware.extract_token(&req);
        
        if let Err(e) = token_result {
            return Box::pin(async {
                Err(actix_web::error::ErrorUnauthorized(e.to_string()))
            });
        }
        
        let token = token_result.unwrap();
        
        // Validate token
        let claims_result = middleware.validate_token(&token);
        
        if let Err(e) = claims_result {
            return Box::pin(async {
                Err(actix_web::error::ErrorUnauthorized(e.to_string()))
            });
        }
        
        let claims = claims_result.unwrap();
        
        // Extract user_id and tenant_id from claims
        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid user ID in token"));
            
        if let Err(e) = user_id {
            return Box::pin(async {
                Err(e)
            });
        }
        
        let user_id = user_id.unwrap();
        let tenant_id = claims.tenant_id;
        
        // Add user_id and tenant_id to request extensions
        req.extensions_mut().insert(user_id);
        req.extensions_mut().insert(tenant_id);
        
        let fut = self.service.call(req);
        
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}