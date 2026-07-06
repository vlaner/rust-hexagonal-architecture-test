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
    #[error("User with ID {0} not found")]
    NotFound(Uuid),
    #[error("Username '{0}' already exists")]
    DuplicateUsername(String),
    #[error("Unknown internal error: {0}")]
    Unknown(String),
}

// TODO: possible to avoid mut?
// sql transaction requires &mut, pool does not
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn by_id(&mut self, id: Uuid) -> Result<User, UserError>;
}
