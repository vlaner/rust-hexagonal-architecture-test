use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Executor, PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::modules::audit::core::domain::audit::{AuditError, AuditLog, AuditRepository};

pub struct PostgresAuditRepositoryPool {
    pool: PgPool,
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
impl AuditRepository for PostgresAuditRepoTx<'_, '_> {
    async fn log(&mut self, user_id: Uuid, action: &str) -> Result<AuditLog, AuditError> {
        log(&mut **self.tx, user_id, action).await
    }
}

async fn log<'e, E>(executor: E, user_id: Uuid, action: &str) -> Result<AuditLog, AuditError>
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
        Err(error) => Err(AuditError::Unknown(
            anyhow::Error::from(error).context("insert audit log"),
        )),
    }
}

#[derive(Debug)]
struct AuditLogRow {
    id: Uuid,
    user_id: Uuid,
    action: String,
    timestamp: DateTime<Utc>,
}

impl AuditLogRow {
    fn to_domain(self) -> AuditLog {
        AuditLog {
            id: self.id,
            user_id: self.user_id,
            action: self.action,
            timestamp: self.timestamp,
        }
    }
}
