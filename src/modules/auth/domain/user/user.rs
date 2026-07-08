use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::modules::auth::domain::user::error::UserError;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
}

// TODO: possible to avoid mut?
// sql transaction requires &mut, pool does not
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn by_id(&mut self, id: Uuid) -> Result<User, UserError>;
}
