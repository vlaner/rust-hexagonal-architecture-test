use std::sync::Arc;

use crate::{
    modules::auth::api::{AuditPort, HasAuditPort, User, UserServiceApi},
    shared::{
        apperror::AppError,
        uow::{UnitOfWork, UnitOfWorkCallback, UnitOfWorkTransaction},
    },
};
use async_trait::async_trait;
use uuid::Uuid;

use crate::modules::auth::core::{
    domain::user::{User as DomainUser, UserRepository, error::UserError},
    uow::types::HasUserRepo,
};

pub(crate) struct UserService<U: UnitOfWork>
where
    U::Tx: HasUserRepo + HasAuditPort,
{
    uow: Arc<U>,
}

impl<U: UnitOfWork> UserService<U>
where
    U::Tx: HasUserRepo + HasAuditPort,
{
    pub(crate) fn new(uow: Arc<U>) -> Self {
        Self { uow }
    }
}

#[async_trait]
impl<U> UserServiceApi for UserService<U>
where
    U: UnitOfWork + Send + Sync,
    U::Tx: HasUserRepo + HasAuditPort,
{
    async fn get_user(&self, uid: Uuid) -> Result<User, AppError> {
        let mut tx = self.uow.begin().await?;
        let user = tx.user().by_id(uid).await?;
        tx.audit().log(uid, "read_user").await?;
        tx.commit().await?;

        Ok(user.into())
    }
}

pub(crate) struct UserServiceCallback<U: UnitOfWorkCallback>
where
    U::Tx: HasUserRepo + HasAuditPort,
{
    uow: Arc<U>,
}

impl<U: UnitOfWorkCallback> UserServiceCallback<U>
where
    U::Tx: HasUserRepo + HasAuditPort,
{
    pub(crate) fn new(uow: Arc<U>) -> Self {
        Self { uow }
    }
}

#[async_trait]
impl<U> UserServiceApi for UserServiceCallback<U>
where
    U: UnitOfWorkCallback + Send + Sync,
    U::Tx: HasUserRepo + HasAuditPort,
{
    async fn get_user(&self, uid: Uuid) -> Result<User, AppError> {
        self.uow
            .execute(|tx| {
                Box::pin(async move {
                    let user = tx.user().by_id(uid).await?;
                    tx.audit().log(uid, "read_user").await?;
                    Ok(user.into())
                })
            })
            .await
    }
}

impl From<UserError> for AppError {
    fn from(error: UserError) -> Self {
        match error {
            UserError::NotFound => AppError::not_found(),
            UserError::DuplicateUsername => AppError::conflict(),
            UserError::Unknown(cause) => AppError::internal(cause),
        }
    }
}

impl From<DomainUser> for User {
    fn from(user: DomainUser) -> Self {
        Self {
            id: user.id,
            username: user.username,
            created_at: user.created_at,
        }
    }
}
