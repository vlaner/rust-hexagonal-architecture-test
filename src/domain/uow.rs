use crate::domain::{audit::AuditRepository, users::UserRepository};
use anyhow::Error as AnyhowError;
use async_trait::async_trait;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UoWError {
    #[error("unknown unit of work error")]
    Unknown(#[from] AnyhowError),
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
