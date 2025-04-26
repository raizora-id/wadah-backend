use async_trait::async_trait;
use domain::{entities::User, repositories::UserRepository};
use sqlx::PgPool;
use uuid::Uuid;

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: &User) -> Result<User, Box<dyn std::error::Error>> {
        let result = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, email, username, password_hash, full_name, avatar_url, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#,
            user.id,
            user.email,
            user.username,
            user.password_hash,
            user.full_name,
            user.avatar_url,
            user.is_active,
            user.created_at,
            user.updated_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let result = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let result = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE email = $1",
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let result = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE username = $1",
            username
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn update(&self, user: &User) -> Result<User, Box<dyn std::error::Error>> {
        let result = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET email = $1, username = $2, password_hash = $3, full_name = $4,
                avatar_url = $5, is_active = $6
            WHERE id = $7
            RETURNING *
            "#,
            user.email,
            user.username,
            user.password_hash,
            user.full_name,
            user.avatar_url,
            user.is_active,
            user.id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn delete(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
