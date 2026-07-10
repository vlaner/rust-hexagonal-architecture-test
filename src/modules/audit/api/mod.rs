use async_trait::async_trait;
use chrono::{DateTime, Utc};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AuditLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub action: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Error)]
pub enum AuditError {
    #[error("unexpected audit error")]
    Unknown(#[from] anyhow::Error),
}

#[async_trait]
pub trait AuditApi: Send + Sync {
    async fn log(&mut self, user_id: Uuid, action: &str) -> Result<AuditLog, AuditError>;
}
