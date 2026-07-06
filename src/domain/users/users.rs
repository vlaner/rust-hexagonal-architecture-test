use anyhow::Error as AnyhowError;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Error, Debug)]
pub enum UserError {
    #[error("user with ID {0} not found")]
    NotFound(Uuid),
    #[error("username '{0}' already exists")]
    DuplicateUsername(String),
    #[error("unknown user error")]
    Unknown(#[from] AnyhowError),
}

// TODO: possible to avoid mut?
// sql transaction requires &mut, pool does not
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn by_id(&mut self, id: Uuid) -> Result<User, UserError>;
}
