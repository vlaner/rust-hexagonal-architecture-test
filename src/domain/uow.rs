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
    async fn begin(&self) -> Result<Box<dyn UnitOfWorkTransaction>, UoWError>;
}

// TODO: non hard coded user and audit?
#[async_trait]
pub trait UnitOfWorkTransaction: Send {
    fn user(&mut self) -> Box<dyn UserRepository + '_>;
    fn audit(&mut self) -> Box<dyn AuditRepository + '_>;
    async fn commit(self: Box<Self>) -> Result<(), UoWError>;
    async fn rollback(self: Box<Self>) -> Result<(), UoWError>;
}
