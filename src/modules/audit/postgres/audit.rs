use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Executor, PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::contracts::audit::{
    audit::{AuditLog, AuditRepository},
    error::AuditError,
};

pub struct PostgresAuditRepositoryPool {
    pub pool: PgPool,
}

impl PostgresAuditRepositoryPool {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuditRepository for PostgresAuditRepositoryPool {
    async fn log(&mut self, user_id: Uuid, action: &str) -> Result<AuditLog, AuditError> {
        log(&self.pool, user_id, action).await
    }
}

pub struct PostgresAuditRepoTx<'a, 'c> {
    pub tx: &'a mut Transaction<'c, Postgres>,
}

impl<'a, 'c> PostgresAuditRepoTx<'a, 'c> {
    pub fn new(tx: &'a mut Transaction<'c, Postgres>) -> Self {
        Self { tx }
    }
}

#[async_trait]
impl<'a, 'c> AuditRepository for PostgresAuditRepoTx<'a, 'c> {
    async fn log(&mut self, user_id: Uuid, action: &str) -> Result<AuditLog, AuditError> {
        log(&mut **self.tx, user_id, action).await
    }
}

// TODO: possible not to have `log` helper function with `executor: E`?
pub async fn log<'e, E>(executor: E, user_id: Uuid, action: &str) -> Result<AuditLog, AuditError>
where
    E: Executor<'e, Database = Postgres>,
{
    let result = sqlx::query_as!(
        AuditLogRow,
        "INSERT INTO audit_log (user_id, action) VALUES ($1, $2) RETURNING id, user_id, action, timestamp",
        user_id,
        action
    )
    .fetch_one(executor)
    .await;

    match result {
        Ok(row) => Ok(row.to_domain()),
        Err(e) => Err(AuditError::Unknown(
            anyhow::Error::from(e).context("insert audit log"),
        )),
    }
}

#[derive(Debug)]
pub struct AuditLogRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub action: String,
    pub timestamp: DateTime<Utc>,
}

impl AuditLogRow {
    pub fn to_domain(self) -> AuditLog {
        AuditLog {
            id: self.id,
            user_id: self.user_id,
            action: self.action,
            timestamp: self.timestamp,
        }
    }
}
