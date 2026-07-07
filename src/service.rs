use std::sync::Arc;
use uuid::Uuid;

use crate::domain::audit::AuditError;
use crate::domain::uow::{UnitOfWork, UoWError};
use crate::domain::users::{User, UserError};
use crate::error::AppError;

pub struct UserService {
    uow: Arc<dyn UnitOfWork>,
}

impl UserService {
    pub fn new(uow: Arc<dyn UnitOfWork>) -> Self {
        Self { uow }
    }

    pub async fn get_user(&self, uid: Uuid) -> Result<User, AppError> {
        let mut tx = self.uow.begin().await?;
        let user = tx.user().by_id(uid).await?;
        tx.audit().log(uid, "read_user").await?;
        tx.commit().await?;
        Ok(user)
    }
}

impl From<UserError> for AppError {
    fn from(e: UserError) -> Self {
        match e {
            UserError::NotFound => AppError::not_found(),
            UserError::DuplicateUsername => AppError::conflict(),
            UserError::Unknown(cause) => AppError::internal(cause),
        }
    }
}

impl From<AuditError> for AppError {
    fn from(e: AuditError) -> Self {
        match e {
            AuditError::Unknown(cause) => AppError::internal(cause),
        }
    }
}

impl From<UoWError> for AppError {
    fn from(e: UoWError) -> Self {
        match e {
            UoWError::Begin(cause) => AppError::internal(cause),
            UoWError::Commit(cause) => AppError::internal(cause),
            UoWError::Rollback(cause) => AppError::internal(cause),
        }
    }
}
