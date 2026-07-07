use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::uow::{UnitOfWork, UnitOfWorkCallback, UoWError};
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

pub struct UserServiceCallback<U: UnitOfWorkCallback>
where
    U::Tx: HasUserRepo + HasAuditRepo,
{
    uow: Arc<U>,
}

#[async_trait]
impl<U> UserServiceApi for UserServiceCallback<U>
where
    U: UnitOfWorkCallback + Send + Sync,
    U::Tx: HasUserRepo + HasAuditRepo,
{
    async fn get_user(&self, uid: Uuid) -> Result<User, AppError> {
        UserServiceCallback::get_user(self, uid).await
    }
}

impl<U: UnitOfWorkCallback> UserServiceCallback<U>
where
    U::Tx: HasUserRepo + HasAuditRepo,
{
    pub fn new(uow: Arc<U>) -> Self {
        Self { uow }
    }

    pub async fn get_user(&self, uid: Uuid) -> Result<User, AppError> {
        self.uow
            .execute(|repos| {
                Box::pin(async move {
                    let user = repos.user().by_id(uid).await?;
                    repos.audit().log(uid, "read_user").await?;
                    Ok(user)
                })
            })
            .await
            .map_err(|e| AppError::internal(e.into()))
    }
}
impl From<UserError> for UoWError {
    fn from(err: UserError) -> Self {
        Self::Commit(err.into())
    }
}
impl From<AuditError> for UoWError {
    fn from(err: AuditError) -> Self {
        Self::Commit(err.into())
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
