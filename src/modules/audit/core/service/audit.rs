use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    modules::audit::{
        api::{AuditApi, AuditLog as AuditApiLog},
        core::domain::audit::{AuditError, AuditLog, AuditRepository},
    },
    shared::apperror::AppError,
};

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
    async fn log(&mut self, user_id: Uuid, action: &str) -> Result<AuditApiLog, AppError> {
        Ok(self.repository.log(user_id, action).await?.into())
    }
}

impl From<AuditLog> for AuditApiLog {
    fn from(log: AuditLog) -> Self {
        Self {
            id: log.id,
            user_id: log.user_id,
            action: log.action,
            timestamp: log.timestamp,
        }
    }
}

impl From<AuditError> for AppError {
    fn from(error: AuditError) -> Self {
        match error {
            AuditError::Unknown(cause) => AppError::internal(cause),
        }
    }
}
