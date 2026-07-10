use async_trait::async_trait;
use uuid::Uuid;

use crate::shared::apperror::AppError;

#[async_trait]
pub trait AuditPort: Send + Sync {
    async fn log(&mut self, user_id: Uuid, action: &str) -> Result<(), AppError>;
}

pub trait HasAuditPort {
    fn audit(&mut self) -> impl AuditPort + '_;
}
