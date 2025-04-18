use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;

use crate::models::tenant::Tenant;
use crate::models::user::User;
use crate::utils::error::AppError;

// Helper functions for common database queries

pub fn get_tenant_by_id(conn: &mut PgConnection, tenant_id: Uuid) -> Result<Option<Tenant>, AppError> {
    // This would use diesel's ORM functionality
    // For simplicity, we're using raw SQL here
    let query = format!("SELECT * FROM meta.tenants WHERE id = '{}'", tenant_id);
    
    let results = diesel::sql_query(query)
        .load::<Tenant>(conn)
        .map_err(|e| AppError::DatabaseError(format!("Failed to get tenant: {}", e)))?;
        
    Ok(results.into_iter().next())
}

pub fn get_tenant_by_slug(conn: &mut PgConnection, slug: &str) -> Result<Option<Tenant>, AppError> {
    let query = format!("SELECT * FROM meta.tenants WHERE slug = '{}'", slug);
    
    let results = diesel::sql_query(query)
        .load::<Tenant>(conn)
        .map_err(|e| AppError::DatabaseError(format!("Failed to get tenant: {}", e)))?;
        
    Ok(results.into_iter().next())
}

pub fn get_user_by_id(conn: &mut PgConnection, user_id: Uuid) -> Result<Option<User>, AppError> {
    let query = format!("SELECT * FROM meta.users WHERE id = '{}'", user_id);
    
    let results = diesel::sql_query(query)
        .load::<User>(conn)
        .map_err(|e| AppError::DatabaseError(format!("Failed to get user: {}", e)))?;
        
    Ok(results.into_iter().next())
}

pub fn get_user_by_email(conn: &mut PgConnection, tenant_id: Uuid, email: &str) -> Result<Option<User>, AppError> {
    let query = format!(
        "SELECT * FROM meta.users WHERE tenant_id = '{}' AND email = '{}'",
        tenant_id, email
    );
    
    let results = diesel::sql_query(query)
        .load::<User>(conn)
        .map_err(|e| AppError::DatabaseError(format!("Failed to get user: {}", e)))?;
        
    Ok(results.into_iter().next())
}
