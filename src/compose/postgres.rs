use crate::{
    modules::{
        audit::core::postgres::audit::PostgresAuditRepoTx,
        auth::{
            api::{AuditPort, HasAuditPort},
            core::{HasUserRepo, UserRepository, postgres::user::PostgresUserRepoTx},
        },
    },
    shared::postgres::uow::PostgresUnitOfWorkTransaction,
};

impl HasUserRepo for PostgresUnitOfWorkTransaction {
    fn user(&mut self) -> impl UserRepository + '_ {
        PostgresUserRepoTx::new(&mut self.tx)
    }
}

impl HasAuditPort for PostgresUnitOfWorkTransaction {
    fn audit(&mut self) -> impl AuditPort + '_ {
        PostgresAuditRepoTx::new(&mut self.tx)
    }
}
