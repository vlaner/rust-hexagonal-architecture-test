use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::shared::apperror::AppError;

#[derive(Debug, Clone)]
pub struct AuditLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub action: String,
    pub timestamp: DateTime<Utc>,
}

#[async_trait]
pub trait AuditApi: Send + Sync {
    async fn log(&mut self, user_id: Uuid, action: &str) -> Result<AuditLog, AppError>;
}
