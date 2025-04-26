use async_trait::async_trait;
use uuid::Uuid;

use crate::entities::User;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> Result<User, Box<dyn std::error::Error>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, Box<dyn std::error::Error>>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Box<dyn std::error::Error>>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, Box<dyn std::error::Error>>;
    async fn update(&self, user: &User) -> Result<User, Box<dyn std::error::Error>>;
    async fn delete(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error>>;
}
