use crate::shared::apperror::AppError;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub created_at: DateTime<Utc>,
}

#[async_trait]
pub trait UserServiceApi: Send + Sync {
    async fn get_user(&self, uid: Uuid) -> Result<User, AppError>;
}
