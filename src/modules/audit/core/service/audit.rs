use crate::modules::audit::api::{AuditApi, AuditError, AuditLog};
use async_trait::async_trait;
use uuid::Uuid;

use crate::modules::audit::core::domain::audit::AuditRepository;

pub struct AuditService<R> {
    repository: R,
}

impl<R> AuditService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R> AuditApi for AuditService<R>
where
    R: AuditRepository,
{
    async fn log(&mut self, user_id: Uuid, action: &str) -> Result<AuditLog, AuditError> {
        let log = self
            .repository
            .log(user_id, action)
            .await
            .map_err(AuditError::Unknown)?;

        Ok(AuditLog {
            id: log.id,
            user_id: log.user_id,
            action: log.action,
            timestamp: log.timestamp,
        })
    }
}
