use std::pin::Pin;

use async_trait::async_trait;
use thiserror::Error;

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

// 🙃 TODO: how
#[async_trait]
pub trait UnitOfWorkCallback: Send + Sync {
    type Tx: UnitOfWorkTransaction;

    async fn execute<T, F, E>(&self, f: F) -> Result<T, E>
    where
        T: Send + 'static,
        E: From<UoWError> + Send + 'static,
        F: for<'a> FnOnce(
                &'a mut Self::Tx,
            ) -> Pin<Box<dyn Future<Output = Result<T, E>> + Send + 'a>>
            + Send;
}
