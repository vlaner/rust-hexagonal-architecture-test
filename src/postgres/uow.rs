use anyhow::Context;
use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};

use crate::domain::audit::AuditRepository;
use crate::domain::uow::{UnitOfWork, UnitOfWorkTransaction, UoWError};
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
    async fn begin(&self) -> Result<Box<dyn UnitOfWorkTransaction>, UoWError> {
        let tx = self
            .pool
            .begin()
            .await
            .context("begin unit of work")
            .map_err(UoWError::Begin)?;
        Ok(Box::new(PostgresUnitOfWorkTransaction { tx }))
    }
}

pub struct PostgresUnitOfWorkTransaction {
    tx: Transaction<'static, Postgres>,
}

#[async_trait]
impl UnitOfWorkTransaction for PostgresUnitOfWorkTransaction {
    fn user(&mut self) -> Box<dyn UserRepository + '_> {
        Box::new(PostgresUserRepoTx::new(&mut self.tx))
    }

    fn audit(&mut self) -> Box<dyn AuditRepository + '_> {
        Box::new(PostgresAuditRepoTx::new(&mut self.tx))
    }

    async fn commit(self: Box<Self>) -> Result<(), UoWError> {
        self.tx
            .commit()
            .await
            .context("commit unit of work")
            .map_err(UoWError::Commit)?;
        Ok(())
    }

    async fn rollback(self: Box<Self>) -> Result<(), UoWError> {
        self.tx
            .rollback()
            .await
            .context("rollback unit of work")
            .map_err(UoWError::Rollback)?;
        Ok(())
    }
}
