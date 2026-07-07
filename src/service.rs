use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::uow::{UnitOfWork, UoWError};
use crate::domain::users::{User, UserError};
use crate::domain::{
    audit::AuditError,
    uow::{HasAuditRepo, HasUserRepo, UnitOfWorkTransaction},
};
use crate::error::AppError;

#[async_trait]
pub trait UserServiceApi: Send + Sync {
    async fn get_user(&self, uid: Uuid) -> Result<User, AppError>;
}

#[async_trait]
impl<U> UserServiceApi for UserService<U>
where
    U: UnitOfWork + Send + Sync,
    U::Tx: HasUserRepo + HasAuditRepo,
{
    async fn get_user(&self, uid: Uuid) -> Result<User, AppError> {
        UserService::get_user(self, uid).await
    }
}

pub struct UserService<U: UnitOfWork>
where
    U::Tx: HasUserRepo + HasAuditRepo,
{
    uow: Arc<U>,
}

impl<U: UnitOfWork> UserService<U>
where
    U::Tx: HasUserRepo + HasAuditRepo,
{
    pub fn new(uow: Arc<U>) -> Self {
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
