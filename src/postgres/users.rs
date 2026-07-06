use crate::domain::users::{User, UserError, UserRepository};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Executor, PgPool, Postgres, Transaction};
use uuid::Uuid;

pub struct PostgresUserRepoPool {
    pub pool: PgPool,
}

impl PostgresUserRepoPool {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepoPool {
    async fn by_id(&mut self, id: Uuid) -> Result<User, UserError> {
        by_id(&self.pool, id).await
    }
}

pub struct PostgresUserRepoTx<'a> {
    pub tx: &'a mut Transaction<'static, Postgres>,
}

impl<'a> PostgresUserRepoTx<'a> {
    pub fn new(tx: &'a mut Transaction<'static, Postgres>) -> Self {
        Self { tx }
    }
}

#[async_trait]
impl<'a> UserRepository for PostgresUserRepoTx<'a> {
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
        Ok(r) => Ok(r.to_domain()),
        Err(sqlx::Error::RowNotFound) => Err(UserError::NotFound(id)),
        Err(e) => Err(UserError::Unknown(e.into())),
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
    pub fn to_domain(self) -> User {
        User {
            id: self.id,
            username: self.username,
            password: self.password,
            created_at: self.created_at,
        }
    }
}
