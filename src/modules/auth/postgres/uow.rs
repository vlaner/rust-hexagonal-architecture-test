use crate::{
    modules::auth::{
        domain::user::user::UserRepository, postgres::user::PostgresUserRepoTx,
        uow::types::HasUserRepo,
    },
    shared::postgres::uow::PostgresUnitOfWorkTransaction,
};

impl HasUserRepo for PostgresUnitOfWorkTransaction {
    fn user(&mut self) -> impl UserRepository + '_ {
        PostgresUserRepoTx::new(&mut self.tx)
    }
}
