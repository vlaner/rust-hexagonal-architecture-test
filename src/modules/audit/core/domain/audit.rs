use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub(crate) struct AuditLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub action: String,
    pub timestamp: DateTime<Utc>,
}

#[async_trait]
pub(crate) trait AuditRepository: Send + Sync {
    async fn log(&mut self, user_id: Uuid, action: &str) -> Result<AuditLog, anyhow::Error>;
}
