use crate::modules::auth::api::{AuditPort, AuditPortError};
use anyhow::Context;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Executor, PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::modules::audit::core::domain::audit::{AuditLog, AuditRepository};

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
    async fn log(&mut self, user_id: Uuid, action: &str) -> Result<AuditLog, anyhow::Error> {
        log(&self.pool, user_id, action).await
    }
}

pub struct PostgresAuditRepoTx<'a> {
    tx: &'a mut Transaction<'static, Postgres>,
}

impl<'a> PostgresAuditRepoTx<'a> {
    pub fn new(tx: &'a mut Transaction<'static, Postgres>) -> Self {
        Self { tx }
    }
}

#[async_trait]
impl AuditRepository for PostgresAuditRepoTx<'_> {
    async fn log(&mut self, user_id: Uuid, action: &str) -> Result<AuditLog, anyhow::Error> {
        log(&mut **self.tx, user_id, action).await
    }
}

#[async_trait]
impl AuditPort for PostgresAuditRepoTx<'_> {
    async fn log(&mut self, user_id: Uuid, action: &str) -> Result<(), AuditPortError> {
        log(&mut **self.tx, user_id, action)
            .await
            .map(|_| ())
            .map_err(AuditPortError)
    }
}

async fn log<'e, E>(executor: E, user_id: Uuid, action: &str) -> Result<AuditLog, anyhow::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    let row = sqlx::query_as!(
        AuditLogRow,
        "INSERT INTO audit_log (user_id, action) VALUES ($1, $2) RETURNING id, user_id, action, timestamp",
        user_id,
        action
    )
    .fetch_one(executor)
    .await
    .context("insert audit log")?;

    Ok(row.into_domain())
}

#[derive(Debug)]
struct AuditLogRow {
    id: Uuid,
    user_id: Uuid,
    action: String,
    timestamp: DateTime<Utc>,
}

impl AuditLogRow {
    fn into_domain(self) -> AuditLog {
        AuditLog {
            id: self.id,
            user_id: self.user_id,
            action: self.action,
            timestamp: self.timestamp,
        }
    }
}
