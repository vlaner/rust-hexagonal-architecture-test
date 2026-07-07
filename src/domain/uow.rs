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

    async fn begin(&self) -> Result<Box<Self::Tx>, UoWError>;
}

#[async_trait]
pub trait UnitOfWorkTransaction: Send {
    async fn commit(self: Box<Self>) -> Result<(), UoWError>;
    async fn rollback(self: Box<Self>) -> Result<(), UoWError>;
}

pub trait HasUserRepo {
    fn user(&mut self) -> Box<dyn UserRepository + '_>;
}

pub trait HasAuditRepo {
    fn audit(&mut self) -> Box<dyn AuditRepository + '_>;
}
