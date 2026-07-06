use anyhow::Error as AnyhowError;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuditLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub action: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Error, Debug)]
pub enum AuditError {
    #[error("unknown audit error")]
    Unknown(#[from] AnyhowError),
}

// TODO: possible to avoid mut?
// sql transaction requires &mut, pool does not
#[async_trait]
pub trait AuditRepository: Send + Sync {
    async fn log(
        &mut self,
        user_id: Uuid,
        action: &str,
    ) -> Result<AuditLog, AuditError>;
}
