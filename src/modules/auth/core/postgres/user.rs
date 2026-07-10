use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Executor, Postgres, Transaction};
use uuid::Uuid;

use crate::modules::auth::core::domain::user::{User, UserRepository, error::UserError};

pub struct PostgresUserRepoTx<'a, 'c> {
    pub tx: &'a mut Transaction<'c, Postgres>,
}

impl<'a, 'c> PostgresUserRepoTx<'a, 'c> {
    pub fn new(tx: &'a mut Transaction<'c, Postgres>) -> Self {
        Self { tx }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepoTx<'_, '_> {
    async fn by_id(&mut self, id: Uuid) -> Result<User, UserError> {
        by_id(&mut **self.tx, id).await
    }
}

// TODO: possible not to have `by_id` helper function with `executor: E`?
pub async fn by_id<'e, E>(executor: E, id: Uuid) -> Result<User, UserError>
where
    E: Executor<'e, Database = Postgres>,
{
    let row = sqlx::query_as!(
        UserRow,
        "SELECT id, username, password, created_at FROM users WHERE id = $1",
        id
    )
    .fetch_one(executor)
    .await;

    match row {
        Ok(r) => Ok(r.into_domain()),
        Err(sqlx::Error::RowNotFound) => Err(UserError::NotFound),
        Err(e) => Err(UserError::Unknown(
            anyhow::Error::from(e).context("fetch user by id"),
        )),
    }
}

#[derive(Debug)]
pub struct UserRow {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
}

impl UserRow {
    pub fn into_domain(self) -> User {
        User {
            id: self.id,
            username: self.username,
            password: self.password,
            created_at: self.created_at,
        }
    }
}
