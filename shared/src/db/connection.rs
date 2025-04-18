use std::time::Duration;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use redis::{Client as RedisClient, Connection as RedisConn};
use crate::utils::error::AppError;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct DatabaseConnection {
    pool: PgPool,
}

impl DatabaseConnection {
    pub fn new(database_url: &str) -> Result<Self, AppError> {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .connection_timeout(Duration::from_secs(30))
            .test_on_check_out(true)
            .max_size(20)
            .build(manager)
            .map_err(|e| AppError::DatabaseError(format!("Failed to create pool: {}", e)))?;
            
        Ok(DatabaseConnection { pool })
    }
    
    pub fn get_connection(&self) -> Result<PgPooledConnection, AppError> {
        self.pool
            .get()
            .map_err(|e| AppError::DatabaseError(format!("Failed to get connection: {}", e)))
    }
    
    pub fn set_tenant_schema(&self, tenant_slug: &str) -> Result<(), AppError> {
        let conn = &mut self.get_connection()?;
        diesel::sql_query(format!("SET search_path TO tenant_{}, public", tenant_slug))
            .execute(conn)
            .map_err(|e| AppError::DatabaseError(format!("Failed to set search path: {}", e)))?;
            
        Ok(())
    }
    
    pub fn reset_schema(&self) -> Result<(), AppError> {
        let conn = &mut self.get_connection()?;
        diesel::sql_query("SET search_path TO public")
            .execute(conn)
            .map_err(|e| AppError::DatabaseError(format!("Failed to reset search path: {}", e)))?;
            
        Ok(())
    }
}

#[derive(Clone)]
pub struct RedisConnection {
    client: RedisClient,
}

impl RedisConnection {
    pub fn new(redis_url: &str) -> Result<Self, AppError> {
        let client = RedisClient::open(redis_url)
            .map_err(|e| AppError::DatabaseError(format!("Failed to connect to Redis: {}", e)))?;
            
        Ok(RedisConnection { client })
    }
    
    pub fn get_connection(&self) -> Result<RedisConn, AppError> {
        self.client
            .get_connection()
            .map_err(|e| AppError::DatabaseError(format!("Failed to get Redis connection: {}", e)))
    }
    
    pub fn set_value(&self, key: &str, value: &str, expiry_seconds: Option<usize>) -> Result<(), AppError> {
        let mut conn = self.get_connection()?;
        
        if let Some(expiry) = expiry_seconds {
            redis::cmd("SETEX")
                .arg(key)
                .arg(expiry)
                .arg(value)
                .execute(&mut conn);
        } else {
            redis::cmd("SET").arg(key).arg(value).execute(&mut conn);
        }
        
        Ok(())
    }
    
    pub fn get_value(&self, key: &str) -> Result<Option<String>, AppError> {
        let mut conn = self.get_connection()?;
        
        let value: Option<String> = redis::cmd("GET")
            .arg(key)
            .query(&mut conn)
            .map_err(|e| AppError::DatabaseError(format!("Failed to get Redis value: {}", e)))?;
            
        Ok(value)
    }
    
    pub fn delete_value(&self, key: &str) -> Result<(), AppError> {
        let mut conn = self.get_connection()?;
        
        redis::cmd("DEL")
            .arg(key)
            .execute(&mut conn);
            
        Ok(())
    }
}
