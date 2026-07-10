use crate::shared::apperror::AppError;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct AuditPortError(pub anyhow::Error);

impl From<AuditPortError> for AppError {
    fn from(error: AuditPortError) -> Self {
        AppError::internal(error.0)
    }
}

#[async_trait]
pub trait AuditPort: Send + Sync {
    async fn log(&mut self, user_id: Uuid, action: &str) -> Result<(), AuditPortError>;
}

pub trait HasAuditPort {
    fn audit(&mut self) -> impl AuditPort + '_;
}

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
