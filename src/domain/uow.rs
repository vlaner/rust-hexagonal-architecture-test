use std::pin::Pin;

use async_trait::async_trait;
use thiserror::Error;

use crate::domain::audit::AuditRepository;
use crate::domain::users::UserRepository;

#[derive(Debug, Error)]
pub enum UoWError {
    #[error("failed to begin unit of work")]
    Begin(#[source] anyhow::Error),
    #[error("failed to commit unit of work")]
    Commit(#[source] anyhow::Error),
    #[error("failed to rollback unit of work")]
    Rollback(#[source] anyhow::Error),
}

#[async_trait]
pub trait UnitOfWork: Send + Sync {
    type Tx: UnitOfWorkTransaction;

    async fn begin(&self) -> Result<Self::Tx, UoWError>;
}

#[async_trait]
pub trait UnitOfWorkTransaction: Send {
    async fn commit(self) -> Result<(), UoWError>;
    async fn rollback(self) -> Result<(), UoWError>;
}

pub trait HasUserRepo {
    fn user(&mut self) -> impl UserRepository + '_;
}

pub trait HasAuditRepo {
    fn audit(&mut self) -> impl AuditRepository + '_;
}

#[async_trait]
pub trait UnitOfWorkCallback: Send + Sync {
    type Tx: UnitOfWorkTransaction;

    async fn execute<T, F>(&self, f: F) -> Result<T, UoWError>
    where
        T: Send + 'static,
        F: for<'a> FnOnce(
                &'a mut Self::Tx,
            )
                -> Pin<Box<dyn Future<Output = Result<T, UoWError>> + Send + 'a>>
            + Send;
}
