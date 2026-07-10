use std::pin::Pin;

use anyhow::Context;
use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};

use crate::shared::uow::{UnitOfWork, UnitOfWorkCallback, UnitOfWorkTransaction, UoWError};

pub struct PostgresUnitOfWork {
    pool: PgPool,
}

impl PostgresUnitOfWork {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

pub struct PostgresUnitOfWorkTransaction {
    pub tx: Transaction<'static, Postgres>,
}

#[async_trait]
impl UnitOfWork for PostgresUnitOfWork {
    type Tx = PostgresUnitOfWorkTransaction;

    async fn begin(&self) -> Result<Self::Tx, UoWError> {
        let tx = self
            .pool
            .begin()
            .await
            .context("begin unit of work")
            .map_err(UoWError::Begin)?;
        Ok(PostgresUnitOfWorkTransaction { tx })
    }
}

#[async_trait]
impl UnitOfWorkTransaction for PostgresUnitOfWorkTransaction {
    async fn commit(self) -> Result<(), UoWError> {
        self.tx
            .commit()
            .await
            .context("commit unit of work")
            .map_err(UoWError::Commit)
    }

    async fn rollback(self) -> Result<(), UoWError> {
        self.tx
            .rollback()
            .await
            .context("rollback unit of work")
            .map_err(UoWError::Rollback)
    }
}

#[async_trait]
impl UnitOfWorkCallback for PostgresUnitOfWork {
    type Tx = PostgresUnitOfWorkTransaction;

    async fn execute<'a, T, F, E>(&'a self, f: F) -> Result<T, E>
    where
        T: Send + 'static,
        E: From<UoWError> + Send + 'static,
        F: for<'b> FnOnce(
                &'b mut Self::Tx,
            ) -> Pin<Box<dyn Future<Output = Result<T, E>> + Send + 'b>>
            + Send,
    {
        let tx = self
            .pool
            .begin()
            .await
            .context("begin unit of work")
            .map_err(UoWError::Begin)?;

        let mut tx_uow = PostgresUnitOfWorkTransaction { tx };
        let result = f(&mut tx_uow).await;

        match result {
            Ok(value) => {
                tx_uow
                    .commit()
                    .await
                    .context("commit unit of work")
                    .map_err(UoWError::Commit)?;

                Ok(value)
            }

            Err(err) => {
                if let Err(rollback_err) = tx_uow.rollback().await {
                    tracing::error!(error = ?rollback_err, "rollback failed");
                }

                Err(err)
            }
        }
    }
}
