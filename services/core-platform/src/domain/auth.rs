use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use uuid::Uuid;

use shared::{
    DatabaseConnection, RedisConnection,
    models::user::{AuthTokens, RegisterUserRequest, TokenClaims, User, UserStatus},
    utils::{
        error::AppError,
        validation::validate_email
    }
};

pub struct AuthService {
    db_conn: DatabaseConnection,
    redis_conn: RedisConnection,
    jwt_secret: String,
    jwt_expiry: i64,
}

impl AuthService {
    pub fn new(
        db_conn: DatabaseConnection,
        redis_conn: RedisConnection,
        jwt_secret: String,
        jwt_expiry: i64,
    ) -> Self {
        Self {
            db_conn,
            redis_conn,
            jwt_secret,
            jwt_expiry,
        }
    }
    
    pub async fn login(&self, email: &str, password: &str) -> Result<AuthTokens, AppError> {
        // In a real implementation, this would query the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Validate inputs
        if email.is_empty() || password.is_empty() {
            return Err(AppError::ValidationError("Email and password are required".to_string()));
        }
        
        // Validate email format
        validate_email(email)?;
        
        // Query for user with matching email
        // In a real implementation, this would be a database query
        // For now, we'll mock a user for testing
        
        // Mock user for testing
        let user_id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        
        // In a real implementation, we would verify password against stored hash
        // For now, we'll just accept any password
        
        // Generate tokens
        let tokens = self.generate_tokens(user_id, tenant_id)?;
        
        // Store refresh token in Redis
        self.redis_conn.set_value(
            &format!("refresh_token:{}", tokens.refresh_token),
            &user_id.to_string(),
            Some(30 * 24 * 60 * 60), // 30 days
        )?;
        
        Ok(tokens)
    }
    
    pub async fn register(&self, req: &RegisterUserRequest) -> Result<User, AppError> {
        // In a real implementation, this would insert into the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Validate inputs
        if req.email.is_empty() || req.password.is_empty() || req.full_name.is_empty() {
            return Err(AppError::ValidationError("All fields are required".to_string()));
        }
        
        // Validate email format
        validate_email(&req.email)?;
        
        // Check if email is already in use
        // In a real implementation, this would query the database
        
        // Hash password
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(req.password.as_bytes(), &salt)
            .map_err(|e| AppError::InternalError(format!("Password hashing error: {}", e)))?
            .to_string();
        
        // Create user
        // In a real implementation, this would insert into the database
        let user = User {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(), // In a real implementation, this would come from the request or context
            email: req.email.clone(),
            password_hash,
            full_name: req.full_name.clone(),
            status: UserStatus::Active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_login: None,
        };
        
        Ok(user)
    }
    
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<AuthTokens, AppError> {
        // In a real implementation, this would verify the refresh token
        // For now, we'll mock the response
        
        // Get user ID from Redis using refresh token
        let user_id_str = self.redis_conn.get_value(&format!("refresh_token:{}", refresh_token))?
            .ok_or_else(|| AppError::AuthenticationError("Invalid refresh token".to_string()))?;
        
        let user_id = Uuid::parse_str(&user_id_str)
            .map_err(|_| AppError::AuthenticationError("Invalid user ID".to_string()))?;
        
        // In a real implementation, we would get the tenant ID from the database
        let tenant_id = Uuid::new_v4();
        
        // Generate new tokens
        let tokens = self.generate_tokens(user_id, tenant_id)?;
        
        // Invalidate old refresh token
        self.redis_conn.delete_value(&format!("refresh_token:{}", refresh_token))?;
        
        // Store new refresh token
        self.redis_conn.set_value(
            &format!("refresh_token:{}", tokens.refresh_token),
            &user_id.to_string(),
            Some(30 * 24 * 60 * 60), // 30 days
        )?;
        
        Ok(tokens)
    }
    
    pub async fn get_current_user(&self, user_id: Uuid) -> Result<User, AppError> {
        // In a real implementation, this would query the database
        // For now, we'll mock the response
        let conn = &mut self.db_conn.get_connection()?;
        
        // Query for user with matching ID
        // In a real implementation, this would be a database query
        // For now, we'll mock a user for testing
        
        let user = User {
            id: user_id,
            tenant_id: Uuid::new_v4(),
            email: "user@example.com".to_string(),
            password_hash: "hashed_password".to_string(),
            full_name: "Test User".to_string(),
            status: UserStatus::Active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_login: Some(Utc::now()),
        };
        
        Ok(user)
    }
    
    fn generate_tokens(&self, user_id: Uuid, tenant_id: Uuid) -> Result<AuthTokens, AppError> {
        let now = Utc::now();
        let access_token_expiry = now
            .checked_add_signed(Duration::seconds(self.jwt_expiry))
            .unwrap_or(now)
            .timestamp();
        
        let refresh_token_expiry = now
            .checked_add_signed(Duration::days(30))
            .unwrap_or(now)
            .timestamp();
        
        // Create access token claims
        let access_claims = TokenClaims {
            sub: user_id.to_string(),
            tenant_id,
            exp: access_token_expiry,
            iat: now.timestamp(),
        };
        
        // Create refresh token claims
        let refresh_claims = TokenClaims {
            sub: user_id.to_string(),
            tenant_id,
            exp: refresh_token_expiry,
            iat: now.timestamp(),
        };
        
        // Generate tokens
        let access_token = encode(
            &Header::default(),
            &access_claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(|e| AppError::InternalError(format!("Token generation error: {}", e)))?;
        
        let refresh_token = encode(
            &Header::default(),
            &refresh_claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(|e| AppError::InternalError(format!("Token generation error: {}", e)))?;
        
        Ok(AuthTokens {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: self.jwt_expiry,
        })
    }
}