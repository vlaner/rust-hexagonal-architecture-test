use std::pin::Pin;

use anyhow::Context;
use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};

use crate::domain::audit::AuditRepository;
use crate::domain::uow::{HasAuditRepo, HasUserRepo};
use crate::domain::uow::{UnitOfWork, UnitOfWorkCallback, UnitOfWorkTransaction, UoWError};
use crate::domain::users::UserRepository;
use crate::postgres::audit::PostgresAuditRepoTx;
use crate::postgres::users::PostgresUserRepoTx;

pub struct PostgresUnitOfWork {
    pool: PgPool,
}

impl PostgresUnitOfWork {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
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

pub struct PostgresUnitOfWorkTransaction {
    tx: Transaction<'static, Postgres>,
}

impl HasUserRepo for PostgresUnitOfWorkTransaction {
    fn user(&mut self) -> impl UserRepository + '_ {
        PostgresUserRepoTx::new(&mut self.tx)
    }
}

impl HasAuditRepo for PostgresUnitOfWorkTransaction {
    fn audit(&mut self) -> impl AuditRepository + '_ {
        PostgresAuditRepoTx::new(&mut self.tx)
    }
}

#[async_trait]
impl UnitOfWorkCallback for PostgresUnitOfWork {
    type Tx = PostgresUnitOfWorkTransaction;

    async fn execute<'a, T, F>(&'a self, f: F) -> Result<T, UoWError>
    where
        T: Send + 'static,
        F: for<'b> FnOnce(
                &'b mut Self::Tx,
            )
                -> Pin<Box<dyn Future<Output = Result<T, UoWError>> + Send + 'b>>
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
                tx_uow
                    .rollback()
                    .await
                    .context("rollback unit of work")
                    .map_err(UoWError::Rollback)?;

                Err(err)
            }
        }
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
